use crate::common;
use reqwest::Error;
use std::collections::HashMap;

const VALID_JOLT_DIFFERENCES: [u64; 3] = [1, 2, 3];

fn create_vec_from_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn count_jolt_differences_and_use_adapters(adapters: &mut Vec<u64>) -> HashMap<u64, u64> {
    let mut jolt_differences: HashMap<u64, u64> = hashmap!(1 => 0, 2 => 0, 3 => 0);
    adapters.insert(0, 0);
    adapters.sort();
    adapters.append(&mut vec![adapters.last().unwrap() + 3]);
    for i in 0..(adapters.len() - 1) {
        let diff = adapters[i + 1] - adapters[i];
        if !VALID_JOLT_DIFFERENCES.contains(&diff) {
            panic!("Jolt difference of {} not allowed", diff)
        };
        jolt_differences.insert(diff, jolt_differences[&diff] + 1);
    }
    return jolt_differences;
}

fn count_adapter_arrangements(adapters: &mut Vec<u64>) -> u64 {
    adapters.insert(0, 0);
    adapters.sort();
    adapters.append(&mut vec![adapters.last().unwrap() + 3]);
    let length = adapters.len();
    let seen = hashmap!();
    struct Ctx {
        adapters: Vec<u64>,
        length: usize,
        seen: HashMap<usize, u64>,
    }
    fn count(pos: usize, ctx: &mut Ctx) -> u64 {
        if pos == ctx.length - 1 {
            return 1;
        };
        if ctx.seen.contains_key(&pos) {
            return ctx.seen[&pos];
        };
        let mut arrangements = count(pos + 1, ctx);
        if pos < ctx.length - 2 && ctx.adapters[pos + 2] <= ctx.adapters[pos] + 3 {
            arrangements += count(pos + 2, ctx)
        }
        if pos < ctx.length - 3 && ctx.adapters[pos + 3] <= ctx.adapters[pos] + 3 {
            arrangements += count(pos + 3, ctx)
        }
        ctx.seen.insert(pos, arrangements);
        arrangements
    }
    count(
        0,
        &mut Ctx {
            adapters: adapters.to_vec(),
            length,
            seen,
        },
    )
}

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2020, 10).await?;
    let mut data = create_vec_from_input(&input);
    let mut data_again = data.clone();
    let jolt_differences = count_jolt_differences_and_use_adapters(&mut data);
    println!(
        "Day 10 Part 1: {:?}",
        jolt_differences[&3] * jolt_differences[&1]
    );
    println!(
        "Day 10 Part 2: {:?}",
        count_adapter_arrangements(&mut data_again)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    const TEST_INPUT: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn test_count_jolt_differences() {
        let mut adapters = create_vec_from_input(TEST_INPUT);
        let differences = count_jolt_differences_and_use_adapters(&mut adapters);
        assert_eq!(differences, hashmap!(1 => 22, 3 => 10, 2 => 0));
    }

    #[test]
    fn test_count_adapter_arrangements() {
        let mut adapters = create_vec_from_input(TEST_INPUT);
        assert_eq!(count_adapter_arrangements(&mut adapters), 19208);
    }
}
