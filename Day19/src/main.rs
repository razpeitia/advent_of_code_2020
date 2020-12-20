use std::fs;
use std::io;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
enum Term {
    Expr(u32),
    Chr(char),
}

fn matches(ast : &HashMap<u32, Vec<Vec<Term>>>, current : Term, line : &str, p : &mut usize, max : &mut usize) -> bool {
    match current {
        Term::Chr(c) => {
            if line.chars().nth(*p) == Some(c) {
                *p += 1;
                if *p > *max { *max = *p; }
                return true;
            }
            return false;
        }
        Term::Expr(c) => {
            let op = *p;
            ast
                .get(&c)
                .unwrap()
                .iter()
                .any(|rule| {
                    *p = op;
                    rule.iter().all(|&term| {
                        matches(ast, term, line, p, max)
                    })
                })
        }
    }
}

fn main() -> Result<(), io::Error> {
    let data = fs::read_to_string("assets/input.txt")?;
    let parts : Vec<&str> = data.split("\n\n").collect();

    let mut rules : HashMap<u32, Vec<Vec<Term>>> = HashMap::new();
    for line in parts[0].lines() {
        let p : Vec<&str> = line.split(": ").collect();
        let idx : u32 = p[0].parse().unwrap();
        let raw_rules : Vec<&str> = p[1].split(" | ").collect();
        let mut r : Vec<Vec<Term>> = Vec::new();
        for raw_rule in raw_rules {
            let parsed_rules : Vec<Term> = raw_rule.split_whitespace().map(|term| {
                if term.chars().all(char::is_numeric) {
                    Term::Expr(term.parse::<u32>().unwrap())
                } else {
                    Term::Chr(term.chars().nth(1).unwrap())
                }
            }).collect();
            r.push(parsed_rules);
        }
        rules.insert(idx, r);
    }

    let now = Instant::now();
    let ans = parts[1].lines().filter(|&line| {
        let mut max : usize = 0;
        let is_valid = matches(&rules, Term::Expr(0), line, &mut 0, &mut max);
        is_valid && max == line.len()
    }).count();
    println!("{}", ans);
    println!("Part 1     took {:?}", now.elapsed());

    rules.insert(8, vec![vec![Term::Expr(42)], vec![Term::Expr(42), Term::Expr(8)]]);
    rules.insert(11, vec![vec![Term::Expr(42), Term::Expr(31)], vec![Term::Expr(42), Term::Expr(11), Term::Expr(31)]]);
    let ans = parts[1].lines().filter(|&line| {
        let mut max : usize = 0;
        let is_valid = matches(&rules, Term::Expr(0), line, &mut 0, &mut max);
        is_valid
    }).count();
    println!("{}", ans);
    println!("Part 2     took {:?}", now.elapsed());
    Ok(())
}
