use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_integers(filename : &str) -> Vec<i32> {
    let f = File::open("assets/input.txt").unwrap();
    let br = BufReader::new(f);
    return br.lines().map(|line| line.unwrap().trim().parse().unwrap()).collect();
}

fn part1() {
    let arr : Vec<i32> = read_integers("assets/input.txt");
    let n = arr.len();
    for i in 1..n {
        for j in 1..n {
            if i != j && (arr[i] + arr[j]) == 2020 {
                println!("{} {} {}", arr[i], arr[j], arr[i] * arr[j]);
                return;
            }
        }
    }
}

fn part2() {
    let arr : Vec<i32> = read_integers("assets/input.txt");
    let n = arr.len();
    for i in 1..n {
        for j in (i+1)..n {
            for k in (j+1)..n {
                if (i < j && j < k) && ((arr[i] + arr[j] + arr[k]) == 2020) {
                    println!("{} {} {} {}", arr[i], arr[j], arr[k], arr[i] * arr[j] * arr[k]);
                    return;
                }
            }
        }
    }
}

fn main() {
    part1();
    part2();
}