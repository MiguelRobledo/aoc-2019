use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

use crate::intcode::*;

fn paint_robot(hull: &mut HashMap<(i64, i64), i64>, input: &[i64]) {
    let mut intcode = Intcode::new(input);
    let mut p = (0, 0);
    let mut dir = 0;
    
    loop {
        intcode.input(*hull.entry(p).or_insert(0));
        
        if let Event::Output(o) = intcode.run() {
            hull.insert(p, o);
        } else {
            break;
        }
        
        if let Event::Output(o) = intcode.run() {
            dir = (dir + if o == 0 { 1 } else { 3 }) % 4;
            
            match dir {
                0 => p.1 -= 1,
                1 => p.0 -= 1,
                2 => p.1 += 1,
                _ => p.0 += 1
            }
        }
    }
}

#[aoc_generator(day11)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| i64::from_str_radix(n.trim(), 10).unwrap())
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[i64]) -> usize {
    let mut hull = HashMap::new();
    
    paint_robot(&mut hull, input);
    
    hull.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[i64]) -> String {
    let mut hull = HashMap::new();
    hull.insert((0, 0), 1);
    
    paint_robot(&mut hull, input);
    
    let pos = hull.keys();
    let min_x = pos.clone().map(|n| n.0).min().unwrap();
    let min_y = pos.clone().map(|n| n.1).min().unwrap();
    let max_x = pos.clone().map(|n| n.0).max().unwrap();
    let max_y = pos.map(|n| n.1).max().unwrap();
    
    (min_y..=max_y)
        .map(|y| (min_x..=max_x)
            .map(|x| if hull.get(&(x, y)) == Some(&1) { '█' } else { ' ' })
            .collect::<String>()
        )
        .map(|mut s| { s.insert(0, '\n'); s })
        .collect()
}
