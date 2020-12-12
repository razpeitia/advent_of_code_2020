use std::fs;
use std::time::Instant;

const DIRECTIONS : &[&'static str; 4] = &["E", "N", "W", "S"];

fn move_(x: &mut i32, y: &mut i32, d : &str, val : i32) {
    match d {
        "N" => *y += val,
        "S" => *y -= val,
        "W" => *x -= val,
        "E" => *x += val,
        _ => unreachable!()
    }
}

fn part1(instructions : &Vec<(&str, i32)>) {
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    let mut d : &str = "E";
    for &instruction in instructions {
        match instruction {
            ("L", val) => {
                let i = DIRECTIONS.iter().position(|&c| c == d).unwrap();
                d = DIRECTIONS[(((val / 90) as usize) + i) % 4]
            }
            ("R", val) => {
                let i : i32 = DIRECTIONS.iter().position(|&c| c == d).unwrap() as i32;
                let x = (((i - (val / 90)) % 4) + 4) % 4;
                d = DIRECTIONS[x as usize]
            }
            ("F", val) => move_(&mut x, &mut y, d, val),
            (c , val) => move_(&mut x, &mut y, c, val)
        }
    }
    println!("{}", x.abs() + y.abs());
}

fn move2(x: &mut i32, y: &mut i32, d : &str, val : i32) {
    match d {
        "N" => *y += val,
        "S" => *y -= val,
        "W" => *x -= val,
        "E" => *x += val,
        _ => unreachable!()
    }
}

fn part2(instructions : &Vec<(&str, i32)>) {
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    let mut wx : i32 = 10;
    let mut wy : i32 = 1;
    for &instruction in instructions {
        match instruction {
            ("L", val) => {
                for _ in 0..(val/90) {
                    let owx = wx;
                    let owy = wy;
                    wx = -owy;
                    wy = owx;
                }
            }
            ("R", val) => {
                for _ in 0..(val/90) {
                    let owx = wx;
                    let owy = wy;
                    wx = owy;
                    wy = -owx;
                }
            }
            ("F", val) => {
                move2(&mut x, &mut y, "E", val * wx);
                move2(&mut x, &mut y, "N", val * wy)
            },
            (c , val) => move2(&mut wx, &mut wy, c, val)
        }
    }
    println!("{}", x.abs() + y.abs());
}

fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt").unwrap();
    let instructions : Vec<(&str, i32)> = data.lines().map(|line| (&line[0..1], line[1..].parse().unwrap())).collect();
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&instructions);
    println!("Part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    part2(&instructions);
    println!("Part 2     took {:?}", now.elapsed());
}
