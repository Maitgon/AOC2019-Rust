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
    let mut program = IntCode::new(input.to_vec());
    
    program.input.push(1);
    program.run();
    // Your solution here...
    println!("{:?}", program.output);
    let sol1: i32 = program.output[program.output.len() - 1];

    let mut program = IntCode::new(input);

    program.input.push(5);
    program.run();
    let sol2: i32 = program.output[0];

    (Solution::from(sol1), Solution::from(sol2))
}
