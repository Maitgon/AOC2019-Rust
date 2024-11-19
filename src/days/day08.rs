use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("inputs/day08.txt").unwrap().trim().to_string();

    let sol1: u64 = part1(&input_string);
    part2(&input_string);

    (Solution::from(sol1), Solution::from(0))
}

fn part1(input: &str) -> u64 {
    let width = 25;
    let height = 6;

    let layers: Vec<(u64, u64, u64)> = input
        .as_bytes()
        .chunks(width * height)
        .map(|layer| {
            layer
                .iter()
                .fold((0, 0, 0), |(zeros, ones, twos), &b| match b {
                    b'0' => (zeros + 1, ones, twos),
                    b'1' => (zeros, ones + 1, twos),
                    b'2' => (zeros, ones, twos + 1),
                    _ => unreachable!(),
                })
        })
        .collect();

    layers
        .iter()
        .min_by_key(|(zeros, _, _)| *zeros)
        .map(|(_, ones, twos)| ones * twos)
        .unwrap()
}

fn part2(input: &str) {
    let width = 25;
    let height = 6;

    let layers: Vec<Vec<u8>> = input
        .as_bytes()
        .chunks(width * height)
        .map(|layer| layer.to_vec())
        .collect();

    let mut image = vec![b'2'; width * height];

    for layer in layers {
        for (i, &b) in layer.iter().enumerate() {
            if image[i] == b'2' {
                image[i] = b;
            }
        }
    }

    for row in image.chunks(width) {
        for &b in row {
            print!("{}", if b == b'1' { '█' } else { ' ' });
        }
        println!();
    }
}
