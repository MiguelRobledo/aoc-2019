use aoc_runner_derive::{aoc, aoc_generator};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = b;
        
        b = a % b;
        a = tmp;
    }
    
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn calc_energy(position: &[i64], velocity: &[i64]) -> i64 {
    position
        .iter()
        .map(|n| n.abs())
        .sum::<i64>()
    * velocity
        .iter()
        .map(|n| n.abs())
        .sum::<i64>()
}

fn calc_vel(who: usize, positions: &[Vec<i64>]) -> Vec<i64> {
    let mut vel = vec![0; 3];
    
    positions
        .iter()
        .enumerate()
        .filter(|(n, _)| *n != who)
        .for_each(|(_, p)| p
            .iter()
            .enumerate()
            .for_each(|(m, c)|
                vel[m] += if c > &positions[who][m] { 1 }
                else if c < &positions[who][m] { -1 } else { 0 })
        );
    
    vel
}

fn step_universe(positions: &mut [Vec<i64>], velocities: &mut [Vec<i64>]) {
    for i in 0..positions.len() {
        calc_vel(i, &positions)
            .iter()
            .enumerate()
            .for_each(|(n, c)| velocities[i][n] += c)
    }
    
    for i in 0..positions.len() {
        for j in 0..3 {
            positions[i][j] += velocities[i][j];
        }
    }
}

#[aoc_generator(day12)]
pub fn input_gen(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|l| l
            .trim_matches(|c| c == '<' || c == '>')
            .split(',')
            .map(|s| s.trim_matches(|c: char|
                c != '-' && !c.is_digit(10)).parse::<i64>().unwrap())
            .collect()
        )
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Vec<i64>]) -> i64 {
    let mut positions = input.to_vec();
    let mut velocities = vec![vec![0; 3]; positions.len()];
    
    for _ in 0..1000 {
        step_universe(&mut positions, &mut velocities);
    }
    
    positions
        .iter()
        .zip(velocities)
        .map(|(p, v)| calc_energy(p, &v))
        .sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Vec<i64>]) -> i64 {
    let mut positions = input.to_vec();
    let mut velocities = vec![vec![0; 3]; positions.len()];
    let mut cycles = vec![0; 3];
    let mut found = vec![false; 3];
    
    let initial_state = positions.clone();
    
    loop {
        step_universe(&mut positions, &mut velocities);
        
        for i in 0..3 {
            if !found[i] {
                cycles[i] += 1;
                found[i] = positions
                    .iter()
                    .zip(initial_state.iter())
                    .all(|(p, p0)| p[i] == p0[i])
                    && velocities.iter().all(|v| v[i] == 0);
            }
        }
        
        if found.iter().all(|f| *f) {
            break;
        }
    }
    
    lcm(lcm(cycles[0], cycles[1]), cycles[2])
}
