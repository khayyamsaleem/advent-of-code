use std::collections::BinaryHeap;

use reqwest::Error;

use crate::common;

fn parse_elf_meal_totals(input: &str) -> impl Iterator<Item=i64> + '_ {
    input
        .trim()
        .split("\n\n")
        .map(
            |elf_foods|
            elf_foods.trim()
                .split("\n")
                .map(|food_cals| food_cals.parse::<i64>().unwrap())
                .sum::<i64>()
        )
}

fn get_max_cal_count_for_elves(input: &str) -> i64 {
    parse_elf_meal_totals(input)
        .max()
        .unwrap()

}

fn get_total_cals_for_top_n_elves(input: &str, n: usize) -> i64 {
    BinaryHeap::from_iter(parse_elf_meal_totals(input)).into_iter_sorted().take(n).sum()
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 1).await?;
    println!(
        "Day 01 Part 1: {:?}",
        get_max_cal_count_for_elves(&input)
    );
    println!(
        "Day 01 Part 2: {:?}",
        get_total_cals_for_top_n_elves(&input, 3)
    );
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT : &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_get_max_cal_count_for_elves() {
        assert_eq!(get_max_cal_count_for_elves(TEST_INPUT), 24000);
    }

    #[test]
    fn test_get_total_cals_for_top_n_elves() {
        assert_eq!(get_total_cals_for_top_n_elves(TEST_INPUT, 3), 45000);
    }

}
