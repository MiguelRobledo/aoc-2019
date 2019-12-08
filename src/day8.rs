use aoc_runner_derive::{aoc, aoc_generator};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc_generator(day8)]
pub fn input_gen(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .chunks(WIDTH * HEIGHT)
        .map(|v| v.to_vec())
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> usize {
    let count_n = |l: &Vec<u32>, n| l.iter().filter(|m| **m == n).count();
    
    let layer = input
        .iter()
        .min_by_key(|l| count_n(l, 0))
        .unwrap();
    
    count_n(layer, 1) * count_n(layer, 2)
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> String {
    let mut output = [2; WIDTH * HEIGHT];
    
    input
        .iter()
        .for_each(|l| l
            .iter()
            .enumerate()
            .for_each(|(n, v)| match v {
                0 | 1 if output[n] == 2 => output[n] = *v,
                _ => ()
            })
        );
    
    output
        .iter()
        .enumerate()
        .map(|(n, m)| {
            format!("{}{}",
                if n % WIDTH == 0 { "\n" } else { "" },
                if *m == 1 { "█" } else { " " }
            )
        })
        .collect()
}
