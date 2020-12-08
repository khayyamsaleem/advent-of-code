use crate::common;
use reqwest::Error;

const NOP: &str = "nop";
const JMP: &str = "jmp";
const ACC: &str = "acc";

fn create_instruction_vector(input: &str) -> Vec<(&str, i64)> {
    input
        .trim()
        .split('\n')
        .map(|s| {
            let mut iter = s.split(" ");
            (
                iter.next().unwrap(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn eval_instructions(instructions: &Vec<(&str, i64)>) -> i64 {
    let mut cur = 0;
    let mut acc = 0;
    let mut visited = Vec::new();
    while !visited.contains(&cur) {
        visited.push(cur);
        match instructions[cur] {
            (NOP, _x) => {
                cur += 1;
            }
            (ACC, x) => {
                acc += x;
                cur += 1;
            }
            (JMP, x) => cur = (cur as i64 + x) as usize,
            (i, _x) => panic!("No such instruction: {}", i),
        }
    }
    acc
}

fn eval_instructions_if_terminates(instructions: &Vec<(&str, i64)>) -> Result<i64, ()> {
    let mut cur = 0;
    let mut visited = Vec::new();
    let mut acc = 0;
    while !visited.contains(&cur) && cur < instructions.len() {
        visited.push(cur);
        match instructions[cur] {
            (NOP, _x) => {
                cur += 1;
            }
            (ACC, x) => {
                acc += x;
                cur += 1;
            }
            (JMP, x) => cur = (cur as i64 + x) as usize,
            (i, _x) => panic!("No such instruction: {}", i),
        }
    }
    if visited.contains(&cur) {
        Err(())
    } else {
        return Ok(acc);
    }
}

fn try_switching_jmps_and_nops(instructions: Vec<(&str, i64)>) -> Vec<Vec<(&str, i64)>> {
    let mut positions_to_swap = Vec::new();
    let mut pos = 0;
    instructions.iter().for_each(|(instruction, _y)| {
        if *instruction == JMP || *instruction == NOP {
            positions_to_swap.push(pos);
        }
        pos += 1;
    });
    positions_to_swap
        .iter()
        .map(|pos| {
            let (current, i) = instructions[*pos];
            let mut modified = instructions.clone();
            modified[*pos] = if current == JMP { (NOP, i) } else { (JMP, i) };
            modified.to_owned()
        })
        .collect()
}

fn eval_all_possible_instructions(instructions: Vec<(&str, i64)>) -> i64 {
    try_switching_jmps_and_nops(instructions)
        .iter()
        .map(|i| eval_instructions_if_terminates(i))
        .filter(|r| r.is_ok())
        .next()
        .unwrap()
        .unwrap()
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2020, 8).await?;
    let instructions = create_instruction_vector(&input);
    println!("Day 08 Part 1: {:?}", eval_instructions(&instructions));
    println!(
        "Day 08 Part 2: {:?}",
        eval_all_possible_instructions(instructions)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;
    use mockito;
    use reqwest::Error;
    use tokio;

    #[tokio::test]
    async fn test_create_instruction_vector() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/8/input")
            .with_body(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
",
            )
            .create();
        assert_eq!(
            create_instruction_vector(&common::get_input(2020, 8).await?),
            vec!(
                ("nop", 0),
                ("acc", 1),
                ("jmp", 4),
                ("acc", 3),
                ("jmp", -3),
                ("acc", -99),
                ("acc", 1),
                ("jmp", -4),
                ("acc", 6)
            )
        );
        Ok(())
    }

    #[test]
    fn test_eval_instructions() {
        let instructions = vec![
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("jmp", -4),
            ("acc", 6),
        ];
        assert_eq!(eval_instructions(&instructions), 5)
    }

    #[test]
    fn test_try_switch_jmps_nops() {
        let instructions = vec![
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("jmp", -4),
            ("acc", 6),
        ];
        let possible_instructions = try_switching_jmps_and_nops(instructions);
        assert_eq!(possible_instructions.len(), 4)
    }

    #[test]
    fn test_eval_all_possible_instructions() {
        let instructions = vec![
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("jmp", -4),
            ("acc", 6),
        ];
        assert_eq!(eval_all_possible_instructions(instructions), 8);
    }
}
