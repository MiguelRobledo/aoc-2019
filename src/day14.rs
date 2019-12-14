use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

pub struct Recipe {
    amount: u64,
    required: Vec<(String, u64)>
}

fn convert_to_ore(name: &str, amount: u64, recipes: &HashMap<String, Recipe>, leftovers: &mut HashMap<String, u64>) -> u64 {
    if let Some(recipe) = recipes.get(name) {
        let in_storage = *leftovers.entry(name.to_string()).or_insert(0);
        
        let (leftover, needed) = if in_storage < amount {
            (0, amount - in_storage)
        } else {
            (in_storage - amount, 0)
        };
        
        let iterations = (needed + recipe.amount - 1) / recipe.amount;
        let extra      = recipe.amount * iterations - needed;
        
        leftovers.insert(name.to_string(), leftover + extra);
        
        recipe.required
            .iter()
            .map(|i| convert_to_ore(&i.0, i.1 * iterations, recipes, leftovers))
            .sum()
    } else {
        amount
    }
}

fn calc_fuel(mut ore: u64, recipes: &HashMap<String, Recipe>, leftovers: &mut HashMap<String, u64>) -> u64 {
    let mut amount = 1000000;
    let mut fuel = 0;
    
    loop {
        let required = convert_to_ore("FUEL", amount, recipes, leftovers);
        
        if required > ore {
            if amount == 1 {
                break fuel;
            } else {
                amount /= 10;
            }
        } else {
            ore  -= required;
            fuel += amount;
        }
    }
}

#[aoc_generator(day14)]
pub fn input_gen(input: &str) -> HashMap<String, Recipe> {
    fn parse_product(s: &str) -> (String, u64) {
        let mut it = s.trim().split(' ');
        let amount = u64::from_str_radix(it.next().unwrap(), 10).unwrap();
        let name   = it.next().unwrap().to_string();
        
        (name, amount)
    }
    
    input
        .trim()
        .lines()
        .map(|l| {
            let mut it   = l.split("=>");
            let required = it.next().unwrap();
            let product  = it.next().unwrap();
            
            let (name, amount) = parse_product(product);
            let required = required
                .split(',')
                .map(|s| parse_product(s))
                .collect();
            
            (name, Recipe { amount, required })
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &HashMap<String, Recipe>) -> u64 {
    convert_to_ore("FUEL", 1, input, &mut HashMap::new())
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &HashMap<String, Recipe>) -> u64 {
    calc_fuel(1000000000000, input, &mut HashMap::new())
}
