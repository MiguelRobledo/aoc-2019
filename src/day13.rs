use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::*;

const DISPLAY: bool = false;
const SIZE_X: usize = 43;
const SIZE_Y: usize = 23;

#[aoc_generator(day13)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| i64::from_str_radix(n.trim(), 10).unwrap())
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[i64]) -> usize {
    let mut intcode = Intcode::new(input);
    let mut output = Vec::new();
    
    intcode.execute(|o| output.push(o));
    
    output
        .chunks(3)
        .filter(|x| x[2] == 2)
        .count()
}


#[aoc(day13, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut intcode = Intcode::new(input);
    let mut tmp = Vec::new();
    let mut output = [0; SIZE_X * SIZE_Y];
    let mut score = 0;
    
    let mut ball = (0, 0);
    let mut paddle = (0, 0);
    
    intcode.set_mem(0, 2);
    
    let draw = |output: &[i64]| {
        println!("{}", (0..SIZE_Y)
            .map(|y| (0..SIZE_X)
                .map(|x| match output[x + y * SIZE_X] {
                    1 => '█',
                    2 => '#',
                    3 => '_',
                    4 => 'o',
                    _ => ' ',
                })
                .collect::<String>()
            )
            .map(|mut s| { s.insert(0, '\n'); s })
            .collect::<String>()
        );
    };
    
    loop {
        match intcode.run() {
            Event::Input => {
                if DISPLAY {
                    draw(&output);
                }
                
                intcode.input(
                    if ball.0 < paddle.0 { -1 }
                    else if ball.0 > paddle.0 { 1 }
                    else { 0 }
                );
            },
            Event::Output(o) => if tmp.len() == 2 {
                if (tmp[0], tmp[1]) != (-1, 0) {
                    match o {
                        3 => paddle = (tmp[0], tmp[1]),
                        4 => ball = (tmp[0], tmp[1]),
                        _ => ()
                    };
                    
                    output[tmp[0] as usize + tmp[1] as usize * SIZE_X] = o;
                } else {
                    score = o
                }
                
                tmp.clear();
            } else {
                tmp.push(o);
            },
            Event::Halt => break score
        }
    }
}
