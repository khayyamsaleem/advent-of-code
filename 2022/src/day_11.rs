use crate::common;

use itertools::Itertools;
use recap::Recap;
use reqwest::Error;
use serde::Deserialize;


#[derive(Debug,Deserialize,PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Subtract => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b, // Note: Integer division
        }
    }
}

impl std::str::FromStr for Operator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err("Unknown operator"),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Recap)]
#[recap(
    regex = r"Monkey (?P<monkey_id>\d+):\n  Starting items: (?P<initial_worry_levels>[\d, ]+)\n  Operation: new = old (?P<op_operator>[+\-*/]) (?P<op_operand>.+)\n  Test: divisible by (?P<divisibility_test>\d+)\n    If true: throw to monkey (?P<test_true_branch_monkey>\d+)\n    If false: throw to monkey (?P<test_false_branch_monkey>\d+)"
)]
struct MonkeyBehavior {
    monkey_id: u64,
    initial_worry_levels: String,
    op_operator: String,
    op_operand: String,
    divisibility_test: i64,
    test_true_branch_monkey: u64,
    test_false_branch_monkey: u64,
}

fn evaluate_rounds(num_rounds: u64, behaviors: &Vec<MonkeyBehavior>, worry_modifier: &dyn Fn(i64)->f64) -> Vec<(usize, Vec<i64>)> {
    let mut out = vec![(0,vec![]); behaviors.len()];
    for _round in 0..num_rounds {
        for monkey in behaviors {
            let items = monkey.initial_worry_levels.split(", ").map(|wl|wl.parse::<i64>().unwrap());
            let mut inspected_items = 0;
            items.chain(out[monkey.monkey_id as usize].1.clone()).for_each(|wl| {
                inspected_items += 1;
                let new_worry_level = worry_modifier(monkey.op_operator.parse::<Operator>().unwrap().apply(
                    wl as i64,
                    if monkey.op_operand == "old" { wl as i64 } else { monkey.op_operand.parse::<i64>().unwrap() })
                ).floor() as i64;
                out[
                    if new_worry_level % monkey.divisibility_test == 0 {
                        monkey.test_true_branch_monkey
                    } else {
                        monkey.test_false_branch_monkey
                    } as usize
                ].1.push(new_worry_level);
            });
            out[monkey.monkey_id as usize] = (inspected_items, vec![]);
        }
    }
    out
}

fn calculate_monkey_business(round_results: Vec<(usize,Vec<i64>)>) -> i64 {
    round_results.iter().map(|x|x.0).sorted().rev().take(2).fold(1, |acc,x| acc*x as i64)
}

pub async fn solve() -> Result<(), Error> {
    let results = evaluate_rounds(20, &common::get_input(2022, 11).await?.split("\n\n").map(|mb| mb.parse().unwrap()).collect_vec(), &|x|x as f64/3.0);
    println!("Day 11 Part 1: {}", calculate_monkey_business(results));
    Ok(())
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_evaluate_rounds() {
        let monkeys = TEST_INPUT.split("\n\n").map(|mb| mb.parse::<MonkeyBehavior>().unwrap()).collect_vec();
        let results = evaluate_rounds(1, &monkeys, &|x|x as f64/3.0);
        assert_eq!(4, results[0].1.len());
        assert_eq!(6, results[1].1.len());
        assert_eq!(0, results[2].1.len());
        assert_eq!(0, results[3].1.len());

        let results_20 = evaluate_rounds(20, &monkeys, &|x|x as f64/3.0);
        assert_eq!(101, results_20[0].0);
        assert_eq!(95, results_20[1].0);
        assert_eq!(7, results_20[2].0);
        assert_eq!(105, results_20[3].0);

        assert_eq!(10605, calculate_monkey_business(results_20));
    }

    #[test]
    fn test_monkey_behavior_deserialization() {
        let test_data = r"Monkey 1:
  Starting items: 10
  Operation: new = old + 5
  Test: divisible by 15
    If true: throw to monkey 4
    If false: throw to monkey 5";

        let expected_behavior = MonkeyBehavior {
            monkey_id: 1,
            initial_worry_levels: "10".to_string(),
            op_operator: "+".to_string(),
            op_operand: "5".to_string(),
            divisibility_test: 15,
            test_true_branch_monkey: 4,
            test_false_branch_monkey: 5,
        };

        let parsed_behavior: MonkeyBehavior = test_data.parse().expect("Failed to parse data");
        assert_eq!(parsed_behavior, expected_behavior);
    }

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
