use reqwest::Error;
use crate::common;

#[derive(Debug, PartialEq)]
struct BusSchedule {
    departure_timestamp: u64,
    bus_ids: Vec<u64>
}

fn parse_schedule(input: &str) -> BusSchedule {
    let mut parts = input
        .trim()
        .split('\n');
    BusSchedule {
        departure_timestamp: parts.next().unwrap().parse::<u64>().unwrap(),
        bus_ids: parts.next().unwrap().split(',').filter(|s| *s != "x").map(|s| s.parse::<u64>().unwrap()).collect()
    }
}

fn find_earliest_bus_and_departure_time(schedule: &BusSchedule) -> (u64, u64) {
    let bus_and_difference = schedule.bus_ids
        .iter()
        .map(|id| (id, *id * (schedule.departure_timestamp as f64 / *id as f64).ceil() as u64 - schedule.departure_timestamp))
        .min_by_key(|x| x.1)
        .unwrap();
    (*bus_and_difference.0, schedule.departure_timestamp + bus_and_difference.1)
}

pub async fn solve() -> Result<(), Error> {
    let schedule = parse_schedule(&common::get_input(2020, 13).await?);
    let (earliest_bus_id, earliest_departure) = find_earliest_bus_and_departure_time(&schedule);
    println!("Day 13 Part 1: {:?}", earliest_bus_id * (earliest_departure - schedule.departure_timestamp));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "939
7,13,x,x,59,x,31,19
";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_schedule(TEST_INPUT),
            BusSchedule { departure_timestamp: 939, bus_ids: vec!(7,13,59,31,19) }
        )
    }

    #[test]
    fn test_find_earliest_bus() {
        let schedule = parse_schedule(TEST_INPUT);
        assert_eq!(find_earliest_bus_and_departure_time(&schedule).0, 59);
    }
}