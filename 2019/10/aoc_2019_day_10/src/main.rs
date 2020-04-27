#[macro_use]
extern crate dotenv_codegen;
extern crate reqwest;
extern crate nalgebra as na;

use std::collections::{ HashSet, HashMap };
use reqwest::header::{ COOKIE };
use na::{ DMatrix };
use fraction::{Sign, GenericFraction};
use std::f64::consts::{ PI, FRAC_PI_2 };

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

fn get_all_asteroid_coords(matrix : &DMatrix<char>) -> HashSet<Point> {
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

fn nearly_equal(a: f64, b: f64) -> bool {
	let abs_a = a.abs();
	let abs_b = b.abs();
	let diff = (a - b).abs();

	if a == b { // Handle infinities.
		true
	} else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
		// One of a or b is zero (or both are extremely close to it,) use absolute error.
		diff < (f64::EPSILON * f64::MIN_POSITIVE)
	} else { // Use relative error.
		(diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
	}
}

fn get_true_angle_between_asteroids(a : Point, b : Point) -> f64 {
    let Point(x1, y1) = a;
    let Point(x2, y2) = b;
    let ang = ((y2-y1) as f64).atan2((x2-x1) as f64);
    if ang < 0f64 { 2f64*PI - ang } else { ang }
}

// part one
fn get_best_station(asteroid_field : &DMatrix<char>) -> (Point, usize) {
    get_max_entry(
        get_num_visible_for_all_asteroids(
            get_all_asteroid_coords(
                asteroid_field
            )
        )
    )
}

fn get_distance_between_asteroids(Point(x1,y1) : Point, Point(x2,y2) : Point) -> i32 {
    (x2-x1)*(x2-x1) + (y2-y1)*(y2-y1)
}

fn order_asteroids_by_angle_from_station(station: Point, asteroid_field : &DMatrix<char>) -> Vec<(Point, f64, i32)> {
    let mut asteroids : Vec<(Point, f64, i32)> = get_all_asteroid_coords(asteroid_field).drain()
        .map(|p| (p, get_true_angle_between_asteroids(station, p), get_distance_between_asteroids(station, p)))
        .filter(|(p, _, _)| *p != station)
        .collect();
    asteroids.sort_by(|&(_,a1,_),&(_,a2,_)| a1.partial_cmp(&a2).unwrap());
    asteroids // at this point, cannot guarantee that points along a single ray are ordered by distance
}

fn group_and_sort_by_angle_from_station(asteroids_and_angles : Vec<(Point, f64, i32)>) -> Vec<(Vec<Point>, f64)> {
    let mut groups : Vec<(Vec<Point>, f64)> = Vec::new();
    let mut i = 0;
    while i < asteroids_and_angles.len() {
        let mut group = Vec::new();
        group.push(asteroids_and_angles[i]);
        if i != asteroids_and_angles.len() - 1 {
            while nearly_equal(asteroids_and_angles[i].1, asteroids_and_angles[i+1].1) {
                group.push(asteroids_and_angles[i+1]);
                i += 1
            }
        }
        group.sort_by(|&(_,_,dist1), &(_,_,dist2)| (dist1).cmp(&dist2));
        let ang = group[0].1;
        groups.push((group.iter().map(|(p,_,_)| *p).collect(), ang));
        i += 1
    }
    groups
}

fn get_legit_angle(Point(x1,y1) : Point, Point(x2,y2) : Point) -> f64 {
    match ((y2-y1) as f64).atan2((x2-x1) as f64) + FRAC_PI_2 {
        f if f < 0.0 => f + 2.0*PI,
        f => f,
    }
}

fn zap_asteroids(station : Point, asteroid_field : &DMatrix<char>, stop_after : i64) -> Point {
    let mut groups : Vec<(Vec<Point>, f64)> = group_and_sort_by_angle_from_station(order_asteroids_by_angle_from_station(station, asteroid_field));
    groups.sort_by(|(a,_), (b,_)| get_legit_angle(station,a[0]).partial_cmp(&get_legit_angle(station, b[0])).unwrap());
    let mut group_iterator = 0;
    let mut num_zapped = 0;
    loop {
        if group_iterator == groups.len() {
            group_iterator = 0
        }
        if groups[group_iterator].0.len() > 0 {
            num_zapped += 1;
            if num_zapped == stop_after {
                return groups[group_iterator].0[0]
            }
            groups[group_iterator].0.remove(0);
        }
        group_iterator += 1;
    }
}

fn main() {
    match get_input("https://adventofcode.com/2019/day/10/input") {
        Ok(asteroid_field) => {
            let (station, num_visible) = get_best_station(&asteroid_field);
            println!("Part 1: {:?} => {}", station, num_visible);
            let Point(x,y) = zap_asteroids(station, &asteroid_field, 200);
            println!("Part 2: {:?} => {}", Point(x,y), x*100 + y);
        }
        Err(_) => {
            println!("Error")
        }
    }
}