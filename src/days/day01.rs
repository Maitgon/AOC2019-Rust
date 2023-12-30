use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("inputs/day01.txt").unwrap();

    let input: Vec<u64> = input_string
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    // Your solution here...
    let sol1: u64 = input.clone().iter().map(|x| x / 3 - 2).sum();
    let sol2: u64 = input.iter().map(|x| get_fuel(*x as i64) as u64).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_fuel(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + get_fuel(fuel)
    }
}
