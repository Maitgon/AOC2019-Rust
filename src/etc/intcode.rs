pub type Program = Vec<i32>;

#[derive(Clone)]
pub struct IntCode {
    pub program: Program,
    pub ip: usize,
}

const ADD: usize = 1;
const MULT: usize = 2;
const INPUT: usize = 3;
const OUTPUT: usize = 4;
const JUMP_IF_TRUE: usize = 5;
const JUMP_IF_FALSE: usize = 6;
const LESS_THAN: usize = 7;
const EQUALS: usize = 8;
const HALT: usize = 99;

impl IntCode {
    pub fn new(program: Program) -> Self {
        Self { program, ip: 0 }
    }

    pub fn step(&mut self) -> bool {
        let opcode = (self.program[self.ip] % 100) as usize;

        let par_mode = (self.program[self.ip] / 100) as usize;
        match opcode {
            ADD => {
                let a = self.get_param(self.ip + 1, par_mode & 1);
                let b = self.get_param(self.ip + 2, (par_mode / 10) & 1);
                let c = self.program[self.ip + 3]; // never in immediate mode
                self.program[c as usize] = a + b;
                self.ip += 4;
            }
            MULT => {
                let a = self.get_param(self.ip + 1, par_mode & 1);
                let b = self.get_param(self.ip + 2, (par_mode / 10) & 1);
                let c = self.program[self.ip + 3]; // never in immediate mode
                self.program[c as usize] = a * b;
                self.ip += 4;
            }
            INPUT => {
                let a = self.program[self.ip + 1];
                // get a number input not hardcoded
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                self.program[a as usize] = input.trim().parse().unwrap();
                self.ip += 2;
            }
            OUTPUT => {
                let a = self.get_param(self.ip + 1, par_mode & 1);
                println!("{}", a);
                self.ip += 2;
            }
            JUMP_IF_TRUE => {
                let a = self.get_param(self.ip + 1, par_mode & 1);
                let b = self.get_param(self.ip + 2, (par_mode / 10) & 1);
                if a != 0 {
                    self.ip = b as usize;
                } else {
                    self.ip += 3;
                }
            }
            JUMP_IF_FALSE => {
                let a = self.get_param(self.ip + 1, par_mode & 1);
                let b = self.get_param(self.ip + 2, (par_mode / 10) & 1);
                if a == 0 {
                    self.ip = b as usize;
                } else {
                    self.ip += 3;
                }
            }
            LESS_THAN => {
                let a = self.get_param(self.ip + 1, par_mode & 1);
                let b = self.get_param(self.ip + 2, (par_mode / 10) & 1);
                let c = self.program[self.ip + 3]; // never in immediate mode
                self.program[c as usize] = if a < b { 1 } else { 0 };
                self.ip += 4;
            }  
            EQUALS => {
                let a = self.get_param(self.ip + 1, par_mode & 1);
                let b = self.get_param(self.ip + 2, (par_mode / 10) & 1);
                let c = self.program[self.ip + 3]; // never in immediate mode
                self.program[c as usize] = if a == b { 1 } else { 0 };
                self.ip += 4;
            }
            HALT => return false,
            _ => panic!("Invalid opcode: {}", opcode),
        }
        true
    }

    pub fn get_param(&self, pos: usize, mode: usize) -> i32 {
        match mode {
            0 => self.program[self.program[pos] as usize],
            1 => self.program[pos],
            _ => panic!("Invalid mode: {}", mode),
        }
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

}

