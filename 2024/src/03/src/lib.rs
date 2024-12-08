use std::ffi::{CStr};
use std::os::raw::c_char;

use recap::{from_all_captures, Regex};
use serde::Deserialize;

#[derive(Debug,Deserialize,PartialEq)]
struct Mul {
    x: i16,
    y: i16,
}

impl Mul {
    fn apply(&self) -> i16 {
        return self.x * self.y;
    }
}

fn part_one(input: &str) -> i16 {
    let re = Regex::new(r#"mul\((?P<x>\d+),(?P<y>\d+)\)"#).unwrap();
    from_all_captures(&re, input).unwrap().iter().fold(0, |acc,x: &Mul| acc + x.apply())
}

#[no_mangle]
pub extern "C" fn solve(input_ptr: *const c_char) {
    let c_str = unsafe {
        assert!(!input_ptr.is_null());
        CStr::from_ptr(input_ptr)
    };

    println!("Day 03 - Part 1: {}", part_one(c_str.to_str().unwrap()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_mul() {
        let re = Regex::new(r#"mul\((?P<x>\d+),(?P<y>\d+)\)"#).unwrap();
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let results: Vec<Mul> = from_all_captures(&re, input).unwrap();

        assert_eq!(results, vec![Mul{x:2,y:4},Mul{x:5,y:5},Mul{x:11,y:8},Mul{x:8,y:5}]);
    }

    #[test]
    fn test_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(input), 161);
    }
}
