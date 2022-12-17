use std::collections::HashSet;

use crate::common;

use reqwest::Error;

enum Subroutine {
    MESSAGE,
    PACKET,
}

fn find_start_marker(datastream: &str, subroutine: Subroutine) -> Option<usize> {
    let window_size = match subroutine {
        Subroutine::MESSAGE => 14,
        Subroutine::PACKET => 4,
    };
    for (i, window) in datastream.as_bytes().windows(window_size).enumerate() {
        let hs: HashSet<char> = HashSet::from_iter(window.iter().map(|b| *b as char));
        if hs.len() == window_size {
            return Some(window_size + i);
        }
    }
    return None;
}

pub async fn solve() -> Result<(), Error> {
    let datastream = common::get_input(2022, 6).await?;
    println!(
        "Day 06 Part 1: {:?}",
        find_start_marker(datastream.trim(), Subroutine::PACKET).unwrap()
    );
    println!(
        "Day 06 Part 2: {:?}",
        find_start_marker(datastream.trim(), Subroutine::MESSAGE).unwrap()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
mjqjpqmgbljsphdztnvjfqwrcgsmlb
";

    #[test]
    fn test_find_start_of_packet_marker() {
        assert_eq!(
            find_start_marker(TEST_INPUT.trim(), Subroutine::PACKET).unwrap(),
            7
        )
    }

    #[test]
    fn test_find_start_of_message_marker() {
        assert_eq!(
            find_start_marker(TEST_INPUT.trim(), Subroutine::MESSAGE).unwrap(),
            19
        )
    }
}
