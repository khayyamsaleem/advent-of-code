use std::collections::BinaryHeap;
use std::collections::binary_heap::IntoIterSorted;

use reqwest::Error;

use crate::common;

fn build_heap_of_elf_meal_calorie_totals(input: &str) -> IntoIterSorted<i64> {
    let mut meals: BinaryHeap<i64> = BinaryHeap::new();
    for meal in input.trim().split("\n\n") {
        meals.push(
            meal.trim().split("\n")
            .map(|food_cals| food_cals.parse::<i64>().unwrap())
            .sum::<i64>()
        )
    }
    meals.into_iter_sorted()
}

fn get_total_cals_for_top_n_elves(
    meals: &mut IntoIterSorted<i64>,
    n: usize
) -> i64 {
    meals.take(n).sum()
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 1).await?;
    let mut heap = build_heap_of_elf_meal_calorie_totals(&input);
    let max_cal_meal = get_total_cals_for_top_n_elves(&mut heap, 1);
    println!(
        "Day 01 Part 1: {:?}",
        max_cal_meal
    );
    println!(
        "Day 01 Part 2: {:?}",
        max_cal_meal + get_total_cals_for_top_n_elves(&mut heap, 2)
        // because the first was already consumed in part 1
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
    fn test_get_total_cals_for_top_n_elves() {
        let mut heap = build_heap_of_elf_meal_calorie_totals(&TEST_INPUT);
        assert_eq!(get_total_cals_for_top_n_elves(&mut heap, 3), 45000);
    }

}
