use crate::common;

use itertools::Itertools;
use reqwest::Error;

#[derive(Debug,Clone,PartialEq,Eq,Hash,Copy)]
struct Position(i64, i64);

#[derive(Debug,PartialEq)]
struct Motion(char, i64);

type BridgeState = Vec<Vec<Position>>;

fn new_bridge(knots: usize) -> BridgeState {
    (0..knots).map(|_| vec![Position(0,0)]).collect()
}

fn parse_motions(input: &str) -> impl Iterator<Item=Motion> + '_ {
    input.trim().split('\n').map(|l| {
        let mut parts = l.split_whitespace();
        Motion(
            parts.next().unwrap().chars().next().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        )
    })
}

fn touching(x: &Position, y: &Position) -> bool {
    (x.0 - y.0).abs() <= 1 && (x.1 - y.1).abs() <= 1
}

fn apply_motion(p: &Position, m: &Motion) -> Position {
    match m.0 {
        'U' => Position(p.0, p.1 + m.1),
        'D' => Position(p.0, p.1 - m.1),
        'L' => Position(p.0 - m.1, p.1),
        'R' => Position(p.0 + m.1, p.1),
        _ => panic!("unknown direction in motion: {}", m.0)
    }
}

fn update_bridge<'a>(s: &'a mut BridgeState, m: &Motion) -> &'a mut BridgeState {
    let mut head = s[0].last().unwrap().clone();
    for _ in 0..m.1 {
        head = apply_motion(&head, &Motion(m.0,1));
        s[0].push(head);
        for i in 1..s.len() {
            let cur_head = s[i-1].last().unwrap().clone();
            let cur_next = s[i].last().unwrap().clone();
            s[i].push(match(cur_head,cur_next) {
                (h,t) if touching(&h,&t) => t,
                (h,t) if h.0 - t.0 == 0 && h.1 - t.1 > 0 => apply_motion(&t, &Motion('U',1)),
                (h,t) if h.0 - t.0 == 0 && h.1 - t.1 < 0 => apply_motion(&t, &Motion('D',1)),
                (h,t) if h.1 - t.1 == 0 && h.0 - t.0 > 0 => apply_motion(&t, &Motion('R',1)),
                (h,t) if h.1 - t.1 == 0 && h.0 - t.0 < 0 => apply_motion(&t, &Motion('L',1)),
                (h,t) if h.0 - t.0 > 0 && h.1 - t.1 > 0 => apply_motion(&apply_motion(&t, &Motion('R',1)), &Motion('U',1)),
                (h,t) if h.0 - t.0 > 0 && h.1 - t.1 < 0 => apply_motion(&apply_motion(&t, &Motion('R',1)), &Motion('D',1)),
                (h,t) if h.0 - t.0 < 0 && h.1 - t.1 > 0 => apply_motion(&apply_motion(&t, &Motion('L',1)), &Motion('U',1)),
                (h,t) if h.0 - t.0 < 0 && h.1 - t.1 < 0 => apply_motion(&apply_motion(&t, &Motion('L',1)), &Motion('D',1)),
                _ => panic!("invalid bridge state")
            });
        }
    }
    s
}

fn simulate_rope<'a>(initial_bridge_state: &'a mut BridgeState, motions: impl Iterator<Item=Motion>) -> &'a mut BridgeState {
    motions.fold(initial_bridge_state, |current_bridge_state, motion| {
        update_bridge(current_bridge_state, &motion)
    })
}


pub async fn solve() -> Result<(), Error> {
    let input = common::get_input(2022, 9).await?;

    let mut bridge = new_bridge(2);
    let mut output = simulate_rope(&mut bridge, parse_motions(&input));
    println!("Day 09 Part 1: {:?}", output.last().unwrap().iter().unique().count());

    bridge = new_bridge(10);
    output = simulate_rope(&mut bridge, parse_motions(&input));
    println!("Day 09 Part 2: {:?}", output.last().unwrap().iter().unique().count());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use plotlib::repr::Plot;
    use plotlib::page::Page;
    use plotlib::view::ContinuousView;
    use plotlib::style::{PointMarker, PointStyle};

    const TEST_INPUT: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const TEST_INPUT_LARGE: &str = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    fn plot(points: Vec<Position>, fname: &str) {
        let s1: Plot = Plot::new(points.iter().map(|p|(p.0 as f64,p.1 as f64)).collect::<Vec<(f64,f64)>>()).point_style(
            PointStyle::new()
                .marker(PointMarker::Circle)
                .colour("#DD3355"),
        );

        let s2: Plot = Plot::new(vec![(points[0].0 as f64, points[0].1 as f64)]).point_style(
            PointStyle::new().marker(PointMarker::Cross).colour("#DFDFDF")
        );

        let v = ContinuousView::new()
            .add(s1)
            .add(s2)
            .x_range(-20., 20.)
            .y_range(-20., 20.)
            .x_label("x")
            .y_label("y");

        // A page with a single view is then saved to an SVG file
        Page::single(&v).save(format!("assets/{}.svg", fname)).unwrap();
    }

    #[test]
    fn test_parse_motions() {
        assert_eq!(
            parse_motions(TEST_INPUT).collect::<Vec<Motion>>(),
            vec![
                Motion('R', 4),
                Motion('U', 4),
                Motion('L', 3),
                Motion('D', 1),
                Motion('R', 4),
                Motion('D', 1),
                Motion('L', 5),
                Motion('R', 2),
            ]
        )
    }

    #[test]
    fn test_simulate_rope_2() {
        let mut initial_bridge = new_bridge(2);
        let bridge = simulate_rope(&mut initial_bridge, parse_motions(TEST_INPUT));
        assert_eq!(*bridge.first().unwrap().last().unwrap(), Position(2,2));
        assert_eq!(*bridge.last().unwrap().last().unwrap(), Position(1,2));
        assert_eq!(bridge.last().unwrap().iter().unique().count(), 13);
    }

    #[test]
    fn test_simulate_rope_10_small() {
        let mut initial_bridge = new_bridge(10);
        let bridge = simulate_rope(&mut initial_bridge, parse_motions(TEST_INPUT));
        assert_eq!(*bridge.first().unwrap().last().unwrap(), Position(2,2));
        assert_eq!(*bridge.last().unwrap().last().unwrap(), Position(0,0));
        assert_eq!(bridge.last().unwrap().iter().unique().count(), 1);
    }

    #[test]
    fn test_simulate_rope_10_large() {
        let mut initial_bridge = new_bridge(10);
        let bridge = simulate_rope(&mut initial_bridge, parse_motions(TEST_INPUT_LARGE));
        assert_eq!(*bridge.first().unwrap().last().unwrap(), Position(-11,15));
        assert_eq!(*bridge.last().unwrap().last().unwrap(), Position(-11,6));
        assert_eq!(bridge.last().unwrap().iter().unique().count(), 36);
        plot(bridge.last().unwrap().clone(), "2022-09-knots-10-large")
    }

}
