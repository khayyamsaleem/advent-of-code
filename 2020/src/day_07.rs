use crate::common;
use petgraph::prelude::{Graph, NodeIndex};
use reqwest::Error;

const BAG_TO_FIND: &str = "shiny gold";

fn add_or_get_node(g: &mut Graph<String, u64>, node_name: String) -> NodeIndex {
    let node_if_present = g.node_indices().find(|n| g[*n] == node_name);
    match node_if_present {
        Some(node) => node,
        None => g.add_node(node_name),
    }
}

fn create_trees_from_input(input: String) -> Graph<String, u64> {
    input.trim().split('\n').fold(Graph::new(), |mut acc, cur| {
        let mut iter = cur.split(" ");
        let bag_name = iter.next().unwrap().to_owned() + " " + iter.next().unwrap(); //shade, color
        let parent = add_or_get_node(&mut acc, bag_name);

        //skipping "bags contain"
        itertools::Itertools::intersperse(iter.skip(2), " ")
            .collect::<String>()
            .split(", ")
            .for_each(|bag_exp| {
                let mut bag_exp_iter = bag_exp.split(" ");
                let num_bags = bag_exp_iter
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap_or_default(); // weight, zero if "no"
                if num_bags == 0 {
                    return;
                };
                let nested_bag_name =
                    bag_exp_iter.next().unwrap().to_owned() + " " + bag_exp_iter.next().unwrap(); // shade, color
                let child = add_or_get_node(&mut acc, nested_bag_name);
                acc.add_edge(parent, child, num_bags);
            });
        acc
    })
}

fn has_shiny_gold_bags(g: &Graph<String, u64>, n: &NodeIndex) -> bool {
    let node = &g[*n];
    let mut children = g.neighbors(*n);
    return node == BAG_TO_FIND || children.any(|c| has_shiny_gold_bags(g, &c));
}

fn count_bag_capacity(bag_name: &str, g: &Graph<String, u64>, init_weight: u64) -> u64 {
    let node_to_count_for = g
        .node_indices()
        .find(|n| g[*n] == bag_name)
        .unwrap_or_else(|| panic!("No such node: {}", bag_name));
    let neighbors = g.neighbors(node_to_count_for);
    let sum: u64 = neighbors
        .map(|n| {
            let edge = g.find_edge(node_to_count_for, n).unwrap();
            let weight = g[edge];
            count_bag_capacity(&g[n], g, weight)
        })
        .sum();
    init_weight + (sum * init_weight)
}

pub async fn solve() -> Result<(), Error> {
    let g = create_trees_from_input(common::get_input(2020, 7).await?);
    println!(
        "Day 07 Part 1: {:?}",
        g.node_indices()
            .filter(|n| has_shiny_gold_bags(&g, n) && g[*n] != BAG_TO_FIND)
            .count()
    );
    println!(
        "Day 07 Part 2: {:?}",
        count_bag_capacity(BAG_TO_FIND, &g, 1) - 1
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;
    use mockito;
    use reqwest;
    use tokio;

    #[tokio::test]
    async fn test_create_tree() -> Result<(), reqwest::Error> {
        let _m = mockito::mock("GET", "/2020/day/7/input")
            .with_body(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
",
            )
            .create();
        let g = create_trees_from_input(common::get_input(2020, 7).await?);
        assert_eq!(g.node_count(), 9);
        assert_eq!(count_bag_capacity(BAG_TO_FIND, &g, 1) - 1, 32);
        Ok(())
    }

    #[test]
    fn test_count_bag_capacity() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let g = create_trees_from_input(input.to_string());
        assert_eq!(count_bag_capacity(BAG_TO_FIND, &g, 1) - 1, 126);
    }
}
