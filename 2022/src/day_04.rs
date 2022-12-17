use crate::common;

use recap::Recap;
use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap, PartialEq)]
#[recap(regex = r#"(?P<first_lo>\d+)-(?P<first_hi>\d+),(?P<second_lo>\d+)-(?P<second_hi>\d+)"#)]
struct AssignmentPair {
    first_lo: i64,
    first_hi: i64,
    second_lo: i64,
    second_hi: i64,
}

fn get_assignment_pairs(input: &str) -> impl Iterator<Item = AssignmentPair> + '_ {
    input.trim().split("\n").map(|ap| ap.parse().unwrap())
}

fn count_redundant_assignments(assignments: impl Iterator<Item = AssignmentPair>) -> usize {
    assignments
        .filter(
            |ap| match (ap.first_hi - ap.first_lo, ap.second_hi - ap.second_lo) {
                (d1, d2)
                    if (d1 >= d2 && ap.first_hi >= ap.second_hi && ap.first_lo <= ap.second_lo)
                        || (d2 >= d1
                            && ap.second_hi >= ap.first_hi
                            && ap.second_lo <= ap.first_lo) =>
                {
                    true
                }
                _ => false,
            },
        )
        .count()
}

fn count_overlapping_assignments(assignments: impl Iterator<Item = AssignmentPair>) -> usize {
    assignments
        .filter(|ap| {
            ap.first_hi >= ap.second_lo && ap.first_hi <= ap.second_hi
                || ap.second_hi >= ap.first_lo && ap.second_hi <= ap.first_hi
        })
        .count()
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 4).await?;
    println!(
        "Day 04 Part 1: {:?}",
        count_redundant_assignments(get_assignment_pairs(&input))
    );

    println!(
        "Day 04 Part 2: {:?}",
        count_overlapping_assignments(get_assignment_pairs(&input))
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ASSIGNMENTS: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_deserialize_assignment_pair() {
        let input = "1-2,3-4";
        let ap: AssignmentPair = input.parse().unwrap();
        let test_ap = AssignmentPair {
            first_lo: 1,
            first_hi: 2,
            second_lo: 3,
            second_hi: 4,
        };
        assert_eq!(ap, test_ap);
    }

    #[test]
    fn test_count_redundant_assignments() {
        assert_eq!(
            count_redundant_assignments(get_assignment_pairs(TEST_ASSIGNMENTS)),
            2
        )
    }

    #[test]
    fn test_count_overlapping_assignments() {
        assert_eq!(
            count_overlapping_assignments(get_assignment_pairs(TEST_ASSIGNMENTS)),
            4
        )
    }
}
