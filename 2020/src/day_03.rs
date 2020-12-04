use reqwest::{
    Error,
    Client,
    header::{ COOKIE }
};

#[cfg(test)]
use mockito;

const SLOPE_PART_1 : (i64, i64) = (3, 1);

const SLOPES_PART_2 : [(i64, i64); 5]= [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
];

async fn get_input(
    client: &reqwest::Client,
    token: &str,
    year: i64,
    day: i64
) -> Result<Vec<Vec<char>>, Error> {
    #[cfg(not(test))]
    let base_url = "https://adventofcode.com";

    #[cfg(test)]
    let base_url = &mockito::server_url();

    let res = client.get(&format!("{}/{}/day/{}/input", base_url, year, day))
        .header(COOKIE, ["session", token].join("="))
        .send().await?
        .text().await?;

    Ok(res.trim().split("\n").map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>())
}

fn count_trees(map: &Vec<Vec<char>>, right_jump: i64, down_jump: i64) -> i64 {
    const TREE: char = '#';
    let height = map.len();
    let mut num_trees = 0;
    let (mut x_pos, mut y_pos) = (0, 0);
    while y_pos < height {
        num_trees += if map[y_pos][x_pos % map[y_pos].len()] == TREE {1} else {0};
        x_pos += right_jump as usize;
        y_pos += down_jump as usize;
    }
    return num_trees;
}

pub async fn solve() -> Result<(), Error> {

    dotenv::dotenv().ok();
    let res = get_input(&Client::new(), &std::env::var("session").unwrap(), 2020, 3).await?;
    let ( right_jump, down_jump ) = SLOPE_PART_1;
    println!("Day 03 Part 1: {:?}", count_trees(&res, right_jump, down_jump));
    println!("Day 03 Part 2: {:?}", SLOPES_PART_2.iter().map(|slope| {
        let (right_jump, down_jump) = slope;
        count_trees(&res, *right_jump, *down_jump)
    }).fold(1, |x,y| x*y));
    Ok(())
}


#[cfg(test)]
mod tests {
    use mockito;
    use reqwest::{ Client };
    use super::*;

    #[tokio::test]
    async fn test_get_input() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/1/input")
        .with_status(200)
        .with_body("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"
        ).create();

        let test_input = get_input(&Client::new(), "token", 2020, 1).await?;
        assert_eq!(test_input.len(), 11);
        assert_eq!(test_input[0].len(), 11);
        Ok(())
    }

    #[tokio::test]
    async fn test_count_trees_against_test_input() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/1/input")
        .with_status(200)
        .with_body("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"
        ).create();

        let test_input = get_input(&Client::new(), "token", 2020, 1).await?;
        assert_eq!(count_trees(&test_input, 3, 1), 7);

        Ok(())
    }


}
