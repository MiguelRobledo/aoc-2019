use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_gen(input: &str) -> Vec<(String, String)> {
    input
        .trim()
        .split("\n")
        .map(|l| {
            let x: Vec<&str> = l.split(")").map(|s| s.trim()).collect();
            
            (x[0].to_string(), x[1].to_string())
        })
        .collect()
}

fn find_children<'a>(depth: u64, parent: &str, input: &'a [(String, String)]) -> Vec<(&'a str, u64)> {
    input
        .iter()
        .filter(|(x, _)| x == &parent)
        .map(|(_, y)| (y.as_str(), depth + 1))
        .collect()
}

fn find_parent<'a>(child: &str, input: &'a [(String, String)]) -> Option<&'a str> {
    input
        .iter()
        .find(|(_, y)| y == &child)
        .and_then(|(x, _)| Some(x.as_ref()))
}

fn get_chain<'a>(origin: &str, input: &'a [(String, String)]) -> Vec<&'a str> {
    let mut parent = origin;
    
    std::iter::from_fn(move ||
        find_parent(parent, input).and_then(|p| {
            parent = p;
            Some(p)
        })
    )
    .collect()
}

fn count_until(target: &str, chain: &[&str]) -> u64 {
    chain
        .iter()
        .take_while(|x| x != &&target)
        .count() as u64
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[(String, String)]) -> u64 {
    let mut candidates = find_children(0, "COM", input);
    let mut orbits = 0;
    
    while candidates.len() != 0 {
        let (c, depth) = candidates.pop().unwrap();
        
        candidates.extend(find_children(depth, c, input));
        orbits += depth;
    }
    
    orbits
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[(String, String)]) -> u64 {
    let you_chain = get_chain("YOU", input);
    let san_chain = get_chain("SAN", input);
    let common = you_chain
        .iter()
        .find(|s1|
            san_chain
                .iter()
                .any(|s2| s1 == &s2)
        )
        .unwrap();
    
    count_until(common, &you_chain) + count_until(common, &san_chain)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ex1_p1() {
        assert_eq!(solve_part1(&input_gen("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")), 42);
    }
    
    #[test]
    fn ex1_p2() {
        assert_eq!(solve_part2(&input_gen("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN")), 4);
    }
}