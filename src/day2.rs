use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2, part1)]
pub fn input_gen_part1(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(",")
        .map(|n| u64::from_str_radix(n, 10).unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    let mut mem = input.to_vec();
    let mut pc = 0;
    
    loop {
        let n = *mem.get(pc).unwrap();
        if n == 99 {
            break;
        }
        
        let x_addr    = *mem.get(pc + 1).unwrap() as usize;
        let y_addr    = *mem.get(pc + 2).unwrap() as usize;
        let dest_addr = *mem.get(pc + 3).unwrap() as usize;
        
        let x = mem.get(x_addr).unwrap();
        let y = mem.get(y_addr).unwrap();
        
        *mem.get_mut(dest_addr).unwrap() = match n {
            1 => x + y,
            2 => x * y,
            _ => panic!("invalid opcode"),
        };
        
        pc += 4;
    }
    
    *mem.first().unwrap()
}

#[aoc_generator(day2, part2)]
pub fn input_gen_part2(input: &str) -> Vec<u64> {
    input_gen_part1(input)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    let mut mem = input.to_vec();
    let target = 19690720;
    let max = 100;
    
    for n in 0..max {
        for v in 0..max {
            mem[1] = n;
            mem[2] = v;
            
            if solve_part1(&mem.clone()) == target {
                return 100 * n + v;
            }
        }
    }
    
    panic!("couldn't find a suitable pair");
}
