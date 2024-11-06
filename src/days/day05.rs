use crate::{Solution, SolutionPair};
use crate::etc::intcode::IntCode;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("inputs/day05.txt").unwrap().trim().to_string();

    let input: Vec<i32> = input_string
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    // Your solution here...
    let mut program = IntCode::new(input);
    program.run();
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
