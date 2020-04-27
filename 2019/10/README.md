## AOC 2019 Day 10

## Prereqs

### Environment
Create `.env` file with:

```
session=<aoc session cookie from browser>
```

### Dependencies

```bash
brew install rustup
rustup-init
```

## Run

```bash
cd aoc_2019_day_10
cargo run
```

## TODO
- Learn more about creating crates in rust
- Separate functions into different files
- Learn about Rust unit testing
- Start creating structs and implementing traits ASAP
    - Ordering would have helped a lot to order the asteroid field

## Notes
- I was having trouble with arctan and angles at first, which is why there are three distinct angle functions:
    - `get_angle_between_asteroids(asteroid : Point, other_asteroid : Point) -> (u32, GenericFraction<i32>)`
    - `get_true_angle_between_asteroids(a : Point, b : Point) -> f64`
    - `get_legit_angle(Point(x1,y1) : Point, Point(x2,y2) : Point) -> f64`

    This is the greatest shame of my programming career.

