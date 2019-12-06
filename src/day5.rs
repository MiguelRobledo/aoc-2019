use aoc_runner_derive::{aoc, aoc_generator};

pub struct Intcode {
    mem: Vec<i64>,
    pc: usize,
    input: Vec<i64>
}

impl Intcode {
    fn with_input(mem: Vec<i64>, input: Vec<i64>) -> Self {
        Intcode { mem, pc: 0, input }
    }
    
    fn get_arg(&self, n: u32) -> i64 {
        if self.mem[self.pc] / 10_i64.pow(n + 1) % 10 == 1 {
            self.mem[self.pc + n as usize]
        } else {
            self.mem[self.mem[self.pc + n as usize] as usize]
        }
    }
    
    pub fn run(&mut self) {
        let mut it = self.input.iter();
        
        loop {
            let opcode = self.mem[self.pc] % 100;
            
            match opcode {
                1 | 2 | 7 | 8 => {
                    let (x, y, z) = (
                        self.get_arg(1),
                        self.get_arg(2),
                        self.mem[self.pc + 3] as usize
                    );
                    
                    self.mem[z] = match opcode {
                        1 => x + y,
                        2 => x * y,
                        7 => if x < y { 1 } else { 0 },
                        8 => if x == y { 1 } else { 0 },
                        _ => unreachable!()
                    };
                    
                    self.pc += 4;
                },
                3 | 4 => {
                    let x = self.mem[self.pc + 1] as usize;
                    
                    match opcode {
                        3 => self.mem[x] = *it.next().unwrap(),
                        4 => println!("{}", self.mem[x]),
                        _ => unreachable!()
                    }
                    
                    self.pc += 2;
                },
                5 | 6 => {
                    let (x, y) = (
                        self.get_arg(1),
                        self.get_arg(2) as usize
                    );
                    
                    if (opcode == 5 && x != 0) || (opcode == 6 && x == 0) {
                        self.pc = y;
                        continue;
                    }
                    
                    self.pc += 3;
                },
                99 => break,
                _ => panic!("invalid opcode {}", opcode),
            }
        }
    }
}

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| i64::from_str_radix(n, 10).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[i64]) -> bool {
    Intcode::with_input(input.to_vec(), vec![1]).run();
    
    true
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[i64]) -> bool {
    Intcode::with_input(input.to_vec(), vec![5]).run();
    
    true
}
