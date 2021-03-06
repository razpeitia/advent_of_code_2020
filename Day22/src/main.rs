use std::fs;
use std::io;
use std::time::Instant;
use std::collections::{VecDeque, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn part1(deck1 : &mut VecDeque<u64>, deck2 : &mut VecDeque<u64>) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();
        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else if c2 > c1 {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
        else {
            unreachable!()
        }
    }
    let deck = if !deck1.is_empty() { deck1 } else { deck2 };
    let ans : u64 = deck.iter().rev().enumerate().map(|(i, &v)| ((i+1) as u64) * v).sum();
    println!("{}", ans);
}

fn hash_vec(vec : &VecDeque<u64>) -> u64 {
    let mut hash = DefaultHasher::new();
    vec.hash(&mut hash);
    hash.finish()
}

fn recurse_game(deck1 : &mut VecDeque<u64>, deck2 : &mut VecDeque<u64>) -> bool {
    let mut cache : HashSet<u64> = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        let h1 = hash_vec(&deck1);
        let h2 = hash_vec(&deck2);
        if cache.contains(&h1) && cache.contains(&h2) {
            // Player 1 wins
            return true;
        }
        cache.insert(h1);
        cache.insert(h2);
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if (c1 as usize) <= deck1.len() && (c2 as usize) <= deck2.len() {
            let mut d1 : VecDeque<u64> = deck1.iter().copied().take(c1 as usize).collect();
            let mut d2 : VecDeque<u64> = deck2.iter().copied().take(c2 as usize).collect();

            let result = recurse_game(&mut d1, &mut d2);
            if result {
                deck1.push_back(c1);
                deck1.push_back(c2);
            } else {
                deck2.push_back(c2);
                deck2.push_back(c1);

            }
            continue;
        }
        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else if c2 > c1 {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
        else {
            unreachable!()
        }
    }
    !deck1.is_empty()
}

fn part2(deck1 : &mut VecDeque<u64>, deck2 : &mut VecDeque<u64>) {
    let p1_wins = recurse_game(deck1, deck2);
    if p1_wins {
        let ans : u64 = deck1.iter().rev().enumerate().map(|(i, &v)| ((i+1) as u64) * v).sum();
        println!("{}", ans);
    } else {
        let ans : u64 = deck2.iter().rev().enumerate().map(|(i, &v)| ((i+1) as u64) * v).sum();
        println!("{}", ans);
    };
}

fn main() -> Result<(), io::Error> {
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt")?;
    let parts : Vec<&str> = data.split("\n\n").collect();
    let deck1 : VecDeque<u64> = parts[0].lines().filter(|&x| x.parse::<u64>().is_ok()).map(|x| x.parse().unwrap()).collect();
    let deck2 : VecDeque<u64> = parts[1].lines().filter(|&x| x.parse::<u64>().is_ok()).map(|x| x.parse().unwrap()).collect();
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&mut deck1.clone(), &mut deck2.clone());
    println!("Part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    part2(&mut deck1.clone(), &mut deck2.clone());
    println!("Part 2     took {:?}", now.elapsed());
    Ok(())
}
