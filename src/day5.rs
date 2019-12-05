use aoc_runner_derive::{aoc, aoc_generator};

fn get_value(mem: &[i64], pc: usize, n: u32) -> i64 {
    if mem[pc] / 10_i64.pow(n + 1) % 10 == 1 {
        mem[pc + n as usize]
    } else {
        mem[mem[pc + n as usize] as usize]
    }
}

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|n| i64::from_str_radix(n, 10).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[i64]) -> bool {
    let data = vec![1];
    let mut it = data.iter();
    let mut mem = input.to_vec();
    let mut pc = 0;
    
    loop {
        let opcode = mem[pc] % 100;
        
        match opcode {
            1 | 2 => {
                let (x, y, z) = (get_value(&mem, pc, 1), get_value(&mem, pc, 2), mem[pc + 3] as usize);
                
                mem[z] = match opcode {
                    1 => x + y,
                    2 => x * y,
                    _ => unreachable!()
                };
                
                pc += 4;
            },
            3 | 4 => {
                let x = mem[pc + 1] as usize;
                
                match opcode {
                    3 => mem[x] = *it.next().unwrap(),
                    4 => println!("{}", mem[x]),
                    _ => unreachable!()
                }
                
                pc += 2;
            },
            99 => break,
            _ => panic!("invalid opcode {}", opcode),
        };
        
    }
    
    true
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[i64]) -> bool {
    let data = vec![5];
    let mut it = data.iter();
    let mut mem = input.to_vec();
    let mut pc = 0;
    
    loop {
        let opcode = mem[pc] % 100;
        
        match opcode {
            1 | 2 | 7 | 8 => {
                let (x, y, z) = (get_value(&mem, pc, 1), get_value(&mem, pc, 2), mem[pc + 3] as usize);
                
                mem[z] = match opcode {
                    1 => x + y,
                    2 => x * y,
                    7 => if x < y { 1 } else { 0 },
                    8 => if x == y { 1 } else { 0 },
                    _ => unreachable!()
                };
                
                pc += 4;
            },
            3 | 4 => {
                let x = mem[pc + 1] as usize;
                
                match opcode {
                    3 => mem[x] = *it.next().unwrap(),
                    4 => println!("{}", mem[x]),
                    _ => unreachable!()
                }
                
                pc += 2;
            },
            5 | 6 => {
                let (x, y) = (get_value(&mem, pc, 1), get_value(&mem, pc, 2) as usize);
                
                if (opcode == 5 && x != 0) || (opcode == 6 && x == 0) {
                    pc = y;
                    continue;
                }
                
                pc += 3;
            },
            99 => break,
            _ => panic!("invalid opcode {}", opcode),
        };
        
    }
    
    true
}
