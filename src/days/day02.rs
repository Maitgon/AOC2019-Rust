use crate::{Solution, SolutionPair, etc::intcode::IntCode};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("inputs/day02.txt").unwrap().trim().to_string();

    let input: Vec<i32> = input_string
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    // Your solution here...
    let program = IntCode::new(input);
    let sol1: u64 = part1(program.clone());
    let sol2: u64 = part2(program);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(mut program: IntCode) -> u64 {
    program.program[1] = 12;
    program.program[2] = 2;
    program.run();
    program.program[0] as u64
}

fn part2(program: IntCode) -> u64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = program.clone();
            program.program[1] = noun;
            program.program[2] = verb;
            program.run();
            if program.program[0] == 19690720 {
                return 100 * noun as u64 + verb as u64;
            }
        }
    }

    0
}