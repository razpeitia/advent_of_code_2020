use std::fs;
use std::str;
use std::time::Instant;

fn read_input() -> Vec<u32> {
    let mut data = fs::read("assets/input.txt").unwrap();
    let n = data.len();
    for i in 0..n {
        match data[i] {
            b'B' | b'R' => data[i] = b'1',
            b'F' | b'L' => data[i] = b'0',
            _ => {}
        }
    }
    let n = data.len();
    let mut i = 0;
    let mut seats : Vec<u32> = Vec::new();
    while i < n {
        let row = u32::from_str_radix(str::from_utf8(&data[i..(i+7)]).unwrap(), 2).unwrap();
        let col = u32::from_str_radix(str::from_utf8(&data[(i+7)..(i+10)]).unwrap(), 2).unwrap();
        seats.push((row * 8) + col);
        i += 11;
    }
    seats
}

fn part1(seats : &Vec<u32>) {
    println!("{:?}", seats.iter().max().unwrap());
}

fn part2(mut seats : Vec<u32>) {
    seats.sort();
    let mut previous = seats[0];
    for &seat in seats[1..].iter() {
        if (previous + 1) != seat {
            println!("your seat {}", previous+1);
            break
        }
        previous = seat;
    }
}

fn main() {
    let now = Instant::now();
    let data : Vec<u32> = read_input();
    println!("Read took   {:?}", now.elapsed());

    let now = Instant::now();
    part1(&data);
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    part2(data);
    println!("Part 2 took {:?}", now.elapsed());
}