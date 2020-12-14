use crate::common;
use plotters::prelude::*;

// seats
const FLOOR: char = '.';
const FLOOR_COLOR: &RGBColor = &GREEN;

const EMPTY_SEAT: char = 'L';
const EMPTY_SEAT_COLOR: &RGBColor = &BLUE;

const OCCUPIED_SEAT: char = '#';
const OCCUPIED_SEAT_COLOR: &RGBColor = &RED;

// render configurations
const FONT: &str = "monaco";
const RENDER_FRAME_DELAY: u32 = 200;
const RENDER_OUTPUT_DIR: &str = "artifacts/day_11";
const CANVAS_SIZE: u32 = 500;

// vectors for grid walking
const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split('\n')
        .map(|l| l.chars().collect())
        .collect()
}

fn count_all_occupied_adjacent_seats(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> usize {
    DIRS.iter()
        .filter_map(|(x, y)| grid.get((pos.0 + x) as usize)?.get((pos.1 + y) as usize))
        .filter(|s| *s == &OCCUPIED_SEAT)
        .count()
}

fn is_out_of_bounds(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= (grid.len() as i32)
        || pos.0 < 0
        || pos.1 >= (grid.get(0).unwrap().len() as i32)
        || pos.1 < 0
}

fn count_all_occupied_visible_seats(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> usize {
    DIRS.iter()
        .filter_map(|(x, y)| {
            let mut cur_pos = (pos.0 + x, pos.1 + y);
            loop {
                if is_out_of_bounds(grid, cur_pos) {
                    return None;
                };
                let seat = grid.get(cur_pos.0 as usize)?.get(cur_pos.1 as usize)?;
                if *seat != FLOOR {
                    return Some(seat);
                };
                cur_pos = (cur_pos.0 + x, cur_pos.1 + y);
            }
        })
        .filter(|s| *s == &OCCUPIED_SEAT)
        .count()
}

fn apply_rules(
    grid: &Vec<Vec<char>>,
    visibility_func: fn(&Vec<Vec<char>>, (i32, i32)) -> usize,
    minimum_yielding_move: usize,
) -> Vec<Vec<char>> {
    let mut output_grid = grid.clone();
    for (i, row) in grid.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            match (*seat, visibility_func(grid, (i as i32, j as i32))) {
                (EMPTY_SEAT, 0) => {
                    output_grid[i][j] = OCCUPIED_SEAT;
                }
                (OCCUPIED_SEAT, n) if n >= minimum_yielding_move => {
                    output_grid[i][j] = EMPTY_SEAT;
                }
                _ => {
                    output_grid[i][j] = *seat;
                }
            }
        }
    }
    output_grid
}

fn evolve_grid(
    grid: &Vec<Vec<char>>,
    evolver: fn(&Vec<Vec<char>>) -> Vec<Vec<char>>,
    drawing_area: &DrawingArea<BitMapBackend, plotters::coord::Shift>,
) -> Vec<Vec<char>> {
    let mut prev = grid.clone();
    loop {
        render(&prev, drawing_area).unwrap();
        let next = evolver(&prev);
        if next == prev {
            return next;
        } else {
            prev = next;
        }
    }
}

pub async fn solve() -> Result<(), reqwest::Error> {
    let grid = parse_input_to_matrix(&common::get_input(2020, 11).await?);
    let part_one_evolved = evolve_grid(
        &grid,
        |x| apply_rules(x, count_all_occupied_adjacent_seats, 4),
        &BitMapBackend::gif(
            format!("{}/day_11_part_1.gif", RENDER_OUTPUT_DIR),
            (CANVAS_SIZE, CANVAS_SIZE),
            RENDER_FRAME_DELAY,
        )
        .unwrap()
        .into_drawing_area(),
    );
    println!(
        "Day 11 Part 1: {:?}",
        part_one_evolved
            .iter()
            .map(|r| r.iter().filter(|s| *s == &OCCUPIED_SEAT).count())
            .sum::<usize>()
    );

    let part_two_evolved = evolve_grid(
        &grid,
        |x| apply_rules(x, count_all_occupied_visible_seats, 5),
        &BitMapBackend::gif(
            format!("{}/day_11_part_2.gif", RENDER_OUTPUT_DIR),
            (CANVAS_SIZE, CANVAS_SIZE),
            RENDER_FRAME_DELAY,
        )
        .unwrap()
        .into_drawing_area(),
    );
    println!(
        "Day 11 Part 2: {:?}",
        part_two_evolved
            .iter()
            .map(|r| r.iter().filter(|s| *s == &OCCUPIED_SEAT).count())
            .sum::<usize>()
    );
    Ok(())
}

fn render(
    grid: &Vec<Vec<char>>,
    drawing_area: &DrawingArea<BitMapBackend, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
    drawing_area.fill(&WHITE).ok();

    let grid_height = grid.len();
    let grid_width = grid.get(0).unwrap().len();

    let font_size: i32 = (CANVAS_SIZE as i32) / (grid_width as i32);

    let child_drawing_areas = drawing_area.split_evenly((grid_height, grid_width));

    for (render_index, area) in child_drawing_areas.iter().enumerate() {
        let i = render_index % grid_width;
        let j = render_index / grid_width;
        let seat = grid.get(j).unwrap().get(i).unwrap();
        area.draw_text(
            &format!("{}", *seat),
            &TextStyle::from((FONT, font_size).into_font()).color(match *seat {
                FLOOR => FLOOR_COLOR,
                OCCUPIED_SEAT => OCCUPIED_SEAT_COLOR,
                EMPTY_SEAT => EMPTY_SEAT_COLOR,
                _ => panic!("Unexpected character in input: {}", *seat),
            }),
            (0, 0),
        )
        .ok();
    }
    drawing_area.present()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn test_parse_input_to_matrix() {
        assert_eq!(
            parse_input_to_matrix(TEST_INPUT),
            vec!(
                "L.LL.LL.LL".chars().collect::<Vec<char>>(),
                "LLLLLLL.LL".chars().collect::<Vec<char>>(),
                "L.L.L..L..".chars().collect::<Vec<char>>(),
                "LLLL.LL.LL".chars().collect::<Vec<char>>(),
                "L.LL.LL.LL".chars().collect::<Vec<char>>(),
                "L.LLLLL.LL".chars().collect::<Vec<char>>(),
                "..L.L.....".chars().collect::<Vec<char>>(),
                "LLLLLLLLLL".chars().collect::<Vec<char>>(),
                "L.LLLLLL.L".chars().collect::<Vec<char>>(),
                "L.LLLLL.LL".chars().collect::<Vec<char>>()
            )
        )
    }

    #[test]
    fn test_evolution() {
        let evolved = evolve_grid(
            &parse_input_to_matrix(TEST_INPUT),
            |x| apply_rules(x, count_all_occupied_adjacent_seats, 4),
            &BitMapBackend::gif(
                "artifacts/day_11/day_11_part_1_test.gif",
                (CANVAS_SIZE, CANVAS_SIZE),
                RENDER_FRAME_DELAY,
            )
            .unwrap()
            .into_drawing_area(),
        );
        let occupied = evolved
            .iter()
            .map(|row| row.iter().filter(|s| *s == &OCCUPIED_SEAT).count())
            .sum::<usize>();
        assert_eq!(occupied, 37);
    }

    #[test]
    fn test_get_all_adjacent_seats() {
        assert_eq!(
            count_all_occupied_adjacent_seats(&parse_input_to_matrix(TEST_INPUT), (0, 0)),
            0
        );
        assert_eq!(
            count_all_occupied_adjacent_seats(&parse_input_to_matrix(TEST_INPUT), (8, 4)),
            0
        );
    }

    #[test]
    fn test_count_all_occupied_visible_seats() {
        let evolved = evolve_grid(
            &parse_input_to_matrix(TEST_INPUT),
            |x| apply_rules(x, count_all_occupied_visible_seats, 5),
            &BitMapBackend::gif(
                "artifacts/day_11/day_11_part_2_test.gif",
                (CANVAS_SIZE, CANVAS_SIZE),
                RENDER_FRAME_DELAY,
            )
            .unwrap()
            .into_drawing_area(),
        );
        let occupied = evolved
            .iter()
            .map(|row| row.iter().filter(|s| *s == &OCCUPIED_SEAT).count())
            .sum::<usize>();
        assert_eq!(occupied, 26);
    }
}
