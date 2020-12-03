mod aoc_2020_day_03;

use dotenv;
use tokio;
use reqwest::{ Error, Client};

use aoc_2020_day_03::{ get_input, count_trees };

const SLOPE_PART_1 : (i64, i64) = (3, 1);

const SLOPES_PART_2 : [(i64, i64); 5]= [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
];

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let res = get_input(&Client::new(), &std::env::var("session").unwrap(), 2020, 3).await?;
    let ( right_jump, down_jump ) = SLOPE_PART_1;
    println!("Part One: {:?}", count_trees(&res, right_jump, down_jump));
    println!("Part Two: {:?}", SLOPES_PART_2.iter().map(|slope| {
        let (right_jump, down_jump) = slope;
        count_trees(&res, *right_jump, *down_jump)
    }).fold(1, |x,y| x*y));
    Ok(())
}
