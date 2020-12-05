use std::{
    collections::HashSet,
    iter::FromIterator,
};

use crate::common;
use reqwest::Error;

const LOWER_HALF_ROWS: char = 'F';
const UPPER_HALF_ROWS: char = 'B';
const LOWER_HALF_COLS: char = 'L';
const UPPER_HALF_COLS: char = 'R';

fn parse_boarding_passes(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .split('\n')
        .map(|s| (&s[..7], &s[7..]))
        .collect()
}

fn get_seat_id(row: &i64, col: &i64) -> i64 {
    row * 8 + col
}

fn evaluate_boarding_pass(pass: (&str, &str)) -> i64 {
    let (row_spec, col_spec) = pass;
    let row = row_spec.chars().fold((0..128).collect::<Vec<i64>>(), |rows, x| {
        let halfway = rows.len() / 2;
        match x {
            UPPER_HALF_ROWS => rows[halfway..].to_vec(),
            LOWER_HALF_ROWS => rows[..halfway].to_vec(),
            _ => panic!("unknown row range specifier: {}", x)
        }
    });
    let col = col_spec.chars().fold((0..8).collect::<Vec<i64>>(), |cols, x| {
        let halfway = cols.len() / 2;
        match x {
            UPPER_HALF_COLS => cols[halfway..].to_vec(),
            LOWER_HALF_COLS => cols[..halfway].to_vec(),
            _ => panic!("unknown col range specifier: {}", x)
        }
    });
    get_seat_id(row.first().unwrap(), col.first().unwrap())
}

fn find_missing_seat(seat_ids: &Vec<i64>) -> i64 {
    let max = seat_ids.iter().max().unwrap();
    let min = seat_ids.iter().min().unwrap();
    let seat_range: HashSet<i64> = HashSet::from_iter(*min..*max);
    let actual_seats: HashSet<i64> = HashSet::from_iter(seat_ids.iter().map(|seat|*seat));
    *seat_range.difference(&actual_seats).nth(0).unwrap()
}

pub async fn solve() -> Result<(), Error> {
    let seat_ids = parse_boarding_passes(&common::get_input(2020, 5).await?)
        .iter()
        .map(|pass| evaluate_boarding_pass(*pass))
        .collect::<Vec<i64>>();
    println!("Day 05 Part 1: {:?}", &seat_ids.iter().max().unwrap());
    println!("Day 05 Part 2: {:?}", find_missing_seat(&seat_ids));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use mockito;
    use reqwest::Error;
    use crate::common;

    #[tokio::test]
    async fn test_parse_boarding_passes() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/5/input")
            .with_body("BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
"
        ).create();
        assert_eq!(
            parse_boarding_passes(&common::get_input(2020, 5).await?),
            vec![("BFFFBBF", "RRR"), ("FFFBBBF", "RRR"), ("BBFFBBF", "RLL")]
        );
        Ok(())
    }

    #[test]
    fn test_evaluate_boarding_pass() {
        assert_eq!(evaluate_boarding_pass(("FBFBBFF", "RLR")), 357);
        assert_eq!(evaluate_boarding_pass(("BFFFBBF", "RRR")), 567);
        assert_eq!(evaluate_boarding_pass(("FFFBBBF", "RRR")), 119);
        assert_eq!(evaluate_boarding_pass(("BBFFBBF", "RLL")), 820);
    }

    #[test]
    fn test_find_missing_seat() {
        assert_eq!(find_missing_seat(&vec!(4,5,6,8,9,10)), 7);
    }

}