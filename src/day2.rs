use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<u64> {
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
        let x = mem[mem[pc + 1] as usize];
        let y = mem[mem[pc + 2] as usize];
        let dest = mem[pc + 3] as usize;
        
        mem[dest] = match mem[pc] {
            1 => x + y,
            2 => x * y,
            99 => break,
            _ => panic!("invalid opcode"),
        };
        
        pc += 4;
    }
    
    mem[0]
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
