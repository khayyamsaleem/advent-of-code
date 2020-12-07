use crate::common;
use reqwest::Error;
use std::collections::HashSet;
use std::iter::FromIterator;

fn parse_groups_from_input(input: &str) -> Vec<Vec<&str>> {
    input
        .trim()
        .split("\n\n")
        .map(|group_as_str| group_as_str.split('\n').collect())
        .collect()
}

fn count_sum_for_questions_per_group(groups: &Vec<Vec<&str>>) -> i64 {
    groups
        .iter()
        .map(|group| -> i64 { HashSet::<char>::from_iter(group.join("").chars()).len() as i64 })
        .sum::<i64>()
}

fn count_sum_for_all_answered_per_group(groups: &Vec<Vec<&str>>) -> i64 {
    groups
        .iter()
        .map(|group| {
            let people: Vec<HashSet<char>> = group
                .iter()
                .map(|q| HashSet::from_iter(q.chars()))
                .collect();
            people
                .first()
                .unwrap()
                .iter()
                .filter(|q| people.iter().all(|questions| questions.contains(q)))
                .count() as i64
        })
        .sum()
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2020, 6).await?;
    let groups = parse_groups_from_input(&input);
    println!(
        "Day 06 Part 1: {:?}",
        count_sum_for_questions_per_group(&groups)
    );
    println!(
        "Day 06 Part 2: {:?}",
        count_sum_for_all_answered_per_group(&groups)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;
    use mockito::{self, mock};
    use reqwest::Error;
    use tokio;

    #[tokio::test]
    async fn test_parse_groups_from_input() -> Result<(), Error> {
        let _m = mock("GET", "/2020/day/6/input")
            .with_body(
                "abc

a
b
c

ab
ac

a
a
a
a

b
",
            )
            .create();
        assert_eq!(
            parse_groups_from_input(&common::get_input(2020, 6).await?),
            vec!(
                vec!("abc"),
                vec!("a", "b", "c"),
                vec!("ab", "ac"),
                vec!("a", "a", "a", "a"),
                vec!("b")
            )
        );
        Ok(())
    }

    #[test]
    fn test_count_sum_for_groups() {
        assert_eq!(
            count_sum_for_questions_per_group(&vec!(
                vec!("abc"),
                vec!("a", "b", "c"),
                vec!("ab", "ac"),
                vec!("a", "a", "a", "a"),
                vec!("b")
            )),
            11
        );
    }

    #[test]
    fn test_count_all_answered_per_group() {
        assert_eq!(
            count_sum_for_all_answered_per_group(&vec!(
                vec!("abc"),
                vec!("a", "b", "c"),
                vec!("ab", "ac"),
                vec!("a", "a", "a", "a"),
                vec!("b")
            )),
            6
        );
    }
}
