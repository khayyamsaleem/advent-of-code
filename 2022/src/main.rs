use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // aoc_2022::day_01::solve().await?;
    aoc_2022::day_02::solve().await?;
    Ok(())
}
