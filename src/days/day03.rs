use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::IResult;
use nom::character::complete::{digit1, one_of};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum Direction {
    U(u64),
    D(u64),
    L(u64),
    R(u64),
}

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    d: u64,
}

type Input = (Vec<Direction>, Vec<Direction>);

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("inputs/day03.txt").unwrap().trim().to_owned();
    
    let input = match parse(&input_string) {
        Ok(("", input)) => input,
        _ => panic!("Failed to parse input"),
    };
    // Your solution here...
    let (sol1, sol2) = part(&input.0, &input.1);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part(wire1: &[Direction], wire2: &[Direction]) -> (u64, u64) {
    let mut wire1_points = Vec::new();

    let mut current_point = Point { x: 0, y: 0, d: 0 };
    wire1_points.push(current_point.clone());
    for direction in wire1 {
        move_point(&mut current_point, direction);
        wire1_points.push(current_point.clone());
    }

    let mut wire2_points = Vec::new();

    let mut current_point = Point { x: 0, y: 0, d: 0 };
    wire2_points.push(current_point.clone());
    for direction in wire2 {
        move_point(&mut current_point, direction);
        wire2_points.push(current_point.clone());
    }

    let mut intersections = Vec::new();

    for i in 0..wire1_points.len() - 1 {
        let p1 = (&wire1_points[i], &wire1_points[i + 1]);
        for j in 0..wire2_points.len() - 1 {
            let p2 = (&wire2_points[j], &wire2_points[j + 1]);

            if let Some(intersection) = find_intersection(p1, p2) {
                if intersection.x == 0 && intersection.y == 0 {
                    continue;
                }
                intersections.push(intersection);
            }
        }
    }

    (intersections
        .iter()
        .map(|point| point.x.unsigned_abs() + point.y.unsigned_abs())
        .min()
        .unwrap(),
    intersections
        .iter()
        .map(|point| point.d)
        .min()
        .unwrap())

}

fn find_intersection(p1: (&Point, &Point), p2: (&Point, &Point)) -> Option<Point> {
    // check if p1 is horizontal and p2 is vertical or viceversa (and make them p1 horizontal and p2 vertical if not already so)
    let (p1, p2) = if p1.0.x == p1.1.x && p2.0.y == p2.1.y {
        (p1, p2)
    } else if p2.0.x == p2.1.x && p1.0.y == p1.1.y {
        (p2, p1)
    } else {
        return None;
    };

    // Now check if it is in the range of the other line
    if (p1.0.y <= p2.0.y && p1.1.y >= p2.0.y || p1.0.y >= p2.0.y && p1.1.y <= p2.0.y) && (p2.0.x <= p1.0.x && p2.1.x >= p1.0.x || p2.0.x >= p1.0.x && p2.1.x <= p1.0.x) {
        Some(Point {
            x: p2.0.y,
            y: p1.0.x,
            d: p1.0.d + p2.0.d + (p2.0.y - p1.0.y).unsigned_abs() + (p1.0.x - p2.0.x).unsigned_abs(),
        })
    } else {
        None
    }
}

fn move_point(point: &mut Point, direction: &Direction) {
    match direction {
        Direction::U(distance) => {
            point.y += *distance as i64;
            point.d += distance;
        },
        Direction::D(distance) => {
            point.y -= *distance as i64;
            point.d += distance;
        },
        Direction::L(distance) => {
            point.x -= *distance as i64;
            point.d += distance;
        },
        Direction::R(distance) => {
            point.x += *distance as i64;
            point.d += distance;
        },
    }
}


// Parsing the Input
fn parse(input: &str) -> IResult<&str, Input> {
    let (input, wire1) = parse_wire(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, wire2) = parse_wire(input)?;

    Ok((input, (wire1, wire2)))
}

fn parse_wire(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = separated_list1(tag(","), parse_direction)(input)?;

    Ok((input, directions))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = one_of("UDLR")(input)?;
    let (input, distance) = digit1(input)?;

    let distance = distance.parse().unwrap();

    let direction = match direction {
        'U' => Direction::U(distance),
        'D' => Direction::D(distance),
        'L' => Direction::L(distance),
        'R' => Direction::R(distance),
        _ => unreachable!(),
    };

    Ok((input, direction))
}