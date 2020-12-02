use reqwest::{ Error, header::{ COOKIE } };
use regex::Regex;

#[cfg(test)]
use mockito;

#[derive(Debug)]
pub struct Policy {
    character: char,
    min: usize,
    max: usize,
}

pub async fn get_input(
    client: &reqwest::Client,
    token: &str,
    year: i64,
    day: i64
) -> Result<Vec<(Policy, String)>, Error> {
    #[cfg(not(test))]
    let base_url = "https://adventofcode.com";

    #[cfg(test)]
    let base_url = &mockito::server_url();
    
    let res = client.get(&format!("{}/{}/day/{}/input", base_url, year, day))
        .header(COOKIE, ["session", token].join("="))
        .send().await?
        .text().await?;

    Ok(res.trim().split("\n").map(line_to_policy_pair).collect::<Vec<(Policy, String)>>())
}

fn line_to_policy_pair(line: &str) -> (Policy, String) {
    let re = Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<requirement>\w{1}): (?P<test_string>\w*)").unwrap();
    match re.captures(line) {
        Some(captures) => {
            let min = captures.name("min").unwrap().as_str().parse::<usize>().unwrap();
            let max = captures.name("max").unwrap().as_str().parse::<usize>().unwrap();
            let requirement = captures.name("requirement").unwrap().as_str().chars().next().unwrap();
            let test_string = captures.name("test_string").unwrap().as_str().to_string();
            return (Policy { min: min, max: max, character: requirement}, test_string)
        },
        None => panic!("Bad line: {}", line)
    }
}

pub fn test_policy_one(p: &Policy, s: &String) -> bool {
    let count = s.chars().filter(|x| *x == p.character).count();
    count >= p.min && count <= p.max
}

pub fn test_policy_two(p: &Policy, s: &String) -> bool {
    let at_min = s.chars().nth(p.min-1).unwrap();
    let at_max = s.chars().nth(p.max-1).unwrap();
    (at_min == p.character) ^ (at_max == p.character)
}


#[cfg(test)]
mod tests {
    use mockito;
    use reqwest::{ Client };
    use super::*;

    #[tokio::test]
    async fn policy_two_works() -> Result<(), Error>  {
        let _m = mockito::mock("GET", "/2020/day/2/input")
        .with_status(200)
        .with_body("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"
        ).create();
        assert_eq!(
            get_input(&Client::new(), "token", 2020, 2)
            .await?
            .iter()
            .filter(|t| test_policy_two(&t.0, &t.1))
            .count(),
        1);
        Ok(())
    }


}
