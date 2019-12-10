use aoc_runner_derive::{aoc, aoc_generator};

fn are_coprime(mut a: i64, mut b: i64) -> bool {
    while b != 0 {
        let tmp = b;
        
        b = a % b;
        a = tmp;
    }
    
    a == 1
}

fn check_los((mut x, mut y): (i64, i64), (dx, dy): (i64, i64), input: &[Vec<bool>]) -> Option<(i64, i64)> {
    loop {
        x += dx;
        y += dy;
        
        if (x < 0 || x >= input[0].len() as i64)
            || (y < 0 || y >= input.len() as i64) {
            break None;
        } else if input[y as usize][x as usize] {
            break Some((x, y));
        }
    }
}

fn get_directions(input: &[Vec<bool>]) -> Vec<(i64, i64)> {
    let mut v = (0..input.len() as i64)
        .flat_map(|n| (0..n)
            .map(|m| (n, m))
            .filter(|(n, m)| are_coprime(*n, *m))
            .collect::<Vec<(i64, i64)>>()
        )
        .chain(std::iter::once((1, 1)))
        .flat_map(|(m, n)| vec![
            (n, m), (n, -m), (-n, m), (-n, -m),
            (m, n), (m, -n), (-m, n), (-m, -n)
        ])
        .collect::<Vec<(i64, i64)>>();
    
    v.sort();
    v.dedup();
    
    v
}

fn count_asteroids((x, y): (i64, i64), directions: &[(i64, i64)], input: &[Vec<bool>]) -> usize {
    directions
        .iter()
        .filter(|(n, m)| check_los((x, y), (*n, *m), input) != None)
        .count()
}

fn get_optimal_location(input: &[Vec<bool>]) -> ((i64, i64), usize) {
    let directions = get_directions(input);
    
    (0..input.len() as i64)
        .filter_map(|x| (0..input[0].len() as i64)
            .filter_map(|y| if input[y as usize][x as usize] {
                    Some(((x, y), count_asteroids((x, y), &directions, input)))
                } else { None }
            )
            .max_by_key(|k| k.1)
        )
        .max_by_key(|k| k.1)
        .unwrap()
}

#[aoc_generator(day10)]
pub fn input_gen(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|l| l
            .chars()
            .map(|c| c == '#')
            .collect()
        )
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Vec<bool>]) -> usize {
    get_optimal_location(input).1
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Vec<bool>]) -> i64 {
    let base = get_optimal_location(input).0;
    
    let mut directions = get_directions(input);
    directions.sort_by_cached_key(|(dx, dy)| (1024. * (*dy as f64).atan2(*dx as f64)) as i64);
    
    let mut it = directions.iter().cycle().peekable();
    let mut field = input.to_vec();
    let mut vaporized = 0;
    
    while let Some(p) = it.peek() {
        if **p == (0, -1) {
            break;
        } else {
            it.next();
        }
    }
    
    loop {
        if let Some(p) = check_los(base, *it.next().unwrap(), &field) {
            field[p.1 as usize][p.0 as usize] = false;
            vaporized += 1;
            
            if vaporized == 200 {
                break 100 * p.0 + p.1;
            }
        }
    }
}
