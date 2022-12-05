use crate::common;

use reqwest::Error;


type Stack = Vec;

pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 4).await?;
    panic!("not implemented")
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

}
