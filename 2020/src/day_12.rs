use crate::common;
use recap::Recap;
use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?x)
(?P<action>[A-Z]{1})
(?P<value>\d+)
"#)]
struct NavigationInstruction {
    action: char,
    value: u64,
}

struct Ship {
    x: i64,
    y: i64,
    heading: i64,
}

impl Ship {
    fn perform_navigation_part_one(&mut self, n: &NavigationInstruction) {
        match n.action {
            'L' => self.heading = (self.heading - n.value as i64).rem_euclid(360),
            'R' => self.heading = (self.heading + n.value as i64).rem_euclid(360),
            'N' => self.y += n.value as i64,
            'E' => self.x += n.value as i64,
            'S' => self.y -= n.value as i64,
            'W' => self.x -= n.value as i64,
            'F' => match self.heading {
                0 => self.y += n.value as i64,
                90 => self.x += n.value as i64,
                180 => self.y -= n.value as i64,
                270 => self.x -= n.value as i64,
                _ => panic!("Invalid heading: {}", self.heading),
            },
            _ => panic!("Invalid action: {}", n.action),
        }
    }

    fn perform_navigation_part_two(
        &mut self,
        n: &NavigationInstruction,
        waypoint: (i64, i64),
    ) -> (i64, i64) {
        match n.action {
            'L' | 'R' => {
                let transformed = if n.action == 'L' {
                    360 - n.value
                } else {
                    n.value
                };
                match transformed {
                    0 => return waypoint,
                    90 => return (waypoint.1, -1 * waypoint.0),
                    180 => return (-1 * waypoint.0, -1 * waypoint.1),
                    270 => return (-1 * waypoint.1, waypoint.0),
                    _ => panic!("Invalid heading: {}", transformed),
                }
            }
            'F' => {
                self.x += waypoint.0 * n.value as i64;
                self.y += waypoint.1 * n.value as i64;
                return waypoint;
            }
            'N' => return (waypoint.0, waypoint.1 + n.value as i64),
            'E' => return (waypoint.0 + n.value as i64, waypoint.1),
            'S' => return (waypoint.0, waypoint.1 - n.value as i64),
            'W' => return (waypoint.0 - n.value as i64, waypoint.1),
            _ => panic!("Invalid action: {}", n.action),
        }
    }
}

fn get_instructions_from_input(input: &str) -> Vec<NavigationInstruction> {
    input
        .trim()
        .split('\n')
        .map(|s| s.parse::<NavigationInstruction>().unwrap())
        .collect()
}

pub async fn solve() -> Result<(), Error> {
    let instructions = get_instructions_from_input(&common::get_input(2020, 12).await?);
    let mut ship_one = Ship {
        x: 0,
        y: 0,
        heading: 90,
    };
    instructions
        .iter()
        .for_each(|i| ship_one.perform_navigation_part_one(i));
    println!("Day 12 Part 1: {:?}", ship_one.x.abs() + ship_one.y.abs());
    let mut ship_two = Ship {
        x: 0,
        y: 0,
        heading: 90,
    };
    let waypoint = (10, 1);
    instructions.iter().fold(waypoint, |wp, i| {
        ship_two.perform_navigation_part_two(i, wp)
    });
    println!("Day 12 Part 2: {:?}", ship_two.x.abs() + ship_two.y.abs());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "F10
N3
F7
R90
F11
";

    #[test]
    fn test_parse_instructions() {
        let instructions = get_instructions_from_input(TEST_INPUT);
        assert_eq!(instructions.len(), 5);
    }

    #[test]
    fn test_perform_navigation_part_one() {
        let mut s = Ship {
            x: 0,
            y: 0,
            heading: 90,
        };
        let instructions = get_instructions_from_input(TEST_INPUT);
        instructions
            .iter()
            .for_each(|i| s.perform_navigation_part_one(i));
        assert_eq!((s.x, s.y), (17, -8));
    }

    #[test]
    fn test_perform_navigation_part_two() {
        let mut s = Ship {
            x: 0,
            y: 0,
            heading: 90,
        };
        let instructions = get_instructions_from_input(TEST_INPUT);
        instructions.iter().fold((10, 1), |waypoint, i| {
            s.perform_navigation_part_two(i, waypoint)
        });
        assert_eq!((s.x, s.y), (214, -72));
    }
}
