use std::ffi::{CStr};
use std::os::raw::c_char;

use recap::{from_all_captures, Regex,Recap};
use serde::Deserialize;

#[derive(Debug,Deserialize,PartialEq,Recap)]
#[recap(regex=r#"mul\((?P<x>\d+),(?P<y>\d+)\)"#)]
struct Mul {
    x: i64,
    y: i64,
}

impl Mul {
    const PATTERN: &str = r#"mul\((?P<x>\d+),(?P<y>\d+)\)"#;
    fn apply(&self) -> i64 {
        return self.x * self.y;
    }
}

fn part_one(input: &str) -> i64 {
    let re = Regex::new(Mul::PATTERN).unwrap();
    from_all_captures(&re, input).unwrap().iter().fold(0, |acc,x: &Mul| acc + x.apply())
}

fn part_two(input: &str) -> i64 {
    let re = Regex::new(vec![Mul::PATTERN,r#"don't\(\)"#,r#"do\(\)"#].join("|").as_str()).unwrap();

    re.find_iter(input).fold((0,true), |(acc,doit),x| match x.as_str() {
        "don't()" => (acc,false),
        "do()" => (acc,true),
        m => if doit {(acc + m.parse::<Mul>().unwrap().apply(), doit)} else {(acc, doit)}
    }).0
}

#[no_mangle]
pub extern "C" fn solve(input_ptr: *const c_char) {
    let c_str = unsafe {
        assert!(!input_ptr.is_null());
        CStr::from_ptr(input_ptr)
    };

    println!("Day 03 - Part 1: {}", part_one(c_str.to_str().unwrap()));
    println!("Day 03 - Part 2: {}", part_two(c_str.to_str().unwrap()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_mul() {
        let re = Regex::new(Mul::PATTERN).unwrap();
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let results: Vec<Mul> = from_all_captures(&re, input).unwrap();

        assert_eq!(results, vec![Mul{x:2,y:4},Mul{x:5,y:5},Mul{x:11,y:8},Mul{x:8,y:5}]);
    }

    #[test]
    fn test_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(input), 161);
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_two(input), 48);
    }
}
