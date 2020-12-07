use reqwest::Error;
use std::collections::HashSet;

use crate::common;

#[cfg(test)]
use mockito;

const TARGET_SUM: i64 = 2020;

async fn get_vector_of_ints_from_input(year: i64, day: i64) -> Result<Vec<i64>, Error> {
    Ok(common::get_input(year, day)
        .await?
        .trim()
        .split("\n")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>())
}

fn find_pair_for_sum(nums: Vec<i64>, sum: i64) -> Vec<i64> {
    let mut m = HashSet::new();
    for &n in nums.iter() {
        match m.get(&(sum - n)) {
            Some(found) => return vec![n, *found],
            None => {
                m.insert(n);
            }
        }
    }
    Vec::new()
}

fn mult_vec(t: Vec<i64>) -> i64 {
    t.iter().fold(1, |x, y| x * y)
}

fn find_triple_for_sum(nums: Vec<i64>, sum: i64) -> Vec<i64> {
    for (i, n) in nums.iter().enumerate() {
        let cur_sum = sum - n;
        let maybe = find_pair_for_sum(nums[i..].to_vec(), cur_sum);
        if maybe.len() > 0 {
            return [&maybe[..], &[*n][..]].concat();
        }
    }
    Vec::new()
}

pub async fn solve() -> Result<(), Error> {
    let result = get_vector_of_ints_from_input(2020, 1).await?;
    let another_result = result.clone();
    println!(
        "Day 01 Part 1: {:?}",
        mult_vec(find_pair_for_sum(result, TARGET_SUM))
    );
    println!(
        "Day 01 Part 2: {:?}",
        mult_vec(find_triple_for_sum(another_result, TARGET_SUM))
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[tokio::test]
    async fn test_get_input() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/1/input")
            .with_status(200)
            .with_body(
                "1
2
3
",
            )
            .create();
        assert_eq!(get_vector_of_ints_from_input(2020, 1).await?, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_find_pair() {
        assert_eq!(
            find_pair_for_sum(vec![1, 2, 17, 2, 5, -5, 2, 4], 12),
            vec![-5, 17]
        );
    }

    #[test]
    fn test_find_pair_empty_when_no_sum() {
        assert_eq!(
            find_pair_for_sum(vec![1, 2, 17, 2, 5, -5, 2, 4], 11),
            Vec::<i64>::new()
        );
    }

    #[test]
    fn test_find_triple() {
        assert_eq!(
            find_triple_for_sum(vec![1, 2, 17, 2, 5, -5, 2, 4], 14),
            vec![-5, 17, 2]
        );
    }

    #[tokio::test]
    async fn test_find_triple_bigger() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/1/input")
            .with_status(200)
            .with_body(
                "1721
979
366
299
675
1456
",
            )
            .create();
        assert_eq!(
            find_triple_for_sum(get_vector_of_ints_from_input(2020, 1).await?, 2020),
            vec![675, 366, 979]
        );
        Ok(())
    }

    #[test]
    fn test_mult_vec() {
        assert_eq!(mult_vec(vec![10, 20]), 200);
    }
}
