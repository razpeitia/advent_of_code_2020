use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Input {
    low : u32,
    high : u32,
    c : char,
    password: String
}

impl Input {
    fn is_valid(&self) -> bool {
        let count  = self.password.matches(self.c).count() as u32;
        count >= self.low && count <= self.high
    }

    fn is_valid2(&self) -> bool {
        let low_char  = self.password.chars().nth((self.low - 1) as usize).unwrap();
        let high_char = self.password.chars().nth((self.high - 1) as usize).unwrap();
        (low_char == self.c) ^ (high_char == self.c)
    }
}

fn read_input() -> Vec<Input> {
    let f = File::open("assets/input.txt").unwrap();
    let br = BufReader::new(f);
    return br.lines().map(|line| {
        let line = line.unwrap();
        // 1-3 a: abcde
        let parts : Vec<&str> = line.split(":").collect();
        let rest = parts[0];
        let password = parts[1].trim().to_string();
        let parts : Vec<&str> = rest.split(" ").collect();
        let rest = parts[0];
        let c  = parts[1].chars().nth(0).unwrap();
        let parts : Vec<&str> = rest.split("-").collect();
        let low : u32 = parts[0].parse().unwrap();
        let high: u32 = parts[1].parse().unwrap();
        Input{low, high, c, password}
    }).collect();
}

fn part1() {
    let arr : Vec<Input> = read_input();
    let mut count : u32 = 0;
    for input in arr {
        if input.is_valid() {
            count += 1;
        }
    }
    println!("Total {}", count);
}

fn part2() {
    let arr : Vec<Input> = read_input();
    let mut count : u32 = 0;
    for input in arr {
        if input.is_valid2() {
            count += 1;
        }
    }
    println!("Total {}", count);
}

fn main() {
    part1();
    part2();
}
