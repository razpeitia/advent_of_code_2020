use std::fs;
use std::io;

fn mult_mod(a : u64, b : u64, n : u64) -> u64 {
    return ((a % n) * (b % n)) % n;
}

fn search_len(n : u64) -> u64 {
    let mut i = 1;
    let mut x : u64 = 1;
    loop {
        x = mult_mod(x, 7, 20201227);
        if x == n {
            return i;
        }
        i += 1;
    }
}

fn get_private_key(subject : u64, len : u64) -> u64 {
    let mut i = 1;
    let mut x : u64 = 1;
    for _ in 1..=len {
        x = mult_mod(x, subject, 20201227);
        i += 1;
    }
    x
}

fn main() -> Result<(), io::Error> {
    let numbers : Vec<u64> = fs::read_to_string("assets/input.txt")?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let n1 = numbers[0];
    let len1 = search_len(n1);
    let n2 = numbers[1];
    let len2 = search_len(n2);
    println!("{}", get_private_key(n1, len2));
    println!("{}", get_private_key(n2, len1));
    Ok(())
}
