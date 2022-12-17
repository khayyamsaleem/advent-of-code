use crate::common;

use itertools::Itertools;
use recap::Recap;
use reqwest::Error;
use serde::Deserialize;

type Stack = Vec<char>;

#[derive(Debug, Recap, Deserialize, PartialEq, Clone)]
#[recap(regex = r#"move (?P<n_crates>\d+) from (?P<source_stack>\d+) to (?P<target_stack>\d+)"#)]
struct Operation {
    n_crates: usize,
    source_stack: usize,
    target_stack: usize,
}

const CRATE_WIDTH: usize = 3;
const CRATE_GAP: usize = 1;

fn get_stacks_and_operations(input: &str) -> (Vec<Stack>, impl Iterator<Item = Operation> + '_) {
    let (raw_stacks, raw_ops) = input.trim_end().split("\n\n").next_tuple().unwrap();
    let mut raw_stacks_iter = raw_stacks.rsplit("\n");
    let raw_lst_row = raw_stacks_iter.next().unwrap();
    let num_stacks: usize = raw_lst_row
        .trim()
        .rsplit(&" ".repeat(CRATE_WIDTH))
        .next()
        .unwrap()
        .parse()
        .unwrap();
    (
        raw_stacks_iter.fold::<Vec<Stack>, _>(vec![vec![]; num_stacks], |mut acc, raw_crates| {
            raw_crates
                .chars()
                .chunks(CRATE_WIDTH + CRATE_GAP)
                .into_iter()
                .enumerate()
                .for_each(|(i, mut c)| {
                    if c.next().unwrap() == ' ' {
                        return;
                    }; // check if its whitespace
                    acc[i].push(c.next().unwrap()) // if it wasn't a space, we skipped the left square bracket
                });
            acc
        }),
        raw_ops.split("\n").map(|raw_op| raw_op.parse().unwrap()),
    )
}

fn rearrange_crates(stacks: &mut Vec<Stack>, operations: impl Iterator<Item = Operation>) {
    operations.for_each(|op| {
        (0..op.n_crates).for_each(|_| {
            let crate_to_move = stacks[op.source_stack - 1].pop().unwrap();
            stacks[op.target_stack - 1].push(crate_to_move)
        })
    })
}

fn rearrange_crates_cratemover_9001(
    stacks: &mut Vec<Stack>,
    operations: impl Iterator<Item = Operation>,
) {
    operations.for_each(|op| {
        let mut short_stacc: Stack = vec![];
        (0..op.n_crates).for_each(|_| {
            let crate_to_move = stacks[op.source_stack - 1].pop().unwrap();
            short_stacc.push(crate_to_move)
        });
        while !short_stacc.is_empty() {
            stacks[op.target_stack - 1].push(short_stacc.pop().unwrap())
        }
    })
}

fn top_crates(stacks: &Vec<Stack>) -> String {
    stacks.iter().fold(String::from(""), |mut acc, s| {
        acc.push(*s.last().unwrap());
        acc
    })
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 5).await?;
    let (mut stacks, operations) = get_stacks_and_operations(&input);
    rearrange_crates(&mut stacks, operations);
    println!("Day 05 Part 1: {}", top_crates(&stacks));

    let (mut stacks, operations) = get_stacks_and_operations(&input);
    rearrange_crates_cratemover_9001(&mut stacks, operations);
    println!("Day 05 Part 2: {}", top_crates(&stacks));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_get_stacks_and_operations() {
        let (stacks, operations) = get_stacks_and_operations(TEST_INPUT);
        assert_eq!(stacks, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(
            operations.collect::<Vec<Operation>>(),
            vec![
                Operation {
                    n_crates: 1,
                    source_stack: 2,
                    target_stack: 1
                },
                Operation {
                    n_crates: 3,
                    source_stack: 1,
                    target_stack: 3
                },
                Operation {
                    n_crates: 2,
                    source_stack: 2,
                    target_stack: 1
                },
                Operation {
                    n_crates: 1,
                    source_stack: 1,
                    target_stack: 2
                },
            ]
        );
    }

    #[test]
    fn test_rearrange_crates() {
        let (mut stacks, operations) = get_stacks_and_operations(TEST_INPUT);
        rearrange_crates(&mut stacks, operations);
        assert_eq!(stacks, vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']])
    }

    #[test]
    fn test_rearrange_crates_cratemover_9001() {
        let (mut stacks, operations) = get_stacks_and_operations(TEST_INPUT);
        rearrange_crates_cratemover_9001(&mut stacks, operations);
        assert_eq!(stacks, vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']])
    }

    #[test]
    fn test_top_crates() {
        let (mut stacks, operations) = get_stacks_and_operations(TEST_INPUT);
        rearrange_crates(&mut stacks, operations);
        assert_eq!(top_crates(&stacks), "CMZ");
    }
}
