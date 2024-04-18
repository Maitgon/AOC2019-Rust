# Day 03 : Crossed Wires

* Difficulty: ⭐⭐

## Description

This day's problem consisted in checking the intersections of 2 wires in a grid. In the first part we need to find the intersection closest to the origin, and in the second part we need to find the intersection with the smallest number of steps of both wires combined.

## Problem solution

I decided to learn nom this year so I parsed the input with it. The grammar is as follows:

```Grammar
Input     -> Wire \n Wire
Wire      -> (Direction)(, Direction)*
Direction -> (U|D|L|R) (0-9)+
```

```rust
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
```

I decided to solve part 1 and 2 at the same time, so I stored the distance from the origin of each wire in the struct.

```rust
#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    d: u64,
}
```

First, I calculate where the wire turns for each direction, moving the initial point (0, 0)

```rust
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
```

Then I calculated the intersections of every pair of segments of the wires while calculating their distance in the wire

```rust
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
```

Finally, I calculated the answer for part 1 and 2

```rust
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
```

Both parts run in around 0.15ms on my machine.

## Opinion

This problem was a bit more challenging than the previous ones, especially finding the intersection of the wires, as there are a lot of cases for them.