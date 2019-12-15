use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

use crate::intcode::*;

const DISPLAY: bool = false;
const EXPLORABLE_TILES: usize = 1658;

struct Node {
    contents: u8,
    visits: Vec<u32>
}

impl Node {
    fn new(contents: u8) -> Self {
        Node { contents, visits: vec![0; 4] }
    }
    
    fn new_from(contents: u8, dir: u8) -> Self {
        let mut visits = vec![0; 4];
        let opposite_dir = match dir {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => unreachable!()
        };
        
        visits[opposite_dir - 1] = 1;
        Node { contents, visits }
    }
    
    fn set_wall(&mut self, dir: u8) {
        self.visits[dir as usize - 1] = 9999;
    }
    
    fn get_new_dir(&mut self) -> u8 {
        let least_visited = self.visits
            .iter()
            .enumerate()
            .min_by_key(|(_, x)| *x)
            .unwrap()
            .0;
        
        self.visits[least_visited] += 1;
        
        (least_visited + 1) as u8
    }
}

fn next_position((x, y): (i64, i64), dir: u8) -> (i64, i64) {
    match dir {
        1 => (x, y - 1),
        2 => (x, y + 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => unreachable!()
    }
}

fn draw_map(map: &HashMap<(i64, i64), Node>) {
    let pos = map.keys();
    let min_x = pos.clone().map(|n| n.0).min().unwrap();
    let min_y = pos.clone().map(|n| n.1).min().unwrap();
    let max_x = pos.clone().map(|n| n.0).max().unwrap();
    let max_y = pos.map(|n| n.1).max().unwrap();
    
    let output: String = (min_y..=max_y)
        .map(|y| (min_x..=max_x)
            .map(|x| match map.get(&(x, y)) {
                Some(n) => match n.contents {
                    1 => if x == 0 && y == 0 { 'O' } else { ' ' },
                    2 => 'X',
                    _ => '█'
                },
                _ => '?',
            })
            .collect::<String>()
        )
        .map(|mut s| { s.insert(0, '\n'); s })
        .collect();
    
    println!("{}", output);
}

fn generate_map(input: &[i64]) -> (HashMap<(i64, i64), Node>, (i64, i64)) {
    let mut intcode = Intcode::new(input);
    let mut p = (0, 0);
    let mut map: HashMap<(i64, i64), Node> = HashMap::new();
    let mut dir = 1;
    let mut oxygen_system = (0, 0);
    
    map.insert(p, Node::new(1));
    
    loop {
        match intcode.run() {
            Event::Input => {
                dir = map.get_mut(&p).unwrap().get_new_dir();
                
                intcode.input(dir as i64);
            },
            Event::Output(o) => {
                let next = next_position(p, dir);
                
                match o {
                    1 | 2 => {
                        p = next;
                        map.entry(p).or_insert(Node::new_from(o as u8, dir));
                        
                        if o == 2 {
                            oxygen_system = p;
                        }
                    },
                    _ => {
                        map.entry(p).and_modify(|x| x.set_wall(dir));
                        map.insert(next, Node::new(o as u8));
                    }
                }
            },
            Event::Halt => break
        }
        
        if map.len() == EXPLORABLE_TILES {
            break;
        }
    }
    
    if DISPLAY {
        draw_map(&map);
    }
    
    (map, oxygen_system)
}

#[aoc_generator(day15)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| i64::from_str_radix(n.trim(), 10).unwrap())
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[i64]) -> u64 {
    let map = generate_map(input).0;
    
    let mut explored = vec![];
    let mut explorable = vec![((0, 0), 0)];
    
    loop {
        let (pos, dist) = explorable.pop().unwrap();
        explored.push(pos);
        
        let node = map.get(&pos).unwrap();
        if node.contents == 2 {
            break dist;
        }
        
        (1..=4)
            .map(|n| next_position(pos, n))
            .filter(|p| map.get(&p).unwrap().contents != 0)
            .for_each(|p| if !explored.contains(&p) {
                explorable.push((p, dist + 1))
            });
    }
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[i64]) -> u64 {
    let (map, p) = generate_map(input);
    
    let mut explored = vec![];
    let mut explorable = vec![(p, 0)];
    
    loop {
        let (pos, dist) = explorable.pop().unwrap();
        explored.push((pos, dist));
        
        (1..=4)
            .map(|n| next_position(pos, n))
            .filter(|p| map.get(&p).unwrap().contents != 0)
            .for_each(|p| if explored.iter().all(|(pos, _)| p != *pos) {
                explorable.push((p, dist + 1))
            });
        
        if explorable.is_empty() {
            break;
        }
    }
    
    explored
        .iter()
        .max_by_key(|(_, d)| d)
        .unwrap()
        .1
}
