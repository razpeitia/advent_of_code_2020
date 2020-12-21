use std::fs;
use std::io;
use std::collections::HashMap;

fn remove_ingredient(table : &mut HashMap<&str, HashMap<&str, u32>>, ingredient : &str) {
    for (_k, v) in table.iter_mut() {
        v.remove(ingredient);
    }
}

fn main() -> Result<(), io::Error> {
    let data = fs::read_to_string("assets/input.txt")?;
    let mut table: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    let mut ingredient_map : HashMap<&str, u32> = HashMap::new();
    for line in data.lines() {
        let parts : Vec<&str> = line.split(" (contains ").collect();
        let allergens : Vec<&str> = parts[1].strip_suffix(")").unwrap().split(", ").collect();
        let ingredients : Vec<&str> = parts[0].split_whitespace().collect();
        for &ingredient in ingredients.iter() {
            *ingredient_map.entry(ingredient).or_insert(0) += 1;
        }
        for &allergen in allergens.iter() {
            for &ingredient in ingredients.iter() {
                *table.entry(allergen).or_insert(HashMap::new()).entry(ingredient).or_insert(0) += 1;
            }
        }
    }

    // Part 1
    println!("{:?}", table);


    let mut allergen_map : HashMap<&str, &str> = HashMap::new();
    let allergens : Vec<&str> = table.keys().map(|&x| x).collect();
    while allergen_map.len() != table.len() {
        for &allergen in allergens.iter() {
            let v = table.get_mut(allergen).unwrap();
            if v.len() == 0 { continue; }
            if v.len() == 1 {
                let ingredients : Vec<&str> = v.keys().map(|&x| x).collect();
                let &ingredient = ingredients.first().unwrap();
                allergen_map.insert(allergen, ingredient);
                remove_ingredient(&mut table, ingredient);
            } else {
                let (&ingredient, &count) = v.iter().max_by_key(|(_ingredient, &count)| count).unwrap();
                if count == 1 { continue; }
                allergen_map.insert(allergen, ingredient);
                remove_ingredient(&mut table, ingredient);
            }
        }
    }
    for (_, &ingredient) in allergen_map.iter() {
        ingredient_map.remove(ingredient).unwrap();
    }
    let ans : u32 = ingredient_map.iter().map(|(_, &v)| v).sum();
    println!("{:}", ans);

    let mut allergens : Vec<&str> = allergen_map.keys().map(|&x| x).collect();
    allergens.sort();
    for &allergen in allergens.iter() {
        let &ingredient = allergen_map.get(allergen).unwrap();
        println!("{} {}", allergen, ingredient);
    }

    Ok(())
}
