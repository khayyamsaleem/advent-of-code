use crate::common;
use regex::Regex;
use reqwest::Error;
use std::collections::HashMap;

fn parse_passports(input: String) -> Vec<HashMap<String, String>> {
    input
        .trim()
        .split("\n\n")
        .map(|l| {
            l.replace("\n", " ")
                .split(" ")
                .fold(hashmap! {}, |mut acc, x| {
                    let v: Vec<String> = x.split(":").map(|s| s.to_string()).collect();
                    acc.insert(v[0].to_string(), v[1].to_string());
                    acc
                })
        })
        .collect()
}

fn is_valid_passport_basic(
    passport: &HashMap<String, String>,
    required_fields: &HashMap<&str, fn(&str) -> bool>,
    optional_fields: &HashMap<&str, fn(&str) -> bool>,
) -> bool {
    let only_required = passport
        .keys()
        .filter(|k| !optional_fields.contains_key(k.as_str()));
    return only_required.count() == required_fields.len();
}

fn is_valid_passport_advanced(
    passport: &HashMap<String, String>,
    required_fields: &HashMap<&str, fn(&str) -> bool>,
    optional_fields: &HashMap<&str, fn(&str) -> bool>,
) -> bool {
    let only_required: Vec<&String> = passport
        .keys()
        .filter(|k| !optional_fields.contains_key(k.as_str()))
        .collect();

    if only_required.len() != required_fields.len() {
        return false;
    };

    only_required
        .iter()
        .all(|k| required_fields.get(k.as_str()).unwrap()(passport.get(k.as_str()).unwrap()))
}

fn get_field_rules() -> (
    HashMap<&'static str, fn(&str) -> bool>,
    HashMap<&'static str, fn(&str) -> bool>,
) {
    let mut required_fields: HashMap<&str, fn(&str) -> bool> = HashMap::new();
    required_fields.insert("byr", |byr| {
        let num = byr.parse::<i64>().unwrap();
        byr.len() == 4 && num >= 1920 && num <= 2002
    });
    required_fields.insert("iyr", |iyr| {
        let num = iyr.parse::<i64>().unwrap();
        iyr.len() == 4 && num >= 2010 && num <= 2020
    });
    required_fields.insert("eyr", |eyr| {
        let num = eyr.parse::<i64>().unwrap();
        eyr.len() == 4 && num >= 2020 && num <= 2030
    });
    required_fields.insert("hgt", |hgt| {
        if !Regex::new(r"^[0-9]+(cm|in)$").unwrap().is_match(hgt) {
            return false;
        };
        let unit: String = hgt[(hgt.len() - 2)..].to_string();
        let num = hgt[..hgt.len() - 2].parse::<i64>().unwrap();
        match unit.as_str() {
            "cm" => num >= 150 && num <= 193,
            "in" => num >= 59 && num <= 76,
            _ => panic!("No such unit: {}!", unit),
        }
    });
    required_fields.insert("hcl", |hcl| {
        Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(hcl)
    });
    required_fields.insert("ecl", |ecl| {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|clr| *clr == ecl)
    });
    required_fields.insert("pid", |pid| {
        Regex::new(r"^[0-9]{9}$").unwrap().is_match(pid)
    });

    let mut optional_fields: HashMap<&str, fn(&str) -> bool> = HashMap::new();
    optional_fields.insert("cid", |_cid| true);

    (required_fields, optional_fields)
}

pub async fn solve() -> Result<(), Error> {
    let (required_fields, optional_fields) = get_field_rules();

    println!(
        "Day 04 Part 1: {:?}",
        parse_passports(common::get_input(2020, 4).await?)
            .iter()
            .filter(|p| is_valid_passport_basic(p, &required_fields, &optional_fields))
            .count()
    );
    println!(
        "Day 04 Part 2: {:?}",
        parse_passports(common::get_input(2020, 4).await?)
            .iter()
            .filter(|p| is_valid_passport_advanced(p, &required_fields, &optional_fields))
            .count()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;
    use mockito;
    use reqwest::Error;
    use tokio;

    #[tokio::test]
    async fn test_parse_passport() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/4/input")
            .with_status(200)
            .with_body(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
",
            )
            .create();

        assert_eq!(
            parse_passports(common::get_input(2020, 4).await?)[0],
            convert_args!(
                keys = String::from,
                values = String::from,
                hashmap!(
                    "ecl" => "gry",
                    "eyr" => "2020",
                    "pid" => "860033327",
                    "hcl" => "#fffffd",
                    "byr" => "1937",
                    "iyr" => "2017",
                    "cid" => "147",
                    "hgt" => "183cm",
                )
            )
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_is_valid_passport_basic() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/4/input")
            .with_status(200)
            .with_body(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
",
            )
            .create();
        let parsed_input = parse_passports(common::get_input(2020, 4).await?);
        let (required_fields, optional_fields) = get_field_rules();
        assert_eq!(
            is_valid_passport_basic(&parsed_input[0], &required_fields, &optional_fields),
            true
        );
        assert_eq!(
            is_valid_passport_basic(&parsed_input[1], &required_fields, &optional_fields),
            false
        );
        assert_eq!(
            is_valid_passport_basic(&parsed_input[2], &required_fields, &optional_fields),
            true
        );
        assert_eq!(
            is_valid_passport_basic(&parsed_input[3], &required_fields, &optional_fields),
            false
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_is_valid_passport_advanced_invalid_input() -> Result<(), Error> {
        let _m = mockito::mock("GET", "/2020/day/4/input")
            .with_status(200)
            .with_body(
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
",
            )
            .create();

        let parsed_input = parse_passports(common::get_input(2020, 4).await?);
        let (required_fields, optional_fields) = get_field_rules();
        assert!(parsed_input.iter().all(|p| !is_valid_passport_advanced(
            p,
            &required_fields,
            &optional_fields
        )));
        Ok(())
    }
}
