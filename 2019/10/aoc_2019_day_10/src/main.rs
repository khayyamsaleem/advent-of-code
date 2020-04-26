#[macro_use]
extern crate dotenv_codegen;
extern crate reqwest;
extern crate nalgebra as na;

use std::collections::{ HashSet, HashMap };
use reqwest::header::{ COOKIE };
use na::{ DMatrix };
use fraction::{Sign, GenericFraction};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self(x,y)
    }
}

fn input_to_matrix(s : &str) -> DMatrix<char> {
    let input_as_vec : Vec<char>= s.chars().collect();
    let width = input_as_vec.iter().position(|x| x.is_ascii_whitespace()).unwrap();
    let height = s.lines().collect::<Vec<&str>>().len();
    DMatrix::from_vec(width, height, s.trim().chars().filter(|x| !x.is_ascii_whitespace()).collect::<Vec<char>>())
}

fn get_input(uri : &str) -> Result<DMatrix<char>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let res = client.get(uri)
        .header(COOKIE, ["session", dotenv!("session")].join("="))
        .send()?
        .text()?;
    Ok(input_to_matrix(&res))
}

fn get_all_asteroid_coords(matrix : DMatrix<char>) -> HashSet<Point> {
    let mut asteroids = HashSet::new();
    for x in 0..matrix.ncols() {
        for y in 0..matrix.nrows() {
            match matrix.get((x,y)) {
                Some('#') => { asteroids.insert(Point::new(x as i32,y as i32)); }
                _   => { }
            }
        }
    }
    asteroids
}

// 'quad' added as a last second thing because i forgot about the fact that a/b == -a/-b 
// but they're not the same in terms of directions
fn get_angle_between_asteroids(asteroid : Point, other_asteroid : Point) -> (u32, GenericFraction<i32>) {
    let (Point(x1, y1), Point(x2, y2)) = (asteroid, other_asteroid);
    let (sign, quad) = match ((y2 - y1) > 0, (x2 - x1) > 0) {
        (true, true) => { (Sign::Plus, 0) }
        (false, true) => { (Sign::Minus, 1) }
        (false, false) => { (Sign::Plus, 2) }
        _ => { (Sign::Minus, 3) }
    };
    (quad, GenericFraction::<i32>::new_generic(sign, y2 - y1, x2 - x1).unwrap())
}

fn get_num_visible_for_asteroid(asteroid : Point, asteroids : HashSet<Point>) -> usize {
    asteroids.iter()
        .map(|other_asteroid| get_angle_between_asteroids(asteroid, *other_asteroid))
        .collect::<HashSet<(u32, GenericFraction<i32>)>>()
        .len() - 1 // skipping the current asteroid
}

fn get_num_visible_for_all_asteroids(asteroids : HashSet<Point>) -> HashMap<Point,usize> {
    let mut visible_counts = HashMap::new();
    for asteroid in asteroids.iter() {
        visible_counts.insert(*asteroid, get_num_visible_for_asteroid(*asteroid, asteroids.clone()));
    }
    visible_counts
}

fn get_max_entry(visible_counts : HashMap<Point, usize>) -> (Point, usize) {
    let mut max_entry = (Point::new(0 as i32,0 as i32), 0 as usize);
    for (asteroid, num_visible) in &visible_counts {
        let (_, cur_max) = max_entry;
        if *num_visible >= cur_max {
            max_entry = (*asteroid, *num_visible)
        }
    }
    max_entry
}

// part one
fn get_best_station(asteroid_field : DMatrix<char>) -> (Point, usize) {
    get_max_entry(
        get_num_visible_for_all_asteroids(
            get_all_asteroid_coords(
                asteroid_field
            )
        )
    )
}

fn main() {
    match get_input("https://adventofcode.com/2019/day/10/input") {
        Ok(asteroid_field) => {
            println!("{:#?}", get_best_station(asteroid_field))
        }
        Err(_) => {
            println!("Error")
        }
    }
}