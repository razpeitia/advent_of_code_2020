use std::collections::HashMap;
use std::time::Instant;

fn decrease_pos(pos : usize, n : usize) -> usize {
    let ans = (((pos as isize) - 1) % (n as isize) + (n as isize)) % (n as isize);
    ans as usize
}

fn part1() {
    let data = "739862541";
    let mut numbers : Vec<u32> = data.chars().map(|c| c as u32 - '0' as u32).collect();
    let mut sorted_numbers : Vec<u32> = numbers.clone();
    sorted_numbers.sort_unstable();
    let mut i = 0;
    let n = numbers.len();
    for _round in 1..=100 {
        let v = numbers[i];
        let s1 = numbers[(i + 1) % n];
        let s2 = numbers[(i + 2) % n];
        let s3 = numbers[(i + 3) % n];
        let mut dst_pos  = sorted_numbers.iter().position(|&x| x == v).unwrap();
        let mut dst = sorted_numbers[dst_pos];
        while dst == s1 || dst == s2 || dst == s3 || dst == v {
            dst_pos = decrease_pos(dst_pos, n);
            dst = sorted_numbers[dst_pos];
        }
        numbers.remove(numbers.iter().position(|x| *x == s1).unwrap());
        numbers.remove(numbers.iter().position(|x| *x == s2).unwrap());
        numbers.remove(numbers.iter().position(|x| *x == s3).unwrap());
        let dst_pos = numbers.iter().position(|x| *x == dst).unwrap();
        numbers.insert(dst_pos+1, s3);
        numbers.insert(dst_pos+1, s2);
        numbers.insert(dst_pos+1, s1);
        let ni = numbers.iter().position(|x| *x == v).unwrap();
        if i > ni {
            numbers.rotate_right(i-ni);
        } else if ni > i {
            numbers.rotate_left(ni-i);
        }
        i = (i + 1) % n;
    }
    let target = numbers.iter().position(|x| *x == 1).unwrap();
    for i in 0..(n-1) {
        print!("{}", numbers[(target + i + 1) % n]);
    }
    println!();
}

fn part2() {
    let data = "739862541";
    let mut numbers : Vec<u64> = data.chars().map(|c| c as u64 - '0' as u64).collect();
    let mut i : u64 = 10;
    while numbers.len() != 1000000 {
        numbers.push(i);
        i += 1;
    }
    let n = numbers.len();
    let mut next_cup : HashMap<u64, u64> = HashMap::new();
    for (i, &cup) in numbers.iter().enumerate() {
        next_cup.insert(cup, numbers[(i+1)%n]);
    }

    let mut cup = numbers[0];
    for _ in 0..10000000 {
        let a = next_cup[&cup];
        let b = next_cup[&a];
        let c = next_cup[&b];
        next_cup.insert(cup, next_cup[&c]);

        let mut dst = if cup == 1 { n as u64 } else { cup - 1 };
        while [a, b, c].contains(&(dst as u64)) {
            dst = if dst == 1 { n as u64 } else { dst - 1 };
        }

        let tmp = next_cup[&dst];
        next_cup.insert(dst, a);
        next_cup.insert(c, tmp);
        cup = next_cup[&cup];
    }
    let a = next_cup[&1];
    let b = next_cup[&a];
    println!("{}", a * b);
}

fn main() {
    let now = Instant::now();
    part1();
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    part2();
    println!("Part 2 took {:?}", now.elapsed());
}