use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use itertools::Itertools;
use crate::etc::intcode::IntCode;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("inputs/day07.txt").unwrap().trim().to_string();

    let input: Vec<i32> = input_string
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let sol1 = part1(&input);
    let sol2 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// Part 1

fn part1(input: &[i32]) -> i32 {
    let perm = (0..5).permutations(5);

    let mut max = 0;

    for p in perm {
        let mut signal = 0;

        for i in 0..5 {
            let mut intcode = IntCode::new(input.to_vec());
            intcode.input.push(p[i]);
            intcode.input.push(signal);
            intcode.run();
            signal = intcode.output[0];
        }

        if signal > max {
            max = signal;
        }
    }

    max
}

// Part 2
fn part2(input: &[i32]) -> i32 {
    let perm = (5..10).permutations(5);

    let mut max = 0;
    for p in perm {
        let signal = get_signal(p, input);
        if signal > max {
            max = signal;
        }
    }
    
    max
}

fn get_signal(p: Vec<i32>, input: &[i32]) -> i32 {
    let mut programs = Vec::new();
    for i in 0..5 {
        programs.push(IntCode::new(input.to_vec()));
        programs[i].input.push(p[i]);
    }

    let mut signal = 0;
    let mut signal_5 = 0;
    let mut exit = false;

    while !exit {
        for i in 0..5 {
            programs[i].input.push(signal);
            programs[i].run();
            if programs[i].halted || programs[i].output.is_empty() {
                exit = true;
                break;
            }
            signal = programs[i].output.remove(0);
            if i == 4 {
                signal_5 = signal;
            }
        }
    }

    signal_5
}
