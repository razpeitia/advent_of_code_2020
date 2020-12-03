use std::fs;
use std::time::Instant;

fn read_input() -> Vec<Vec<u8>> {
    let mut data : Vec<Vec<u8>> = Vec::new();
    let f = fs::read("assets/input.txt").expect("Unable to open file");
    let mut line : Vec<u8> = Vec::new();
    for c in f {
        if c == b'\n' {
            data.push(line.clone());
            line.clear();
        } else {
            line.push(c);
        }
    }
    if line.len() > 0 {
        data.push(line.clone());
        line.clear()
    }
    data
}

fn count(arr: &Vec<Vec<u8>>, r : usize, d : usize) -> u32 {
    let m = arr.len();
    let n = arr[0].len();
    let mut i : usize = 0;
    let mut j : usize = 0;
    let mut trees : u32 = 0;

    loop {
        j = (j + r) % n;
        i += d;
        if i >= m {
            break;
        }
        if arr[i][j] == b'#' {
            trees += 1
        }
    }
    trees
}

fn part1(arr : &Vec<Vec<u8>>) {
    let trees = count(arr, 3, 1);
    println!("{}", trees);
}

fn part2(arr : &Vec<Vec<u8>>) {
    let a1 = count(arr, 1, 1);
    let a2 = count(arr, 3, 1);
    let a3 = count(arr, 5, 1);
    let a4 = count(arr, 7, 1);
    let a5 = count(arr, 1, 2);
    println!("{}", a1 * a2 * a3 * a4 * a5);
}

fn main() {
    let now = Instant::now();
    let arr : Vec<Vec<u8>>  = read_input();
    println!("Reading took {:?}", now.elapsed());

    part1(&arr);
    println!("Part 1 took {:?}", now.elapsed());

    part2(&arr);
    println!("Part 2 took {:?}", now.elapsed());
}