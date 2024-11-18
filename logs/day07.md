# Day 7: Amplification Circuit

* Difficulty: ⭐⭐

## Description

In this problem, we need to make use of the IntCode Computer we made some days ago to solve it. Firts we need to run a program several times and then we will do the same but in a loop.

## Problem solution

For part 1, I decided to change a bit my IntCode computer so I can feed it input and take outputs from the code instead of the command line, I also added a way to see if the program halted or not:

```rust
pub struct IntCode {
    pub program: Program,
    pub ip: usize,
    pub input: Vec<i32>,
    pub output: Vec<i32>,
    pub halted: bool,
}
```

The idea is simple, just run the program and the output of each amplifier is the input of the next one, we need to check every combination of \[0,1,2,3,4\] to check which is the maximum value:

```rust
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
```

For part 2 I was really really confused, I didn't really understand what I was supossed to do, and in the end I had to look it up because I was spending quite some time in this part. So, the idea is that, when a program outputs something, you continue to the next amplifier but the program doesn't reset, so that when the amplifers loop back, the program continues. In this part, when we reach amplifier 5, the output of the amplifier goes to amplifier 1, and when the program halts, we need to output the last value that amplifier 5 outputted:

```rust
n part2(input: &[i32]) -> i32 {
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
            signal = programs[i].output.remove(0);;
            if i == 4 {
                signal_5 = signal;
            }
        }
    }

    signal_5
}
```

Both solutions ran in about 0.4ms so I'm happy with the performance, I don't really want to go back and try to improve it because this problem tired me a bit.

## Opinion

I really didn't like part 2, Not that it was particullarly difficult, but I really couldn't understand what it was asking for, and took way longer than I should... In the other hand, it is really nice that we can start using the computer that we made before, you can start seeing it working and that's really cool

