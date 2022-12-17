use crate::common;

use itertools::Itertools;
use std::collections::HashSet;

use reqwest::Error;

fn parse_rucksacks(input: &str) -> impl Iterator<Item = &str> {
    input.trim().split("\n")
}

fn parse_rucksacks_compartments(input: &str) -> impl Iterator<Item = (&str, &str)> {
    parse_rucksacks(input).map(|rucksack| rucksack.split_at(rucksack.chars().count() / 2))
}

fn total_priority<'a>(rucksacks: impl Iterator<Item = (&'a str, &'a str)>) -> u32 {
    rucksacks.fold(0, |acc, (fst, snd)| {
        let fst_items = fst.chars().collect::<HashSet<char>>();
        let snd_items = snd.chars().collect::<HashSet<char>>();
        acc + item_priority(fst_items.intersection(&snd_items).take(1).next().unwrap())
    })
}

fn item_priority(c: &char) -> u32 {
    match c {
        'A'..='Z' => (((*c as u32) + 32) - 96) + 26,
        'a'..='z' => (*c as u32) - 96,
        _ => panic!("unexpected item"),
    }
}

fn total_priority_for_elf_group_badges<'a>(rucksacks: impl Iterator<Item = &'a str>) -> u32 {
    rucksacks
        .tuples::<(_, _, _)>()
        .fold(0, |acc, (fst, snd, thd)| {
            let fst_items = fst.chars().collect::<HashSet<char>>();
            let snd_items = snd.chars().collect::<HashSet<char>>();
            let thd_items = thd.chars().collect::<HashSet<char>>();
            acc + item_priority(
                fst_items
                    .intersection(&snd_items)
                    .cloned()
                    .collect::<HashSet<char>>()
                    .intersection(&thd_items)
                    .take(1)
                    .next()
                    .unwrap(),
            )
        })
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 3).await?;
    println!(
        "Day 03 Part 1: {:?}",
        total_priority(parse_rucksacks_compartments(&input))
    );
    println!(
        "Day 03 Part 2: {:?}",
        total_priority_for_elf_group_badges(parse_rucksacks(&input))
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_parse_rucksacks_compartments() {
        assert_eq!(
            parse_rucksacks_compartments(TEST_INPUT).collect::<Vec<(&str, &str)>>(),
            vec![
                ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
                ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
                ("PmmdzqPrV", "vPwwTWBwg"),
                ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
                ("ttgJtRGJ", "QctTZtZT"),
                ("CrZsJsPPZsGz", "wwsLwLmpwMDw")
            ]
        )
    }

    #[test]
    fn test_total_priority() {
        assert_eq!(
            total_priority(parse_rucksacks_compartments(TEST_INPUT)),
            157
        )
    }

    #[test]
    fn test_total_priority_for_elf_group_badges() {
        assert_eq!(
            total_priority_for_elf_group_badges(parse_rucksacks(TEST_INPUT)),
            70
        )
    }
}
