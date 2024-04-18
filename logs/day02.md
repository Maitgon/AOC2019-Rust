# Day 02 : 1202 Program Alarm

* Difficulty: ⭐

## Description

This day's problem was to implement an IntCode computer. Each step of the IntCode computer consist in an OpCode, and three other integers that depending on the OpCode they mean different things. For the first part we need to run the IntCode program once and check the value at position 0. For the second part we need to find the combination of word and verb that produces the output 19690720.

## Problem solution

I know this year has a lot of IntCode problems, so I decided to make a separete file to have all the logic in a file. For now, I have a struct that represents the IntCode computer, and a function that runs the program.

```rust
pub type Program = Vec<usize>;

#[derive(Clone)]
pub struct IntCode {
    pub program: Program,
    pub ip: usize,
}

const ADD: usize = 1;
const MULT: usize = 2;
const HALT: usize = 99;

impl IntCode {
    pub fn new(program: Program) -> Self {
        Self { program, ip: 0 }
    }

    pub fn step(&mut self) -> bool {
        let opcode = self.program[self.ip];
        match opcode {
            ADD => {
                let a = self.program[self.ip + 1];
                let b = self.program[self.ip + 2];
                let c = self.program[self.ip + 3];
                self.program[c] = self.program[a] + self.program[b];
                self.ip += 4;
            }
            MULT => {
                let a = self.program[self.ip + 1];
                let b = self.program[self.ip + 2];
                let c = self.program[self.ip + 3];
                self.program[c] = self.program[a] * self.program[b];
                self.ip += 4;
            }
            HALT => return false,
            _ => panic!("Invalid opcode: {}", opcode),
        }
        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

}
```

First, the input is read into a vector of integers. For part 1, we simply need to run the IntCode program and return the value at position 0. For part 2, I decided to iterate over all possible combinations of word and verb until I find the one that produces the output 19690720.

```rust
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
```

Both parts run in around 8ms on my machine.

## Opinion

The IntCode computer is a very interesting concept. I'm looking forward to see how it will be used in the next problems. For now, I think it was really cool.
