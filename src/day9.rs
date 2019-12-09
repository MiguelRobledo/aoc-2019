use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::*;

#[aoc_generator(day9)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| i64::from_str_radix(n.trim(), 10).unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut intcode = Intcode::with_input(input, &[1]);
    let mut output = 0;
    
    intcode.execute(|o| output = o);
    
    output
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut intcode = Intcode::with_input(input, &[2]);
    let mut output = 0;
    
    intcode.execute(|o| output = o);
    
    output
}
