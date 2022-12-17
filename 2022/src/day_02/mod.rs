use crate::common;

use itertools::Itertools;
use reqwest::{Client, Error};

#[cfg(test)]
use mockito;

fn to_shape(m: &str) -> &str {
    match m {
        "A" | "X" => "rock",
        "B" | "Y" => "paper",
        "C" | "Z" => "scissors",
        _ => panic!("Invalid move: {}", m),
    }
}

fn to_goal(e: &str) -> &str {
    match e {
        "X" => "lose",
        "Y" => "draw",
        "Z" => "win",
        _ => panic!("Invalid endstate: {}", e),
    }
}

fn game_from_input(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .split("\n")
        .map(|round| match round.split(" ").next_tuple().unwrap() {
            (they, me) => (to_shape(they), to_shape(me)),
        })
        .collect()
}

fn game_from_input_clarified(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .split("\n")
        .map(|round| match round.split(" ").next_tuple().unwrap() {
            (they, g) => (to_shape(they), to_goal(g)),
        })
        .collect()
}

async fn solve_with_prolog(game: &Vec<(&str, &str)>, strategy: &str) -> Result<i64, Error> {
    #[cfg(not(test))]
    let base_url = "http://localhost:42069";
    #[cfg(test)]
    let base_url = &mockito::server_url();

    Ok(Client::new()
        .post(&format!("{}/api/v{}/score", base_url, strategy))
        .json(game)
        .send()
        .await?
        .text()
        .await?
        .parse::<i64>()
        .unwrap())
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 2).await?;
    println!(
        "Day 02 Part 1: {:?}",
        solve_with_prolog(&game_from_input(&input), "1").await?
    );
    println!(
        "Day 02 Part 2: {:?}",
        solve_with_prolog(&game_from_input_clarified(&input), "2").await?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
A Y
B X
C Z
";

    #[test]
    fn test_game_from_input() {
        assert_eq!(
            game_from_input(TEST_INPUT),
            vec![
                ("rock", "paper"),
                ("paper", "rock"),
                ("scissors", "scissors")
            ]
        )
    }

    #[test]
    fn test_game_from_input_clarified() {
        assert_eq!(
            game_from_input_clarified(TEST_INPUT),
            vec![("rock", "draw"), ("paper", "lose"), ("scissors", "win")]
        )
    }
}
