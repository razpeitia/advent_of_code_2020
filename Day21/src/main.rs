use std::fs;
use std::io;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::time::Instant;

fn main() -> Result<(), io::Error> {
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt")?;
    let mut table: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredient_map : HashMap<&str, u32> = HashMap::new();
    for line in data.lines() {
        let parts : Vec<&str> = line.split(" (contains ").collect();
        let allergens : Vec<&str> = parts[1].strip_suffix(")").unwrap().split(", ").collect();
        let ingredients : Vec<&str> = parts[0].split_whitespace().collect();
        for &ingredient in ingredients.iter() {
            *ingredient_map.entry(ingredient).or_insert(0) += 1;
        }
        let set_ingredients : HashSet<&str> = HashSet::from_iter(ingredients.iter().cloned());
        for &allergen in allergens.iter() {
            match table.get_mut(allergen) {
                Some(v) => {
                    let intersection : Vec<&str> = v.intersection(&set_ingredients).map(|&x| x).collect();
                    v.clear();
                    v.extend(intersection.iter().cloned());
                }
                None => {
                    table.insert(allergen, set_ingredients.clone());
                }
            }
        }
    }
    println!("Read input took {:?}", now.elapsed());

    // Part 1
    let now = Instant::now();
    let mut allergen_map : HashMap<&str, &str> = HashMap::new();
    let mut taken : HashSet<&str> = HashSet::new();
    let mut all_allergens : Vec<&str> = table.keys().map(|&x| x).collect();
    while allergen_map.len() != table.len() {
        for &allergen in all_allergens.iter() {
            let v : Vec<&str> = table.get(allergen).unwrap().difference(&taken).map(|&x| x).collect();
            if v.len() == 1 {
                for &ingredient in v.iter() {
                    allergen_map.insert(allergen, ingredient);
                    taken.insert(ingredient);
                }
            }
        }
    }

    let ans : u32 = ingredient_map.iter().filter(|(&ingredient, _)| !taken.contains(ingredient)).map(|(_, &v)| v).sum();
    println!("{}", ans);
    println!("Part 1     took {:?}", now.elapsed());

    // Part 2
    let now = Instant::now();
    all_allergens.sort();
    let bad_ingredients : Vec<&str> = all_allergens.iter().map(|&x| *allergen_map.get(x).unwrap()).collect();
    println!("{}", bad_ingredients.join(","));
    println!("Part 2     took {:?}", now.elapsed());
    Ok(())
}
