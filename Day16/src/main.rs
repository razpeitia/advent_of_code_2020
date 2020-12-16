use std::fs;
use std::io;
use std::ops::RangeInclusive;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn part1(fields : &HashMap<&str, Vec<RangeInclusive<u32>>>, tickets : &Vec<Vec<u32>>) {
    let sum : u32 = tickets.iter().map(|ticket| {
        ticket.iter().filter(|&field| {
            !fields.iter().any(|(_, ranges)| ranges.iter().any(|range| range.contains(field)) )
        }).sum::<u32>()
    }).sum();
    println!("{}", sum);
}

fn part2(fields : &HashMap<&str, Vec<RangeInclusive<u32>>>, tickets : &Vec<Vec<u32>>, my_ticket : &Vec<u32>) {
    let mut valid_tickets : Vec<&Vec<u32>> = tickets.iter().filter(|ticket| {
        ticket.iter().all(|field| {
            fields.iter().any(|(_, ranges)| ranges.iter().any(|range| range.contains(field)) )
        })
    }).collect();
    valid_tickets.push(my_ticket);

    let mut possible_cols : HashMap<u32, HashSet<&str>> = HashMap::new();
    for (field, ranges) in fields.iter() {
        for pos in 0..my_ticket.len() {
            if valid_tickets.iter().map(|x| x[pos]).all(|x| ranges.iter().any(|range| range.contains(&x))) {
                possible_cols.entry(pos as u32).or_insert(HashSet::new()).insert(field);
            }
        }
    }

    while !possible_cols.values().all(|s| s.len() == 1) {
        let absolute_fields : HashSet<&str> = possible_cols
            .values()
            .filter(|s| s.len() == 1)
            .fold(HashSet::new(), |x, acc| acc.union(&x).map(|&x| x).collect());
        for (_a, b) in possible_cols.iter_mut() {
            if b.len() > 1 {
                for &absolute_field in absolute_fields.iter() {
                    b.remove(absolute_field);
                }
            }
        }
    }

    let lookup : HashMap<&str, u32> = possible_cols.iter().map(|(&a, b)| (*b.iter().nth(0).unwrap(), a)).collect();
    let ans : u64 = fields
        .keys()
        .filter(|x| x.starts_with("departure"))
        .map(|&x| {
            my_ticket[*lookup.get(x).unwrap() as usize] as u64
        })
        .product();
    println!("{}", ans);
}

fn main() -> Result<(), io::Error>{
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt")?;
    let parts : Vec<&str> = data.split("\n\n").collect();
    let mut fields : HashMap<&str, Vec<RangeInclusive<u32>>> = HashMap::new();

    for line in parts[0].lines() {
        let p : Vec<&str> = line.split(": ").collect();
        let r : Vec<RangeInclusive<u32>>= p[1].split(" or ").map(|x| {
            let s : Vec<&str> = x.split("-").collect();
            RangeInclusive::new(s[0].parse().unwrap(), s[1].parse().unwrap())
        }).collect();
        fields.insert(p[0], r);
    }

    let line = parts[1].lines().nth(1).unwrap();
    let my_ticket : Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();

    let tickets : Vec<Vec<u32>> = parts[2].lines().filter(|line| !line.ends_with(":")).map(|line| {
        line.split(",").map(|x| x.parse().unwrap()).collect()
    }).collect();
    println!("Read input took: {:?}", now.elapsed());

    let now = Instant::now();
    part1(&fields, &tickets);
    println!("Part 1     took: {:?}", now.elapsed());

    let now = Instant::now();
    part2(&fields, &tickets, &my_ticket);
    println!("Part 2     took: {:?}", now.elapsed());
    Ok(())
}
