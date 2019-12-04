use aoc_runner_derive::{aoc, aoc_generator};

fn count_matches(min: u64, max: u64, rules: fn(&Vec<u8>) -> bool) -> usize {
    (min..max)
        .map(|n| n
                .to_string()
                .bytes()
                .collect()
        )
        .filter(|v: &Vec<u8>| rules(v))
        .count()
}

fn is_non_decreasing(v: &Vec<u8>) -> bool {
    v
        .windows(2)
        .all(|w| w[0] <= w[1])
}

fn has_double(v: &Vec<u8>) -> bool {
    v
        .windows(2)
        .any(|w| w[0] == w[1])
}

fn has_exact_double(v: &Vec<u8>) -> bool {
    v
        .windows(2)
        .fold((false, 1), |s, w|
            match s {
                (false, 2) if w[0] != w[1] => (true, 2),
                (false, c) if w[0] == w[1] => (false, c + 1),
                (false, _) => (false, 1),
                _ => s
            }
        )
        .1 == 2
}

#[aoc_generator(day4, part1)]
pub fn input_gen_part1(input: &str) -> Vec<u64> {
    input
        .split("-")
        .map(|c| u64::from_str_radix(c, 10).unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[u64]) -> usize {
    count_matches(input[0], input[1], |v| is_non_decreasing(v) && has_double(v))
}

#[aoc_generator(day4, part2)]
pub fn input_gen_part2(input: &str) -> Vec<u64> {
    input_gen_part1(input)
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[u64]) -> usize {
    count_matches(input[0], input[1], |v| is_non_decreasing(v) && has_exact_double(v))
}
