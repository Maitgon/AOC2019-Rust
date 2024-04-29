# Day 4: Secure Container

* Difficulty: ⭐

## Description

This problem consist in finding the number of valid passwords in a range. A valid password is one that has at least one pair of adjacent digits that are the same and that the digits never decrease from left to right. In part2, an additional constraint is added: the pair of adjacent digits must be exactly 2 digits long.

## Problem solution

This problem is one of the rare instances in aoc where you don't need to parse the input as the input is the range itself. Again, I decided to solve both parts at the same time, taking into account that a valid password for part2 is just a valid password for part 1 but with an additional constraint. The solution for part 1 is very straightforward, I converted the password to an array of digits and checked if it was valid:

```rust
    let a_num = convert_to_array(num);

    let mut two_adjacent = false;

    for i in 0..5 {
        if a_num[i] == a_num[i+1] {
            two_adjacent = true;
        }
        if a_num[i] > a_num[i+1] {
            return false;
        }
    }

    two_adjacent
```

For part 2 I decided to use [groub_by](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.group_by) from Itertools and then check if there are any groups of exactly 2 elements:

```rust
    a_num.into_iter()
        .group_by(|element| *element)
        .into_iter()
        .any(|(_, value)| value.count() == 2)
```

This solution could be impoved easily if we increment the numbers to make sure that the digits never decrease, but my solution runs in about 1.8ms so I didn't bother with it.

## Opinion

This problem was really easy, but it was fun to try some functions of the itertools library.
