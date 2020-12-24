use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // aoc_2020::day_01::solve().await?;
    // aoc_2020::day_02::solve().await?;
    // aoc_2020::day_03::solve().await?;
    // aoc_2020::day_04::solve().await?;
    // aoc_2020::day_05::solve().await?;
    // aoc_2020::day_06::solve().await?;
    // aoc_2020::day_07::solve().await?;
    // aoc_2020::day_08::solve().await?;
    // aoc_2020::day_09::solve().await?;
    // aoc_2020::day_10::solve().await?;
    // aoc_2020::day_11::solve().await?;
    // aoc_2020::day_12::solve().await?;
    aoc_2020::day_13::solve().await?;
    Ok(())
}
