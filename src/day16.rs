use aoc_runner_derive::{aoc, aoc_generator};

fn fft(signal: &mut [i32]) {
    let tmp = signal.to_vec();
    
    for n in 0..signal.len() {
        let mut p = std::iter::repeat(true).take(n + 1)
            .chain(std::iter::repeat(false).take(n + 1))
            .cycle();
        
        signal[n] = tmp[n..]
            .chunks(n + 1)
            .step_by(2)
            .map(|c| c
                .iter()
                .map(|m|
                    if p.next().unwrap() { -*m } else { *m })
                .sum::<i32>()
            )
            .sum::<i32>()
            .abs() % 10;
    }
}

fn to_num(signal: &[i32]) -> i32 {
    signal
        .iter()
        .fold(0, |n, x| 10 * n + x)
}

#[aoc_generator(day16)]
pub fn input_gen(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut signal = input.to_vec();
    
    for _ in 0..100 {
        fft(&mut signal);
    }
    
    to_num(&signal[..8])
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let offset = to_num(&input[..7]) as usize;
    let mut signal: Vec<i32> = std::iter::repeat(input).take(10000).flatten().skip(offset).copied().collect();
    
    for _ in 0..100 {
        (0..signal.len() - 1)
            .rev()
            .for_each(|i| signal[i] += signal[i + 1]);
        
        signal.iter_mut().for_each(|x| *x %= 10);
    }
    
    to_num(&signal[0..8])
}
