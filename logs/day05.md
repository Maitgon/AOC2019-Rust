# Day 5: Sunny with a Chance of Asteroids

* Difficulty: ⭐⭐

## Description

In this problem, we need to implement more operations to the Intcode computer. The new operations are:

* **Input**: Takes a single integer as input and saves it to the position given by the parameter.
* **Output**: Outputs the value of the parameter.
* **Position mode**: The parameter is a position.
* **Immediate mode**: The parameter is a value.
* **Jump if true**: If the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter.
* **Jump if false**: If the first parameter is zero, it sets the instruction pointer to the value from the second parameter.
* **Less than**: If the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
* **Equals**: If the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.

## Problem solution

The implementations are as follows:

```rust
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
```

For the inmediate mode, we need to change the `get_param` function to return the value directly if the mode is 1:

```rust
    pub fn get_param(&self, pos: usize, mode: usize) -> i32 {
        match mode {
            0 => self.program[self.program[pos] as usize],
            1 => self.program[pos],
            _ => panic!("Invalid mode: {}", mode),
        }
    }
```



## Opinion

Building an IntComputer little by little is quite interesting.
