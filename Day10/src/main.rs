use std::fs;
use std::collections::HashMap;
use std::time::Instant;

fn read_input() -> Vec<u32> {
    let data = fs::read_to_string("assets/input.txt").unwrap();
    let mut numbers : Vec<u32> = data.lines().map(|line| line.parse().unwrap()).collect();
    numbers.push(0);
    let max = numbers.iter().max().unwrap() + 3;
    numbers.push(max);
    numbers.sort();
    numbers
}

fn part1(numbers : &Vec<u32>) {
    let n =  numbers.len();
    let mut count = [0u32; 3];
    for i in 1..n {
        let diff = numbers[i] - numbers[i-1];
        if diff == 1 || diff == 2 || diff == 3 {
            count[(diff-1) as usize] += 1;
        } else {
            panic!("Invalid difference {}", diff);
        }
    }
    println!("{}", count[0] * count[2]);
}

fn children(numbers : &Vec<u32>, start : usize) -> usize {
    let n = numbers.len();
    if (start+3) < n && (numbers[start+3] - numbers[start]) <= 3 {
        return start+3;
    }
    else if (start+2) < n && (numbers[start+2] - numbers[start]) <= 3 {
        return start+2;
    }
    start+1
}

fn solve(numbers : &Vec<u32>, start : usize, cache : &mut HashMap<u32, u64>) -> u64 {
    if start == (numbers.len()-1) {
        return 1;
    }
    let end = children(&numbers, start);
    let mut sum : u64 = 0;
    for i in (start+1)..=end {
        let k = &numbers[i];
        if !cache.contains_key(k) {
            let ans = solve(numbers, i, cache);
            cache.insert(*k, ans);
        }
        sum += cache[k];
    }
    sum
}

fn part2(numbers : &Vec<u32>) {
    let mut cache : HashMap<u32, u64> = HashMap::new();
    let ans = solve(&numbers, 0, &mut cache);
    println!("{:?}", ans);
}

fn main() {
    let now = Instant::now();
    let numbers = read_input();
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&numbers);
    println!("Part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    part2(&numbers);
    println!("Part 2     took {:?}", now.elapsed());
}

