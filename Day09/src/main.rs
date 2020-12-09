use std::fs;
use std::time::Instant;

fn read_input() -> Vec<u64> {
    let data = fs::read_to_string("assets/input.txt").unwrap();
    let mut numbers : Vec<u64> = Vec::new();
    for line in data.lines() {
        numbers.push(line.parse().unwrap());
    }
    numbers
}

fn is_valid(numbers : &Vec<u64>, start : usize, preamble : usize, target : u64) -> bool {
    for i in 0..(preamble-1) {
        for j in (i+1)..preamble {
            if (numbers[start+i-preamble] + numbers[start+j-preamble]) == target {
                return true;
            }
        }
    }
    false
}

fn part1(numbers : &Vec<u64>, preamble : usize) -> u64 {
    let n = numbers.len();
    for start in preamble..=(n-preamble) {
        if !is_valid(numbers, start, preamble, numbers[start]) {
            println!("{}", numbers[start]);
            return numbers[start];
        }
    }
    panic!("It should have found a solution!");
}

fn part2(numbers : &Vec<u64>, target : u64) {
    let mut sum : u64 = 0;
    let mut start : usize = 0;
    for (i, value) in numbers.iter().enumerate() {
        sum += value;

        if sum < target {
            continue;
        }

        if sum == target {
            let &min = numbers[start..=i].iter().min().unwrap();
            let &max = numbers[start..=i].iter().max().unwrap();
            println!("{}", min + max);
            return;
        }

        while sum > target {
            sum -= numbers[start];
            start += 1;
        }
    }
}

const PREAMBLE: usize = 25;

fn main() {
    let now = Instant::now();
    let numbers = read_input();
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    let invalid_number = part1(&numbers, PREAMBLE);
    println!("Part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    part2(&numbers, invalid_number);
    println!("Part 2     took {:?}", now.elapsed());
}
