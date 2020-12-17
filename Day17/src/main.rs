use std::fs;
use std::io;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x : i32,
    y : i32,
    z : i32
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point4D {
    x : i32,
    y : i32,
    z : i32,
    w : i32,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let mut points : Vec<Point> = Vec::new();
        for z in (self.z-1)..=(self.z+1) {
            for x in (self.x-1)..=(self.x+1) {
                for y in (self.y-1)..=(self.y+1) {
                    let point = Point{x, y, z};
                    if *self != point {
                        points.push(point);
                    }
                }
            }
        }
        points
    }
}

const MAX_SIZE : i32 = 13;
const MIN_SIZE : i32 = -13;

impl Point4D {
    fn neighbors(&self) -> Vec<Point4D> {
        let mut points : Vec<Point4D> = Vec::new();
        for w in (self.w-1)..=(self.w+1) {
            for z in (self.z - 1)..=(self.z + 1) {
                for x in (self.x - 1)..=(self.x + 1) {
                    for y in (self.y - 1)..=(self.y + 1) {
                        let point = Point4D { x, y, z , w};
                        if *self != point {
                            points.push(point);
                        }
                    }
                }
            }
        }
        points
    }
}

fn next_state1(graph1 : &HashMap<Point, char>, graph2 : &mut HashMap<Point, char>) {
    for (point, &c) in graph1.iter() {
        let active = point
            .neighbors()
            .iter()
            .map(|x| graph1.get(&x).unwrap_or(&'.'))
            .filter(|&&x| x == '#')
            .count();

        if c == '#' {
            if !(active == 2 || active == 3) {
                graph2.insert(*point, '.');
            } else {
                graph2.insert(*point, '#');
            }

        }
        else if c == '.' {
            if active == 3 {
                graph2.insert(*point, '#');
            }
            else {
                graph2.insert(*point, '.');
            }
        }
    }
}

fn next_state2(graph1 : &HashMap<Point4D, char>, graph2 : &mut HashMap<Point4D, char>) {
    for (point, &c) in graph1.iter() {
        let active = point
            .neighbors()
            .iter()
            .map(|x| graph1.get(&x).unwrap_or(&'.'))
            .filter(|&&x| x == '#')
            .count();

        if c == '#' {
            if !(active == 2 || active == 3) {
                graph2.insert(*point, '.');
            } else {
                graph2.insert(*point, '#');
            }

        }
        else if c == '.' {
            if active == 3 {
                graph2.insert(*point, '#');
            }
            else {
                graph2.insert(*point, '.');
            }
        }
    }
}

fn read_input1(graph1 : &mut HashMap<Point, char>, graph2 : &mut HashMap<Point, char>) -> Result<(), io::Error> {
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt")?;
    for z in MIN_SIZE..=MAX_SIZE {
        for x in MIN_SIZE..=MAX_SIZE {
            for y in MIN_SIZE..=MAX_SIZE {
                graph1.insert(Point{x, y, z}, '.');
            }
        }
    }

    data
        .lines()
        .enumerate()
        .for_each(|(y, line)|
            line.chars().enumerate().for_each(|(x, c)|{
                graph1.insert(Point{x: x as i32, y: y as i32, z:0}, c);
            }));

    graph1.iter().for_each(|(&k, &v)| { graph2.insert(k, v); });
    println!("Read input took {:?}", now.elapsed());
    Ok(())
}

fn read_input2(graph1 : &mut HashMap<Point4D, char>, graph2 : &mut HashMap<Point4D, char>) -> Result<(), io::Error> {
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt")?;
    for w in MIN_SIZE..=MAX_SIZE {
        for z in MIN_SIZE..=MAX_SIZE {
            for x in MIN_SIZE..=MAX_SIZE {
                for y in MIN_SIZE..=MAX_SIZE {
                    graph1.insert(Point4D { x, y, z, w }, '.');
                }
            }
        }
    }

    data
        .lines()
        .enumerate()
        .for_each(|(y, line)|
            line.chars().enumerate().for_each(|(x, c)|{
                graph1.insert(Point4D{x: x as i32, y: y as i32, z:0, w:0}, c);
            }));

    graph1.iter().for_each(|(&k, &v)| { graph2.insert(k, v); });
    println!("Read input took {:?}", now.elapsed());
    Ok(())
}

fn part1(graph1 : &mut HashMap<Point, char>, graph2: &mut HashMap<Point, char>) {
    let now = Instant::now();
    next_state1(&graph1, graph2);
    next_state1(&graph2, graph1);
    next_state1(&graph1, graph2);
    next_state1(&graph2, graph1);
    next_state1(&graph1, graph2);
    next_state1(&graph2, graph1);
    let ans = graph1.values().filter(|&&x| x == '#').count();
    println!("{}", ans);
    println!("Part 1     took {:?}", now.elapsed());
}

fn part2(graph1 : &mut HashMap<Point4D, char>, graph2: &mut HashMap<Point4D, char>) {
    let now = Instant::now();
    next_state2(&graph1, graph2);
    next_state2(&graph2, graph1);
    next_state2(&graph1, graph2);
    next_state2(&graph2, graph1);
    next_state2(&graph1, graph2);
    next_state2(&graph2, graph1);
    let ans = graph1.values().filter(|&&x| x == '#').count();
    println!("{}", ans);
    println!("Part 1     took {:?}", now.elapsed());
}

fn main() -> Result<(), io::Error> {
    let mut graph1 : HashMap<Point, char> = HashMap::new();
    let mut graph2 : HashMap<Point, char> = HashMap::new();
    read_input1(&mut graph1, &mut graph2)?;
    part1(&mut graph1, &mut graph2);

    let mut graph1 : HashMap<Point4D, char> = HashMap::new();
    let mut graph2 : HashMap<Point4D, char> = HashMap::new();
    read_input2(&mut graph1, &mut graph2)?;
    part2(&mut graph1, &mut graph2);
    Ok(())
}
