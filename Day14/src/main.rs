use std::fs;
use std::collections::HashMap;
use std::time::Instant;

fn part1(input : &String) {
    let mut mem : HashMap<u64, u64> = HashMap::new();
    let mut mask : &str = "";
    input.lines().for_each(|line| {
        if line.starts_with("mask = ") {
            let parts: Vec<&str> = line.split(" = ").collect();
            mask = parts[1];
        } else {
            let parts : Vec<&str> = line.split(" = ").collect();
            let rec : u64 = parts[0].strip_prefix("mem[").unwrap().strip_suffix("]").unwrap().parse().unwrap();
            let val : u64 = parts[1].parse().unwrap();

            let mut ans : u64 = 0;
            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    'X' => ans |= ((val >> i) & 1) << i,
                    '1' => ans |= 1 << i,
                    '0' => {}
                    _ => unreachable!()
                }
            }
            mem.insert(rec, ans);
        }
    });
    println!("{}", mem.values().sum::<u64>());
}

fn part2(input : &String) {
    let mut mem : HashMap<u64, u64> = HashMap::new();
    let mut mask : &str = "";
    input.lines().for_each(|line| {
        if line.starts_with("mask = ") {
            let parts: Vec<&str> = line.split(" = ").collect();
            mask = parts[1];
        } else {
            let parts : Vec<&str> = line.split(" = ").collect();
            let rec : u64 = parts[0].strip_prefix("mem[").unwrap().strip_suffix("]").unwrap().parse().unwrap();
            let val : u64 = parts[1].parse().unwrap();
            let mut addresses : Vec<u64> = Vec::new();
            addresses.push(0);
            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    'X' => {
                        let n = addresses.len();
                        for x in 0..n {
                            addresses.push(addresses[x] | (1 << i));
                        }
                    },
                    '1' => {
                        for x in 0..addresses.len() {
                            addresses[x] |= 1 << i
                        }
                    },
                    '0' => {
                        for x in 0..addresses.len() {
                            addresses[x] |= ((rec >> i) & 1) << i
                        }
                    }
                    _ => unreachable!()
                }
            }

            for address in addresses {
                mem.insert(address, val);
            }
        }
    });
    println!("{}", mem.values().sum::<u64>());
}

fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt").unwrap();
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&data);
    println!("Part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    part2(&data);
    println!("Part 2     took {:?}", now.elapsed());
}
