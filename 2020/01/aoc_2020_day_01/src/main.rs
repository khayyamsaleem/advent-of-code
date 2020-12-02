mod aoc_2020_day_01;

extern crate dotenv;
extern crate reqwest;

use std::{ env, time::{ Duration } };
use reqwest::{ Error, Client };
use tokio;
use aoc_2020_day_01::{ find_pair_for_sum, find_triple_for_sum, get_input, mult_vec };

const SESSION: &'static str = "session";
const TARGET_SUM: i64 = 2020;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let timeout = Duration::new(5, 0);
    let client = Client::builder().timeout(timeout).build()?;


    dotenv::dotenv().ok();
    let token = env::var(SESSION).unwrap();
    let result = get_input(&client, token.as_str(), 2020, 1).await?;
    let another_result = result.clone();
    println!("Part One: {:?}", mult_vec(find_pair_for_sum(result, TARGET_SUM)));
    println!("Part Two: {:?}", mult_vec(find_triple_for_sum(another_result, TARGET_SUM)));
    Ok(())
}
