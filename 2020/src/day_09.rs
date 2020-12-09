use crate::common;
use itertools::Itertools;
use reqwest::Error;

const PREAMBLE_LENGTH: u64 = 25;

fn parse_to_vec(input: &str) -> Vec<i64> {
    input
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_sums_for_preamble(data: &Vec<i64>, start: u64, preamble_length: u64) -> Vec<i64> {
    data[
        (start as usize)..((start+preamble_length) as usize)
    ].iter().combinations(2).map(|s| s[0] + s[1]).collect()
}

fn find_first_invalid(data: &Vec<i64>, preamble_length: u64) -> i64 {
    let mut check_from = 0;
    for i in data.iter().skip(preamble_length as usize) {
       if !get_sums_for_preamble(data, check_from, preamble_length).contains(i) {
           return *i
       }
       check_from += 1
    };
    panic!("All numbers after preamble were valid")
}

fn find_encryption_weakness(data: &Vec<i64>, target_num: i64) -> i64 {
    for i in 0..data.len() {
        let mut init = target_num;
        let mut pos = i;
        let mut contiguous_subset = Vec::new();
        while init > 0 {
            init -= data[pos];
            contiguous_subset.push(data[pos]);
            pos+=1;
        }
        if init == 0 && contiguous_subset.len() >= 2 {
            return *contiguous_subset.iter().min().unwrap() + *contiguous_subset.iter().max().unwrap()
        }
    }
    panic!("No contiguous subset found that sums to {}", target_num)
}

pub async fn solve() -> Result<(), Error> {
    let data = parse_to_vec(&common::get_input(2020, 9).await?);
    let first_invalid = find_first_invalid(&data, PREAMBLE_LENGTH);
    println!("Day 09 Part 1: {:?}", first_invalid);
    println!("Day 09 Part 2: {:?}", find_encryption_weakness(&data, first_invalid));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_vec() {
        assert_eq!(parse_to_vec(
"1
2
3
"), vec!(1,2,3));
    }

    #[test]
    fn test_find_first_invalid() {
        assert_eq!(find_first_invalid(&parse_to_vec("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"), 5), 127)
    } 

    #[test]
    fn test_find_encryption_weakness() {
        let data = parse_to_vec("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576");
        let target_num = find_first_invalid(&data, 5);
        assert_eq!(find_encryption_weakness(&data, target_num), 62)
    }
}