use std::fs;
use std::io;
use std::collections::VecDeque;
use std::time::Instant;

fn evaluate(line : &str) -> u64 {
    let mut stack: VecDeque<&str> = VecDeque::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut tokens : Vec<&str> = Vec::new();

    let mut i = 0;
    let s: Vec<char> = line.chars().collect();
    while s.get(i).is_some() {
        while s.get(i).is_some() && s.get(i).unwrap().is_whitespace() { i += 1; }
        let oi = i;
        while s.get(i).is_some() && s.get(i).unwrap().is_ascii_digit() { i += 1; }
        if oi < s.len() && i > oi {
            tokens.push(&line[oi..i]);
        }

        match s.get(i) {
            Some('*') => { tokens.push("*"); i += 1; }
            Some('+') => { tokens.push("+"); i += 1; }
            Some(')') => { tokens.push(")"); i += 1; }
            Some('(') => { tokens.push("("); i += 1; }
            _ => {}
        }
    }

    for token in tokens {
        if token.parse::<u64>().is_ok() {
            queue.push_back(token);
        } else if token == "*" || token == "+" {
            while !stack.is_empty() {
                let &op = stack.back().unwrap();
                if op == "(" { break; }
                stack.pop_back().unwrap();
                queue.push_back(op);
            }
            stack.push_back(token);
        } else if token == "(" {
            stack.push_back("(");
        }
        else if token == ")" {
            while !stack.is_empty() {
                let op = stack.pop_back().unwrap();
                if op == "(" { break; }
                queue.push_back(op);
            }
        }
        else {
            panic!("Unknown token '{}'", token);
        }
    }
    while !stack.is_empty() {
        let &op = stack.back().unwrap();
        if op == "(" || op == ")" { unreachable!("Mismatched parenthesis"); }
        queue.push_back(stack.pop_back().unwrap())
    }

    let mut output : VecDeque<u64> = VecDeque::new();
    while !queue.is_empty() {
        let op = queue.pop_front().unwrap();
        match op {
            "*" => {
                let a = output.pop_back().unwrap();
                let b = output.pop_back().unwrap();
                output.push_back(a * b);
            }
            "+" => {
                let a = output.pop_back().unwrap();
                let b = output.pop_back().unwrap();
                output.push_back(a + b);
            }
            _ => { output.push_back(op.parse::<u64>().unwrap() ) }
        }
    }
    assert_eq!(1, output.len());
    output.pop_back().unwrap()
}

fn has_greater_presedence(a : &str, b : &str) -> bool {
    let pa = if a == "*" { 1 } else { 0 };
    let pb = if b == "*" { 1 } else { 0 };
    pa < pb
}

fn evaluate2(line : &str) -> u64 {
    let mut stack: VecDeque<&str> = VecDeque::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut tokens : Vec<&str> = Vec::new();

    let mut i = 0;
    let s: Vec<char> = line.chars().collect();
    while s.get(i).is_some() {
        while s.get(i).is_some() && s.get(i).unwrap().is_whitespace() { i += 1; }
        let oi = i;
        while s.get(i).is_some() && s.get(i).unwrap().is_ascii_digit() { i += 1; }
        if oi < s.len() && i > oi {
            tokens.push(&line[oi..i]);
        }

        match s.get(i) {
            Some('*') => { tokens.push("*"); i += 1; }
            Some('+') => { tokens.push("+"); i += 1; }
            Some(')') => { tokens.push(")"); i += 1; }
            Some('(') => { tokens.push("("); i += 1; }
            _ => {}
        }
    }

    for token in tokens {
        if token.parse::<u64>().is_ok() {
            queue.push_back(token);
        } else if token == "*" || token == "+" {
            while !stack.is_empty() {
                let &op = stack.back().unwrap();
                if op == "(" { break; }
                if !has_greater_presedence(op, token) { break; }
                stack.pop_back().unwrap();
                queue.push_back(op);
            }
            stack.push_back(token);
        } else if token == "(" {
            stack.push_back("(");
        }
        else if token == ")" {
            while !stack.is_empty() {
                let op = stack.pop_back().unwrap();
                if op == "(" { break; }
                queue.push_back(op);
            }
        }
        else {
            panic!("Unknown token '{}'", token);
        }
    }
    while !stack.is_empty() {
        let &op = stack.back().unwrap();
        if op == "(" || op == ")" { unreachable!("Mismatched parenthesis"); }
        queue.push_back(stack.pop_back().unwrap())
    }

    let mut output : VecDeque<u64> = VecDeque::new();
    while !queue.is_empty() {
        let op = queue.pop_front().unwrap();
        match op {
            "*" => {
                let a = output.pop_back().unwrap();
                let b = output.pop_back().unwrap();
                output.push_back(a * b);
            }
            "+" => {
                let a = output.pop_back().unwrap();
                let b = output.pop_back().unwrap();
                output.push_back(a + b);
            }
            _ => { output.push_back(op.parse::<u64>().unwrap() ) }
        }
    }
    assert_eq!(1, output.len());
    output.pop_back().unwrap()
}

fn main() -> Result<(), io::Error> {
    let now = Instant::now();
    let data = fs::read_to_string("assets/input.txt")?;
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    let ans = data.lines().map(|line| evaluate(line)).sum::<u64>();
    println!("{}", ans);
    println!("Part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    let ans = data.lines().map(|line| evaluate2(line)).sum::<u64>();
    println!("{}", ans);
    println!("Part 2     took {:?}", now.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::evaluate;
    use crate::evaluate2;

    #[test]
    fn part1_test() {
        assert_eq!(26, evaluate("2 * 3 + (4 * 5)"));
        assert_eq!(437, evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(13632, evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
        assert_eq!(12240, evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    }

    #[test]
    fn part2_test() {
        assert_eq!(51, evaluate2("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, evaluate2("2 * 3 + (4 * 5)"));
        assert_eq!(1445, evaluate2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(669060, evaluate2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(23340, evaluate2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}