use reqwest::{ Error, header::{ COOKIE } };
use std::collections::{ HashSet };

#[cfg(test)]
use mockito;

pub async fn get_input(
    client: &reqwest::Client,
    token: &str,
    year: i64,
    day: i64
) -> Result<Vec<i64>, Error> {
    #[cfg(not(test))]
    let base_url = "https://adventofcode.com";

    #[cfg(test)]
    let base_url = &mockito::server_url();
    
    let res = client.get(&format!("{}/{}/day/{}/input", base_url, year, day))
        .header(COOKIE, ["session", token].join("="))
        .send().await?
        .text().await?;

    Ok(res.trim().split("\n").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>())
}

pub fn find_pair_for_sum(nums: Vec<i64>, sum: i64) -> Vec<i64> {
    let mut m = HashSet::new();
    for &n in nums.iter() {
        match m.get(&(sum - n)) {
            Some(found) => return vec![n, *found],
            None => {
                m.insert(n);
            }
        }
    }
    Vec::new()
}

pub fn mult_vec(t: Vec<i64>) -> i64 {
    t.iter().fold(1, |x,y| x*y)
}

pub fn find_triple_for_sum(nums: Vec<i64>, sum: i64) -> Vec<i64> {
    for (i, n) in nums.iter().enumerate() {
        let cur_sum = sum - n;
        let maybe = find_pair_for_sum(nums[i..].to_vec(),cur_sum);
        if maybe.len() > 0 {
            return [&maybe[..], &[*n][..]].concat();
        }
    }
    Vec::new()
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
        .with_body("1
2
3
"
        ).create();
        assert_eq!(get_input(&Client::new(), "token", 2020, 1).await?, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_find_pair() {
        assert_eq!(find_pair_for_sum(vec![1, 2, 17, 2, 5, -5, 2, 4], 12), vec![-5, 17]);
    }

    #[test]
    fn test_find_pair_empty_when_no_sum() {
        assert_eq!(find_pair_for_sum(vec![1, 2, 17, 2, 5, -5, 2, 4], 11), Vec::<i64>::new());
    }

    #[test]
    fn test_find_triple() {
        assert_eq!(find_triple_for_sum(vec![1, 2, 17, 2, 5, -5, 2, 4], 14), vec![-5, 17, 2]);
    }

    #[tokio::test]
    async fn test_find_triple_bigger() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/1/input")
        .with_status(200)
        .with_body("1721
979
366
299
675
1456
"
        ).create();
        assert_eq!(find_triple_for_sum(get_input(&Client::new(), "token", 2020, 1).await?, 2020), vec![675, 366, 979]);
        Ok(())
    }


    #[test]
    fn test_mult_vec() {
        assert_eq!(mult_vec(vec![10, 20]), 200);
    }

}
