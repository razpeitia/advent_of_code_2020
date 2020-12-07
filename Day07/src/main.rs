use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, BTreeSet};
use std::time::Instant;

type Graph = HashMap<String, Vec<(u32, String)>>;

fn read_input() -> Graph {
    let file = File::open("assets/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rules : Graph = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts:Vec<&str> = line.split("bags contain").map(|x| x.trim()).collect();
        let key = String::from(parts[0]);
        if parts[1].trim() == "no other bags." {
            rules.insert(key, Vec::new());
        } else {
            let bags : Vec<(u32, String)> = parts[1].split(",").map(|x| {
                let p : Vec<&str> = x.trim().split(" ").map(|z| z.trim()).collect();
                let number : u32 = p[0].parse().unwrap();
                let color = String::from(format!("{} {}", p[1], p[2]));
                (number, color)
            }).collect();
            rules.insert(key, bags);
        }

    }
    rules
}

fn contains(start : &String, graph : &Graph) -> bool {
    let mut stack : Vec<&String> = Vec::new();
    let mut visited : BTreeSet<&String> = BTreeSet::new();
    stack.push(start);
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        if v == "shiny gold" {
            return true;
        }
        if !visited.contains(v) {
            visited.insert(v);
            for (_, adjacent) in graph.get(v).unwrap() {
                stack.push(adjacent);
            }
        }
    }
    false
}

fn part1(graph : &Graph) {
    let mut count : u32 = 0;
    for k in graph.keys() {
        if k == "shiny gold" {
            continue;
        }
        if contains(k, graph) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn count(node : &String, graph : &Graph) -> u32 {
    let children = graph.get(node).unwrap();
    let mut total : u32 = 0;
    for (w, _) in children {
        total += w;
    }

    for (w, c) in children {
        let ans = count(c, graph);
        total += w * ans;
    }
    total
}

fn part2(graph : &Graph) {
    let s = String::from("shiny gold");
    let ans = count(&s, graph);
    println!("{}", ans);
}

fn main() {
    let now = Instant::now();
    let graph : Graph = read_input();
    println!("read_input took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&graph);
    println!("part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    part2(&graph);
    println!("part 2     took {:?}", now.elapsed());
}
