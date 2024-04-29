use crate::{Solution, SolutionPair};
use itertools::Itertools;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let min = 307237;
    let max = 769058;
    // Your solution here...
    let (sol1, sol2) = part(min, max);

    (Solution::from(sol1), Solution::from(sol2))
}

// Part 1

fn part(min: u64, max: u64) -> (u64, u64) {
    let mut count1 = 0;
    let mut count2 = 0;

    for i in min..=max {
        if is_valid_password(i) {
            count1 += 1;
            if is_valid_password_2(i) {
                count2 += 1;
            }
        }
    }

    (count1, count2)
}

fn is_valid_password(num: u64) -> bool {
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
}

fn convert_to_array(num: u64) -> [u64; 6] {
    let mut new_num = num;
    let mut a_num: [u64; 6] = [0, 0, 0, 0, 0, 0];
    let mut i = 5;

    while new_num != 0 {
        a_num[i] = new_num % 10;
        new_num /= 10;
        i -= 1;
    }

    a_num
}

fn is_valid_password_2(num: u64) -> bool {
    let a_num = convert_to_array(num);

    a_num.into_iter()
        .group_by(|element| *element)
        .into_iter()
        .any(|(_, value)| value.count() == 2)
}
