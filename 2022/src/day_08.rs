use crate::common;

use reqwest::Error;

const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn build_forest(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|t| t.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn count_trees_visible_from_outside(forest: &Vec<Vec<u8>>) -> usize {
    let forest_height = forest.len() as i8;
    let forest_width = forest[0].len() as i8;
    let mut visible_trees = 0;
    for (i, row) in forest.iter().enumerate() {
        for (j, tree_height) in row.iter().enumerate() {
            for d in DIRECTIONS {
                let mut check = (i as i8 + d.0, j as i8 + d.1);
                while check.0 < forest_height
                    && check.0 >= 0
                    && check.1 < forest_width
                    && check.1 >= 0
                    && forest[check.0 as usize][check.1 as usize] < *tree_height
                {
                    check = (check.0 + d.0, check.1 + d.1);
                }
                if check.0 < 0 || check.0 == forest_height || check.1 < 0 || check.1 == forest_width
                {
                    visible_trees += 1;
                    break;
                }
            }
        }
    }
    visible_trees
}

fn find_max_scenic_score_in_forest(forest: &Vec<Vec<u8>>) -> u64 {
    let forest_height = forest.len() as i8;
    let forest_width = forest[0].len() as i8;
    let mut scenic_scores = vec![];
    for (i, row) in forest.iter().enumerate() {
        for (j, tree_height) in row.iter().enumerate() {
            let mut trees_visible_from_tree_per_direction = vec![];
            for d in DIRECTIONS {
                let mut check = (i as i8 + d.0, j as i8 + d.1);
                let mut trees_visible_in_current_direction = 0;
                while check.0 < forest_height
                    && check.0 >= 0
                    && check.1 < forest_width
                    && check.1 >= 0
                {
                    trees_visible_in_current_direction += 1;
                    let checked_tree_height = forest[check.0 as usize][check.1 as usize];
                    if checked_tree_height >= *tree_height {
                        break;
                    }
                    check = (check.0 + d.0, check.1 + d.1);
                }
                trees_visible_from_tree_per_direction.push(trees_visible_in_current_direction);
            }
            scenic_scores.push(trees_visible_from_tree_per_direction.iter().product())
        }
    }
    return *scenic_scores.iter().max().unwrap();
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 8).await?;
    let forest = build_forest(&input);
    println!(
        "Day 08 Part 1: {:?}",
        count_trees_visible_from_outside(&forest)
    );
    println!(
        "Day 08 Part 2: {:?}",
        find_max_scenic_score_in_forest(&forest)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_08::find_max_scenic_score_in_forest;

    use super::{build_forest, count_trees_visible_from_outside};

    const TEST_INPUT: &str = "
30373
25512
65332
33549
35390
";

    #[test]
    fn test_count_trees_visible_from_outside() {
        let forest = build_forest(TEST_INPUT);
        assert_eq!(count_trees_visible_from_outside(&forest), 21);
    }

    #[test]
    fn test_find_max_scenic_score_in_forest() {
        let forest = build_forest(TEST_INPUT);
        assert_eq!(find_max_scenic_score_in_forest(&forest), 8);
    }
}
