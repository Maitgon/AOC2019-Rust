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

