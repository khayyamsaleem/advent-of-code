use crate::common;

use reqwest::Error;

#[derive(Debug,Clone)]
struct State {
    x: i64,
}

const CRT_WIDTH: i64 = 40;

fn run_instructions(initial_state: State, instructions: &str) -> Vec<State> {
    instructions.trim().split("\n").into_iter().fold(vec![initial_state], |mut acc, instruction| {
        let prev_state = acc.last().unwrap();
        acc.extend(match instruction.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => vec![State { x: prev_state.x }],
            ["addx", arg] => vec![
                State { x: prev_state.x },
                State { x: prev_state.x + arg.parse::<i64>().unwrap() }
            ],
            _ => panic!("unexpected instruction")
        });
        acc
    })
}

fn get_signal_strength(states: &Vec<State>, cycle: usize) -> i64 {
    ((cycle as i64) + 1) * states[cycle].x
}

fn get_signal_strength_sum(states: &Vec<State>, initial_cycle: usize, shift: usize, num_steps: usize) -> i64 {
    (initial_cycle-1..states.len()).step_by(shift).take(num_steps).map(|c| get_signal_strength(states, c)).sum()
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 10).await?;
    let states = run_instructions(State {x: 1}, &input);
    println!("Day 10 Part 1: {:?}", get_signal_strength_sum(&states, 20, 40, 6));
    println!("Day 10 Part 2: \n{}\n", crt_render(states));
    Ok(())
}

fn crt_render(states: Vec<State>) -> String {
    let mut pixels = vec![];
    let mut i: i64 = 0;
    states.iter().for_each(|state|{
        i = if i == CRT_WIDTH { 0 } else { i };
        pixels.push(if state.x+1 == i || state.x-1 == i || state.x == i { '#' } else {'.'});
        i = i + 1;
    });
    pixels.iter().rev().skip(1).rev().enumerate().fold("".to_string(), |acc, (i, p)|{
        if i != 0 && i as i64 % CRT_WIDTH == 0 {
            return acc + &format!("\n{}", p)
        }
        return acc + &p.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_position() {
        let states = run_instructions(State {x: 1}, TEST_INPUT);
        assert_eq!(crt_render(states), TEST_EXPECTED_IMAGE);
    }

    #[test]
    fn test_run_instructions() {
        let states = run_instructions(State {x: 1}, TEST_INPUT);
        assert_eq!(states[20-1].x, 21);
        assert_eq!(states[60-1].x, 19);
        assert_eq!(states[100-1].x, 18);
        assert_eq!(states[140-1].x, 21);
        assert_eq!(states[180-1].x, 16);
        assert_eq!(states[220-1].x, 18);
        assert_eq!(get_signal_strength_sum(&states, 20, 40, 6), 13140);
    }

    const TEST_INPUT: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

        const TEST_EXPECTED_IMAGE: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

}
