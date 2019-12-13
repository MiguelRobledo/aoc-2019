use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::*;

#[allow(clippy::many_single_char_names)]
fn permute_with<F: FnMut(&[i64])>(input: &[i64], mut f: F) {
    // heap's algorithm
    let mut i = 0;
    let mut c = [0; 5];
    let mut a = input.to_vec();
    
    while i < a.len() {
        if c[i] < i {
            let (x, y) = if i % 2 == 0 {
                (0, i)
            } else {
                (c[i], i)
            };
            
            a.swap(x, y);
            
            f(&a);
            
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
}

#[aoc_generator(day7)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| i64::from_str_radix(n.trim(), 10).unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    fn get_thrust(input: &[i64], setting: &[i64]) -> i64 {
        setting
            .iter()
            .fold(0, |s, n| match Intcode::with_input(input, &[*n, s]).run() {
                Event::Output(o) => o,
                _ => panic!()
            })
    }
    
    let mut signal = 0;
    permute_with(&(0..=4).collect::<Vec<i64>>(), |p| signal = get_thrust(input, p).max(signal));
    
    signal
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    fn get_thrust(input: &[i64], setting: &[i64]) -> i64 {
        let mut signal = 0;
        let mut amplifiers: Vec<Intcode> = (0..setting.len()).map(|n| Intcode::with_input(input, &[setting[n]])).collect();
        let mut halt = false;
        
        loop {
            for a in &mut amplifiers {
                a.input(signal);
                match a.run() {
                    Event::Output(o) => signal = o,
                    Event::Halt => halt = true,
                    Event::Input => panic!()
                }
            }
            
            if halt {
                break signal;
            }
        }
    }
    
    let mut signal = 0;
    permute_with(&(5..=9).collect::<Vec<i64>>(), |p| signal = get_thrust(input, p).max(signal));
    
    signal
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ex1_p2() {
        assert_eq!(solve_part2(&input_gen("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")), 139629729);
    }
    
    #[test]
    fn ex2_p2() {
        assert_eq!(solve_part2(&input_gen("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")), 18216);
    }
}
