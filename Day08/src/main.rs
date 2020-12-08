use std::fs;
use std::time::Instant;

fn parse_input(data : &String) -> Vec<(&str, i32)>{
    let lines : Vec<&str> = data.split('\n').collect();
    lines.iter().map(|&x| {
        let parts : Vec<&str>= x.split(" ").collect();
        let code = parts[0];
        let offset : i32 = parts[1].parse().unwrap();
        (code, offset)
    }).collect()
}

fn finishes(instructions : &Vec<(&str, i32)>) -> (bool, i32) {
    let mut acc : i32 = 0;
    let n = instructions.len();
    let mut count = vec![0; n];
    let mut pc : i32 = 0;
    loop {
        if (pc as usize) >= n {
            return (true, acc);
        }
        if count[pc as usize] >= 1 {
            return (false, acc);
        }

        count[pc as usize] += 1;
        match instructions[pc as usize] {
            ("nop", _offset) => {},
            ("acc", offset) => acc += offset,
            ("jmp", offset) => { pc += offset; continue; }
            _ => {}
        }
        pc += 1;
    }
}

fn part1(instructions : &Vec<(&str, i32)>) {
    let (_, acc) = finishes(instructions);
    println!("{}", acc);
}

fn part2(mut instructions : Vec<(&str, i32)>) {
    for i in 1..instructions.len() {
        let (instruction, offset) = instructions[i];
        if instruction == "jmp" {
            instructions[i] = ("nop", offset);
        } else if instruction == "nop" {
            instructions[i] = ("jmp", offset);
        }
        let (finish, acc) = finishes(&instructions);
        if finish {
            println!("{}", acc);
            return;
        }
        instructions[i] = (instruction, offset);
    }
}

fn main() {
    let now = Instant::now();
    let data  = fs::read_to_string("assets/input.txt").unwrap();
    let instructions : Vec<(&str, i32)> = parse_input(&data);
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&instructions);
    println!("Part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    part2(instructions);
    println!("Part 2     took {:?}", now.elapsed());
}
