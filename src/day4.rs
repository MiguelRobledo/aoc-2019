use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4, part1)]
pub fn input_gen_part1(input: &str) -> Vec<u64> {
    input
        .split("-")
        .map(|c| u64::from_str_radix(c, 10).unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[u64]) -> usize {
    (input[0]..input[1])
        .map(|n| n
                .to_string()
                .chars()
                .zip(n
                    .to_string()
                    .chars()
                    .skip(1)
                )
                .collect()
        )
        .filter(|v: &Vec<(char, char)>|
            v
                .iter()
                .all(|(curr, next)| curr <= next)
            &&
            v
                .iter()
                .any(|(curr, next)| curr == next)
        )
        .count()
}

#[aoc_generator(day4, part2)]
pub fn input_gen_part2(input: &str) -> Vec<u64> {
    input_gen_part1(input)
}


#[aoc(day4, part2)]
pub fn solve_part2(input: &[u64]) -> usize {
    (input[0]..input[1])
        .map(|n| n
            .to_string()
            .chars()
            .zip(n
                .to_string()
                .chars()
                .skip(1)
            )
            .collect()
        )
        .filter(|v: &Vec<(char, char)>|
            v
                .iter()
                .all(|(curr, next)| curr <= next)
            &&
            v
                .iter()
                .fold((false, 1), |s, (curr, next)|
                    match s {
                        (false, 2) if curr != next => (true, 2),
                        (false, c) if curr == next => (false, c + 1),
                        (false, _) => (false, 1),
                        _ => s
                    }
                )
                .1 == 2
        )
        .count()
}
