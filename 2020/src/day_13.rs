use recap::Recap;
use reqwest::{Client, Error};
use serde::Deserialize;
use sxd_document::parser;
use sxd_xpath::evaluate_xpath;

#[cfg(not(test))]
use dotenv;
#[cfg(not(test))]
use std::env;

use crate::common;

#[cfg(not(test))]
const WOLFRAM_APP_ID: &str = "wolfram_app_id";

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"t = \d+ n \+ (?P<value>\d+), n element Z"#)]
struct WolframIntegerSolution {
    value: u64,
}

#[derive(Debug, PartialEq)]
struct BusSchedule {
    departure_timestamp: u64,
    bus_ids: Vec<u64>,
}

fn parse_schedule(input: &str) -> BusSchedule {
    let mut parts = input.trim().split('\n');
    BusSchedule {
        departure_timestamp: parts.next().unwrap().parse::<u64>().unwrap(),
        bus_ids: parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| {
                if s == "x" {
                    0
                } else {
                    s.parse::<u64>().unwrap()
                }
            })
            .collect(),
    }
}

fn find_earliest_bus_and_departure_time(schedule: &BusSchedule) -> (u64, u64) {
    let bus_and_difference = schedule
        .bus_ids
        .iter()
        .filter(|id| **id != 0)
        .map(|id| {
            (
                id,
                *id * (schedule.departure_timestamp as f64 / *id as f64).ceil() as u64
                    - schedule.departure_timestamp,
            )
        })
        .min_by_key(|x| x.1)
        .unwrap();
    (
        *bus_and_difference.0,
        schedule.departure_timestamp + bus_and_difference.1,
    )
}

fn generate_system_of_equations_from_bus_ids(schedule: &BusSchedule) -> String {
    schedule
        .bus_ids
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != 0)
        .fold("".to_string(), |acc, (i, x)| {
            acc + &format!("(t + {}) mod {} = 0; ", i, x)
        })
}

fn get_integer_solution(wolfram_alpha_xml: &str) -> u64 {
    let package = parser::parse(wolfram_alpha_xml).expect("failed to parse XML");
    let document = package.as_document();
    let value = evaluate_xpath(
        &document,
        "//pod[@id='IntegerSolution' and @primary='true']/subpod/plaintext",
    )
    .expect("unable to evaluate xpath")
    .string()
    .parse::<WolframIntegerSolution>()
    .unwrap()
    .value;
    value
}

async fn query_wolfram_alpha(query: &str) -> Result<String, Error> {
    #[cfg(not(test))]
    dotenv::dotenv().ok();

    #[cfg(not(test))]
    let wolfram_app_id = env::var(WOLFRAM_APP_ID).unwrap();
    #[cfg(test)]
    let wolfram_app_id: String = "token".to_string();

    #[cfg(not(test))]
    let base_url = "http://api.wolframalpha.com/v2";
    #[cfg(test)]
    let base_url = &mockito::server_url();

    Ok(Client::new()
        .get(&format!("{}/query", base_url))
        .query(&[("input", query), ("appid", &wolfram_app_id)])
        .send()
        .await?
        .text()
        .await?)
}

pub async fn solve() -> Result<(), Error> {
    let schedule = parse_schedule(&common::get_input(2020, 13).await?);
    let (earliest_bus_id, earliest_departure) = find_earliest_bus_and_departure_time(&schedule);
    println!(
        "Day 13 Part 1: {:?}",
        earliest_bus_id * (earliest_departure - schedule.departure_timestamp)
    );
    println!(
        "Day 13 Part 2: {:?}",
        get_integer_solution(
            &query_wolfram_alpha(&generate_system_of_equations_from_bus_ids(&schedule)).await?
        )
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use reqwest;
    use tokio;

    const TEST_INPUT: &str = "939
7,13,x,x,59,x,31,19
";

    #[tokio::test]
    async fn test_wolfram_response_parse() -> Result<(), reqwest::Error> {
        let _m = mockito::mock("GET", "/query")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded(
                    "appid".into(),
                    "token".into()
                ),
                mockito::Matcher::UrlEncoded(
                    "input".into(),
                    "(t + 0) mod 7 = 0; (t + 1) mod 13 = 0; (t + 4) mod 59 = 0; (t + 6) mod 31 = 0; (t + 7) mod 19 = 0; ".into()
                )
            ]))
            .with_body(
                "<?xml version='1.0' encoding='UTF-8'?>
                <queryresult success='true'
                    error='false'
                    xml:space='preserve'
                    numpods='3'
                    datatypes=''
                    timedout=''
                    timedoutpods=''
                    timing='4.53'
                    parsetiming='2.269'
                    parsetimedout='false'
                    recalculate=''
                    id='MSP6701884b9822hb45b3f000058i3ha12cd813dcc'
                    host='https://www4d.wolframalpha.com'
                    server='27'
                    related='https://www4d.wolframalpha.com/api/v1/relatedQueries.jsp?id=MSPa6711884b9822hb45b3f000042ddid494a1f84531146275927865144044'
                    version='2.6'
                    inputstring='t mod 7 = 0; (t + 1) mod 13 = 0; (t + 4) mod 59 = 0; (t + 6) mod 31 = 0; (t + 7) mod 19 = 0'
                >
                    <pod title='Input'
                        scanner='Identity'
                        id='Input'
                        position='100'
                        error='false'
                        numsubpods='1'
                    >
                        <subpod title=''>
                            <img src='https://www4d.wolframalpha.com/Calculate/MSP/MSP6721884b9822hb45b3f000022c209gbb0g3c301?MSPStoreType=image/gif&amp;s=27'
                                alt='{t mod 7 = 0, (t + 1) mod 13 = 0, (t + 4) mod 59 = 0, (t + 6) mod 31 = 0, (t + 7) mod 19 = 0}'
                                title='{t mod 7 = 0, (t + 1) mod 13 = 0, (t + 4) mod 59 = 0, (t + 6) mod 31 = 0, (t + 7) mod 19 = 0}'
                                width='393'
                                height='40'
                                type='Default'
                                themes='1,2,3,4,5,6,7,8,9,10,11,12'
                                colorinvertable='true'
                            />
                            <plaintext>{t mod 7 = 0, (t + 1) mod 13 = 0, (t + 4) mod 59 = 0, (t + 6) mod 31 = 0, (t + 7) mod 19 = 0}</plaintext>
                        </subpod>
                        <expressiontypes count='1'><expressiontype name='Default' /></expressiontypes>
                    </pod>
                    <pod title='Alternate form'
                        scanner='Simplification'
                        id='AlternateForm'
                        position='200'
                        error='false'
                        numsubpods='1'
                    >
                        <subpod title=''>
                            <img src='https://www4d.wolframalpha.com/Calculate/MSP/MSP6731884b9822hb45b3f00005589bh247gf572ge?MSPStoreType=image/gif&amp;s=27'
                                alt='{t - 7 floor(t/7) = 0, -13 floor((t + 1)/13) + t + 1 = 0, -59 floor((t + 4)/59) + t + 4 = 0, -31 floor((t + 6)/31) + t + 6 = 0, -19 floor((t + 7)/19) + t + 7 = 0}'
                                title='{t - 7 floor(t/7) = 0, -13 floor((t + 1)/13) + t + 1 = 0, -59 floor((t + 4)/59) + t + 4 = 0, -31 floor((t + 6)/31) + t + 6 = 0, -19 floor((t + 7)/19) + t + 7 = 0}'
                                width='415'
                                height='76'
                                type='Default'
                                themes='1,2,3,4,5,6,7,8,9,10,11,12'
                                colorinvertable='true'
                            />
                            <plaintext>{t - 7 floor(t/7) = 0, -13 floor((t + 1)/13) + t + 1 = 0, -59 floor((t + 4)/59) + t + 4 = 0, -31 floor((t + 6)/31) + t + 6 = 0, -19 floor((t + 7)/19) + t + 7 = 0}</plaintext>
                        </subpod>
                        <expressiontypes count='1'><expressiontype name='Default' /></expressiontypes>
                        <infos count='1'>
                            <info text='floor(x) is the floor function'>
                                <img src='https://www4d.wolframalpha.com/Calculate/MSP/MSP6741884b9822hb45b3f000010di7gb95de4acae?MSPStoreType=image/gif&amp;s=27'
                                    alt='floor(x) is the floor function'
                                    title='floor(x) is the floor function'
                                    width='154'
                                    height='19'
                                />
                                <link url='http://reference.wolfram.com/language/ref/Floor.html'
                                    text='Documentation'
                                    title='Mathematica'
                                />
                                <link url='http://functions.wolfram.com/IntegerFunctions/Floor'
                                    text='Properties'
                                    title='Wolfram Functions Site'
                                />
                                <link url='http://mathworld.wolfram.com/FloorFunction.html'
                                    text='Definition'
                                    title='MathWorld'
                                />
                            </info>
                        </infos>
                    </pod>
                    <pod title='Integer solution'
                        scanner='Reduce'
                        id='IntegerSolution'
                        position='300'
                        error='false'
                        numsubpods='1'
                        primary='true'
                    >
                        <subpod title=''>
                            <img src='https://www4d.wolframalpha.com/Calculate/MSP/MSP6751884b9822hb45b3f00002dc86gf1c032ce7e?MSPStoreType=image/gif&amp;s=27'
                                alt='t = 3162341 n + 1068781, n element Z'
                                title='t = 3162341 n + 1068781, n element Z'
                                width='550'
                                height='22'
                                type='Default'
                                themes='1,2,3,4,5,6,7,8,9,10,11,12'
                                colorinvertable='true'
                            />
                            <plaintext>t = 3162341 n + 1068781, n element Z</plaintext>
                        </subpod>
                        <expressiontypes count='1'><expressiontype name='Default' /></expressiontypes>
                        <infos count='1'>
                            <info text='Z is the set of integers'>
                                <img src='https://www4d.wolframalpha.com/Calculate/MSP/MSP6761884b9822hb45b3f00001abe2ge0c2bf302h?MSPStoreType=image/gif&amp;s=27'
                                    alt='Z is the set of integers'
                                    title='Z is the set of integers'
                                    width='145'
                                    height='19'
                                />
                                <link url='http://reference.wolfram.com/language/ref/Integers.html'
                                    text='Documentation'
                                    title='Documentation'
                                />
                                <link url='http://mathworld.wolfram.com/Z.html'
                                    text='Definition'
                                    title='MathWorld'
                                />
                            </info>
                        </infos>
                    </pod>
                </queryresult>"
            )
            .create();
        let integer_solution = get_integer_solution(
            &query_wolfram_alpha(
                "(t + 0) mod 7 = 0; (t + 1) mod 13 = 0; (t + 4) mod 59 = 0; (t + 6) mod 31 = 0; (t + 7) mod 19 = 0; "
            ).await?
        );
        assert_eq!(integer_solution, 1068781);
        Ok(())
    }

    #[test]
    fn test_generate_system_of_equations_from_bus_ids() {
        assert_eq!(
            generate_system_of_equations_from_bus_ids(
                &BusSchedule {
                    departure_timestamp: 939,
                    bus_ids: vec!(7, 13, 0, 0, 59, 0, 31, 19)
                }
            ),
            "(t + 0) mod 7 = 0; (t + 1) mod 13 = 0; (t + 4) mod 59 = 0; (t + 6) mod 31 = 0; (t + 7) mod 19 = 0; "
        )
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_schedule(TEST_INPUT),
            BusSchedule {
                departure_timestamp: 939,
                bus_ids: vec!(7, 13, 0, 0, 59, 0, 31, 19)
            }
        )
    }

    #[test]
    fn test_find_earliest_bus() {
        let schedule = parse_schedule(TEST_INPUT);
        assert_eq!(find_earliest_bus_and_departure_time(&schedule).0, 59);
    }
}
