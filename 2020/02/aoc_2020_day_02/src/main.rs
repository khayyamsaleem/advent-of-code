mod aoc_2020_day_02;

use tokio;
use reqwest::{ Error, Client };
use aoc_2020_day_02::{ get_input, test_policy_one, test_policy_two };
use dotenv;

#[tokio::main]
async fn main() -> Result<(), Error>{
    dotenv::dotenv().ok();
    let res = get_input(&Client::new(), &std::env::var("session").unwrap(), 2020, 2).await?;
    println!("Part One: {:?}", res.iter().filter(|t| test_policy_one(&t.0, &t.1)).count());
    println!("Part Two: {:?}", res.iter().filter(|t| test_policy_two(&t.0, &t.1)).count());
    Ok(())
}