use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::*;

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| i64::from_str_radix(n, 10).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut intcode = Intcode::with_input(input, &[1]);
    let mut output = 0;
    
    intcode.execute(|o| output = o);
    
    output
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut intcode = Intcode::with_input(input, &[5]);
    let mut output = 0;
    
    intcode.execute(|o| output = o);
    
    output
}
