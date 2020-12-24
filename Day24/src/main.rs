use std::fs;
use std::io;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const DIRECTIONS : &[(&'static str, (i32, i32))] = &[
    ("nw", ( 1, -1)),
    ("ne", ( 1,  0)),
    ("sw", (-1,  0)),
    ("se", (-1,  1)),
    ("w",  ( 0, -1)),
    ("e",  ( 0,  1)),
];

fn mark_tile(grid : &mut HashMap<(i32, i32), bool>, line: &str, x : &mut i32, y : &mut i32) {
    let mut line = line.to_string();
    *x = 0; *y = 0;
    while !line.is_empty() {
        for (direction, (dy, dx)) in DIRECTIONS {
            if line.starts_with(*direction) {
                // println!("Moving from {} ({}, {}) to ({}, {})", direction, *x, *y, *x + *dx, *y + *dy);
                (0..(direction.len())).for_each(|_| { line.remove(0); } );
                *x += *dx;
                *y += *dy;
            }
        }
    }

    let &mut v = grid.entry((*x, *y)).or_insert(true );
    grid.insert((*x, *y), !v);
}

fn part1(data : &String) {
    let mut grid : HashMap<(i32, i32), bool> = HashMap::new();
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    data.lines().for_each(|line| mark_tile(&mut grid, line, &mut x, &mut y));
    let ans = grid.iter().filter(|(_k, &v)| v == false).count();
    println!("{}", ans);
}

fn count_black(grid : &HashMap<(i32, i32), bool>, x : i32, y : i32) -> u32 {
    let mut sum : u32 = 0;
    for (_direction, (dy, dx)) in DIRECTIONS {
        let white = *grid.get(&(x + *dx, y + *dy)).unwrap_or(&true);
        if !white {
            sum += 1;
        }
    }
    sum
}

fn flip_all(grid : &mut HashMap<(i32, i32), bool>) {
    let mut need_check : HashSet<(i32, i32)> = HashSet::new();
    for ((x, y), _v) in grid.iter() {
        need_check.insert((*x, *y));
        for (_direction, (dy, dx)) in DIRECTIONS {
            need_check.insert((*x+*dx, *y+*dy));
        }
    }
    let mut need_flip : HashSet<(i32, i32)> = HashSet::new();
    for (x, y) in need_check.iter() {
        let blacks = count_black(&grid, *x, *y);
        let is_white = *grid.get(&(*x, *y)).unwrap_or(&true);
        if is_white && blacks == 2 {
            need_flip.insert((*x, *y));
        } else if !is_white && (blacks == 0 || blacks > 2) {
            need_flip.insert((*x, *y));
        }
    }
    for (x, y) in need_flip {
        let &mut v = grid.entry((x, y)).or_insert(true );
        grid.insert((x, y), !v);
    }
}

fn part2(data : &String) {
    let mut grid : HashMap<(i32, i32), bool> = HashMap::new();
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    data.lines().for_each(|line| mark_tile(&mut grid, line, &mut x, &mut y));
    for _i in 1..=100 {
        flip_all(&mut grid);
    }
    let ans = grid.iter().filter(|(_k, &v)| v == false).count();
    println!("{}", ans);
}

fn main() -> Result<(), io::Error> {
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt")?;
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&data);
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    part2(&data);
    println!("Part 2 took {:?}", now.elapsed());
    Ok(())
}
