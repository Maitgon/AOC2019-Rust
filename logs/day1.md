# Day 01 : The Tyranny of the Rocket Equation

* Difficulty: ⭐

## Description

First day was very simple. The problem required to read a series of numbers from a file and apply some sort of formula to them. For part 1, the formula was to divide by 3, round down, and subtract 2. For part 2, the formula was to divide by 3, round down, subtract 2, and repeat until the result is less than or equal to 0. The sum of all the results is the answer.

## Problem solution

This is a pretty straightforwards problem. The input is very easy to parse, and the formula is very easy to implement. I just parsed the numbers into a vector, and then iterated over the vector applying each the corresponding formula to each number.

```rust
let sol1: u64 = input.clone().iter().map(|x| x / 3 - 2).sum();
let sol2: u64 = input.iter().map(|x| get_fuel(*x as i64) as u64).sum();
```

where I implemented the function get_fuel as follows:

```rust
fn get_fuel(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + get_fuel(fuel)
    }
}
```

This solution runs in about 9 &mu;s on my machine. I can think of a few ways to improve it, first would be to use multiple threads to calculate the fuel for each number. Another idea is, calculate the sum of the numbers while they are beeing read instead of storing them in a slice and then interating over them again. Lastly, I'm not sure if there is a closed form to calculate the second part getFuel function, but if there is, that would be a good imprevement. But after all, I think 9 &mu;s is good enough and I don't really think that it's worth it to improve it further.

## Opinion

I think this was a good first problem. It was very simple and easy to implement, I can't say much more about it.
