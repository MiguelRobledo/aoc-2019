use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::Intcode;

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| i64::from_str_radix(n, 10).unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut intcode = Intcode::new(input);
    intcode.run();
    
    intcode.get_mem(0)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut mem = input.to_vec();
    let target = 19_690_720;
    let max = 100;
    
    for n in 0..max {
        for v in 0..max {
            mem[1] = n;
            mem[2] = v;
            
            if solve_part1(&mem.clone()) == target {
                return 100 * n + v;
            }
        }
    }
    
    panic!("couldn't find a suitable pair");
}
