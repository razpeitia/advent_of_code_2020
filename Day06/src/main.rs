use std::fs;
use std::convert::TryInto;
use std::time::Instant;

fn count_az(group : &str) -> u32 {
    let mut table = [0u32;26];
    for c in group.chars() {
        match c {
            'a'..='z' => table[122 - (c as usize)] = 1,
            _ => {}
        }
    }
    table.iter().sum()
}

fn part1() {
    let a = fs::read_to_string("assets/input.txt").expect("Unable to open file");
    let records : Vec<&str> = a.split("\n\n").collect();
    let ans : u32 = records.iter().map(|record| count_az(record)).sum();
    println!("{}", ans);
}

fn count_az2(group : &str) -> u32 {
    let n  = group.lines().count().try_into().unwrap();
    let mut table = [0u32;26];
    for c in 'a'..='z' {
        let s : u32 = group.lines().map(|line| line.contains(c) as u32).sum();
        if s == n {
            table[122 - (c as usize)] = 1;
        }
    }
    table.iter().sum()
}

fn part2() {
    let a = fs::read_to_string("assets/input.txt").expect("Unable to open file");
    let records : Vec<&str> = a.split("\n\n").collect();
    let ans : u32 = records.iter().map(|record| count_az2(record)).sum();
    println!("{}", ans);
}

fn main() {
    let now = Instant::now();
    part1();
    println!("Part 1 took {:?}", now.elapsed());

    part2();
    println!("Part 2 took {:?}", now.elapsed());
}