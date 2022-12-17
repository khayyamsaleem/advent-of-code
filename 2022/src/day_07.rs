use std::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::common;

use reqwest::Error;

const DISK_SPACE: usize = 70000000;
const UPGRADE_SIZE: usize = 30000000;

type Tree = Rc<RefCell<Node>>;

#[derive(PartialEq)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Tree>,
}

impl Node {
    fn new(name: &str, size: usize) -> Tree {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            size,
            children: vec![],
        }))
    }

    fn add_child(&mut self, child: Tree) {
        self.children.push(child);
    }

    fn fmt_indented(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let indent_str = "    ".repeat(indent);
        writeln!(f, "{}({},{})", indent_str, self.name, self.size)?;
        for child in &self.children {
            child.borrow().fmt_indented(f, indent + 1)?;
        }
        Ok(())
    }

    #[cfg(test)]
    fn builder() -> NodeBuilder {
        NodeBuilder { node: None }
    }

    fn get_size(&self) -> usize {
        self.children
            .iter()
            .fold(self.size as usize, |acc, n| acc + n.borrow().get_size())
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        self.fmt_indented(f, 0)
    }
}

#[cfg(test)]
struct NodeBuilder {
    node: Option<Tree>,
}

#[cfg(test)]
impl NodeBuilder {
    fn name(mut self, name: &str) -> Self {
        if let Some(node) = &self.node {
            node.borrow_mut().name = name.to_string();
        } else {
            let node = Node::new(name, 0);
            self.node = Some(node);
        }
        self
    }

    fn size(mut self, size: usize) -> Self {
        if let Some(node) = &self.node {
            node.borrow_mut().size = size;
        } else {
            let node = Node::new("", size);
            self.node = Some(node);
        }
        self
    }

    fn children(mut self, children: Vec<Tree>) -> Self {
        if let Some(node) = &self.node {
            for child in children {
                node.borrow_mut().add_child(child);
            }
        } else {
            let node = Node::new("", 0);
            for child in children {
                node.borrow_mut().add_child(child);
            }
            self.node = Some(node);
        }
        self
    }

    fn build(self) -> Tree {
        self.node.unwrap()
    }
}

fn build_file_tree(lines: Vec<&str>) -> Tree {
    let mut line_iterator = 0;
    let root = Node::new("/", 0);
    let mut cur = Rc::clone(&root);
    let mut path: Vec<Tree> = vec![];
    while line_iterator < lines.len() {
        let line = lines[line_iterator];
        let tokens = line.split(" ").collect::<Vec<&str>>();
        line_iterator += 1;
        match tokens[..] {
            ["$", "ls"] => {
                while line_iterator < lines.len() && !lines[line_iterator].starts_with("$") {
                    match lines[line_iterator].split(" ").collect::<Vec<&str>>()[..] {
                        ["dir", dirname] => cur.borrow_mut().add_child(Node::new(&dirname, 0)),
                        [size, filename] => cur
                            .borrow_mut()
                            .add_child(Node::new(&filename, size.parse::<usize>().unwrap())),
                        _ => panic!("unexpected output for ls"),
                    }
                    line_iterator += 1;
                }
            }
            ["$", "cd", ".."] => {
                cur = Rc::clone(&path.pop().unwrap());
            }
            ["$", "cd", dir] => {
                path.push(Rc::clone(&cur));
                cur = Rc::clone(
                    cur.clone()
                        .borrow()
                        .children
                        .iter()
                        .find(|n| n.borrow().name == dir)
                        .unwrap(),
                );
            }
            _ => panic!("unsupported line"),
        };
    }
    root
}

fn total_size_of_dirs_lte_x(tree: Tree, x: usize) -> usize {
    let node = tree.borrow();
    let s = node.get_size();
    (if node.size == 0 && s <= x { s } else { 0 })
        + node
            .children
            .iter()
            .map(|n| total_size_of_dirs_lte_x(Rc::clone(n), x))
            .sum::<usize>()
}

fn find_size_of_smallest_dir_to_delete_for_upgrade(
    tree: Tree,
    disk_space: usize,
    upgrade_size: usize,
) -> usize {
    let unused_space = disk_space - tree.borrow().get_size();
    let space_to_free = upgrade_size - unused_space;

    // saves directory sizes to a vec
    fn h(t: Tree, acc: &mut Vec<usize>) {
        let node = t.borrow();
        if node.size == 0 {
            // if node is a directory
            acc.push(node.get_size());
        }
        node.children.iter().for_each(|n| {
            h(Rc::clone(&n), acc);
        });
    }

    let mut sizes = vec![];
    h(tree, &mut sizes);
    *sizes.iter().filter(|s| **s >= space_to_free).min().unwrap()
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 7).await?;
    let lines = input.trim().split("\n").skip(1).collect();
    let t = build_file_tree(lines);
    println!(
        "Day 07 Part 1: {}",
        total_size_of_dirs_lte_x(Rc::clone(&t), 100000)
    );
    println!(
        "Day 07 Part 2: {}",
        find_size_of_smallest_dir_to_delete_for_upgrade(Rc::clone(&t), DISK_SPACE, UPGRADE_SIZE)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_build_file_tree() {
        let lines = TEST_INPUT.trim().split("\n").skip(1).collect();
        let t = build_file_tree(lines);
        assert_eq!(
            t,
            Node::builder()
                .name("/")
                .size(0)
                .children(vec![
                    Node::builder()
                        .name("a")
                        .size(0)
                        .children(vec![
                            Node::builder()
                                .name("e")
                                .size(0)
                                .children(vec![Node::new("i", 584),])
                                .build(),
                            Node::new("f", 29116),
                            Node::new("g", 2557),
                            Node::new("h.lst", 62596),
                        ])
                        .build(),
                    Node::new("b.txt", 14848514),
                    Node::new("c.dat", 8504156),
                    Node::builder()
                        .name("d")
                        .size(0)
                        .children(vec![
                            Node::new("j", 4060174),
                            Node::new("d.log", 8033020),
                            Node::new("d.ext", 5626152),
                            Node::new("k", 7214296),
                        ])
                        .build()
                ])
                .build()
        )
    }

    #[test]
    fn test_get_size() {
        let lines = TEST_INPUT.trim().split("\n").skip(1).collect();
        assert_eq!(build_file_tree(lines).borrow().get_size(), 48381165)
    }

    #[test]
    fn test_total_size_of_dirs_lt_x() {
        let lines = TEST_INPUT.trim().split("\n").skip(1).collect();
        let t = build_file_tree(lines);
        assert_eq!(total_size_of_dirs_lte_x(t, 100000), 95437)
    }

    #[test]
    fn test_find_size_of_smallest_dir_to_delete_for_upgrade() {
        let lines = TEST_INPUT.trim().split("\n").skip(1).collect();
        let t = build_file_tree(lines);
        assert_eq!(
            find_size_of_smallest_dir_to_delete_for_upgrade(t, DISK_SPACE, UPGRADE_SIZE),
            24933642
        )
    }
}
