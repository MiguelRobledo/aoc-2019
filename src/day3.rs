use aoc_runner_derive::{aoc, aoc_generator};

fn is_between(n: i64, a: i64, b:i64) -> bool {
    if a < b {
        n >= a && n <= b
    } else {
        n >= b && n <= a
    }
}

#[derive(Debug)]
pub struct LineStruct {
    x: i64,
    y: i64,
    len: i64
}

#[derive(Debug)]
pub enum Line {
    Vertical(LineStruct),
    Horizontal(LineStruct)
}

impl Line {
    fn new(is_horizontal: bool, x: i64, y: i64, len: i64) -> Self {
        if is_horizontal {
            Line::Horizontal(LineStruct { x, y, len })
        } else {
            Line::Vertical(LineStruct { x, y, len })
        }
    }
    
    fn intersect(&self, other: &Self) -> Option<(i64, i64)> {
        let get_intersection = |h: &LineStruct, v: &LineStruct| {
            if is_between(h.y, v.y, v.y + v.len)
            && is_between(v.x, h.x, h.x + h.len) {
                Some((v.x, h.y))
            } else { None }
        };
        
        match self {
            Line::Horizontal(h) => match other {
                Line::Horizontal(_) => None,
                Line::Vertical(v) => get_intersection(h, v)
            },
            Line::Vertical(v) => match other {
                Line::Horizontal(h) => get_intersection(h, v),
                Line::Vertical(_) => None
            },
        }
    }
    
    fn steps_to(&self, p: Option<(i64, i64)>) -> i64 {
        let (lx, ly, len) = match self {
            Line::Horizontal(l) => (l.x, l.y, l.len),
            Line::Vertical(l) => (l.y, l.x, l.len)
        };
        
        if let Some((x, y)) = p {
            let (x, y) = match self {
                Line::Horizontal(_) => (x, y),
                Line::Vertical(_) => (y, x)
            };
            
            if y == ly && is_between(x, lx, lx + len) {
                x - lx
            } else {
                len
            }
        } else {
            len
        }
        .abs()
    }
}

#[aoc_generator(day3)]
pub fn input_gen(input: &str) -> Vec<Vec<Line>> {
    input
        .trim()
        .split('\n')
        .map(|c| c
            .trim()
            .split(',')
            .scan((0, 0), |(x, y), s| {
                let mut it = s.chars();
                let dir = it.next().unwrap();
                let v = i64::from_str_radix(it.as_str(), 10).unwrap();
                
                let (is_h, v) = match dir {
                    'U' => (false,  v),
                    'D' => (false, -v),
                    'L' => (true,  -v),
                    'R' => (true,   v),
                    _ => panic!("bad input")
                };
                
                let res = Line::new(is_h, *x, *y, v);
                
                if is_h {
                    *x += v;
                } else {
                    *y += v;
                }
                
                Some(res)
            })
            .collect()
        )
        .collect()
}


#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<Line>]) -> i64 {
    input[0]
        .iter()
        .filter_map(|l1| input[1]
            .iter()
            .filter_map(move |l2| l1
                .intersect(&l2)
                .filter(|c| *c != (0, 0))
                .map(|(x, y)| x.abs() + y.abs())
            )
            .min()
        )
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<Line>]) -> i64 {
    input[0]
        .iter()
        .scan(0, move |steps1, l1| {
            let res = input[1]
                .iter()
                .scan(0, |steps2, l2| {
                    let cross = l1.intersect(&l2);
                    let res = if cross.map_or(false, |v| v != (0, 0)) {
                        Some(*steps1 + *steps2 + l1.steps_to(cross) + l2.steps_to(cross))
                    } else { None };
                    
                    *steps2 += l2.steps_to(None);
                    
                    Some(res)
                })
                .filter_map(|x| x)
                .min();
            
            *steps1 += l1.steps_to(None);
            
            Some(res)
        })
        .filter_map(|x| x)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ex1_p1() {
        assert_eq!(solve_part1(&input_gen("R8,U5,L5,D3\nU7,R6,D4,L4")), 6);
    }
    
    #[test]
    fn ex2_p1() {
        assert_eq!(solve_part1(&input_gen("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")), 159);
    }
    
    #[test]
    fn ex3_p1() {
        assert_eq!(solve_part1(&input_gen("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")), 135);
    }
    
    #[test]
    fn ex1_p2() {
        assert_eq!(solve_part2(&input_gen("R8,U5,L5,D3\nU7,R6,D4,L4")), 30);
    }
    
    #[test]
    fn ex2_p2() {
        assert_eq!(solve_part2(&input_gen("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")), 610);
    }
    
    #[test]
    fn ex3_p2() {
        assert_eq!(solve_part2(&input_gen("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")), 410);
    }
}
