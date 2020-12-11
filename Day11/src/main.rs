use std::fs;
use std::fmt;
use std::time::Instant;

const NEIGHBORS : &'static [(i32, i32); 8] = &[(1,  -1), (1,  0), (1,  1),
                                               (0,  -1),          (0,  1),
                                               (-1, -1), (-1, 0), (-1, 1)];

#[derive(Clone)]
struct Map {
    data : Vec<u8>,
    lines : usize,
    line_len : usize
}

impl Map {
    fn pos(&self, i : usize, j : usize) -> usize {
        (i * self.line_len) + j
    }

    fn get(&self, i : usize, j : usize) -> Option<u8> {
        if (0..self.lines).contains(&i) && (0..self.line_len).contains(&j) {
            let pos = self.pos(i, j);
            return Some(self.data[pos]);
        }
        None
    }

    fn total(&self) -> u32 {
        let mut count = 0;
        for &c in &self.data {
            if c == b'#' {
                count += 1;
            }
        }
        count
    }

    fn set(&mut self, i : usize, j : usize, value : u8) {
        let pos = self.pos(i, j);
        self.data[pos] = value;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, &c) in self.data.iter().enumerate() {
            if i > 0 && (i % self.line_len) == 0 {
                write!(f, "\n").unwrap();
            }
            write!(f, "{}", c as char).unwrap();
        }
        write!(f, "\n")
    }
}

fn get_line_len(map : &Vec<u8>) -> usize {
    for i in 0..(map.len()) {
        if map[i] == b'\n' {
            return i;
        }
    }
    unreachable!()
}

fn read_input() -> Map {
    let data : Vec<u8> = fs::read("assets/input.txt").unwrap();
    let line_len = get_line_len(&data);
    let mut new_data : Vec<u8> = Vec::new();
    for c in data {
        if c != b'\n' {
            new_data.push(c);
        }
    }
    let lines = new_data.len() / line_len;
    Map{data: new_data, lines, line_len}
}

fn count_occupied_seats(map : &Map, i: usize, j : usize) -> u32 {
    let mut count : u32 = 0;
    for (di, dj) in NEIGHBORS {
        let ni = (i as i32) + di;
        let nj = (j as i32) + dj;
        if let Some(b'#') = map.get(ni as usize, nj as usize) {
            count += 1;
        }
    }
    count
}

fn get_new_cell(map : &Map, i : usize, j : usize) -> (bool, u8) {
    let cell = map.get(i, j).unwrap();
    if cell == b'L' {
        if count_occupied_seats(map, i, j) == 0 {
            return (true, b'#');
        }
    } else if cell == b'#' {
        if count_occupied_seats(map, i, j) >= 4 {
            return (true, b'L');
        }
    }
    (false, cell)
}

fn next_map(map : &mut Map, new_map : &mut Map) -> bool {
    let mut changed = false;
    for i in 0..map.lines {
        for j in 0..map.line_len {
            let (changed_, cell) = get_new_cell(&map, i, j);
            changed = changed || changed_;
            new_map.set(i, j, cell);
        }
    }
    changed
}

fn part1(original_map : &Map) {
    let mut map = original_map.clone();
    let mut new_map = original_map.clone();
    loop {
        if !next_map(&mut map, &mut new_map) {
            println!("{}", map.total());
            break;
        }
        if !next_map(&mut new_map, &mut map) {
            println!("{}", map.total());
            break;
        }
    }
}

fn count_occupied_seats2(map : &Map, i: usize, j : usize) -> u32 {
    let mut count : u32 = 0;
    for (di, dj) in NEIGHBORS {
        let mut ni = (i as i32) + di;
        let mut nj = (j as i32) + dj;
        loop {
            match map.get(ni as usize, nj as usize) {
                Some(c) => {
                    if c == b'#' {
                        count += 1;
                        break;
                    } else if c == b'L' {
                        break;
                    }
                },
                None => break
            }
            ni += di;
            nj += dj;
        }
    }
    count
}

fn get_new_cell2(map : &Map, i : usize, j : usize) -> (bool, u8) {
    let cell = map.get(i, j).unwrap();
    if cell == b'L' {
        if count_occupied_seats2(map, i, j) == 0 {
            return (true, b'#');
        }
    } else if cell == b'#' {
        if count_occupied_seats2(map, i, j) >= 5 {
            return (true, b'L');
        }
    }
    (false, cell)
}

fn next_map2(map : &mut Map, new_map : &mut Map) -> bool {
    let mut changed = false;
    for i in 0..map.lines {
        for j in 0..map.line_len {
            let (changed_, cell) = get_new_cell2(&map, i, j);
            changed = changed || changed_;
            new_map.set(i, j, cell);
        }
    }
    changed
}

fn part2(original_map : &Map) {
    let mut map = original_map.clone();
    let mut new_map = original_map.clone();
    loop {
        if !next_map2(&mut map, &mut new_map) {
            println!("{}", map.total());
            break;
        }
        if !next_map2(&mut new_map, &mut map) {
            println!("{}", map.total());
            break;
        }
    }
}

fn main() {
    let now = Instant::now();
    let map = read_input();
    println!("Read input took: {:?}", now.elapsed());

    let now = Instant::now();
    part1(&map);
    println!("Part 1     took: {:?}", now.elapsed());

    let now = Instant::now();
    part2(&map);
    println!("Part 2     took: {:?}", now.elapsed());
}
