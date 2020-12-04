use std::fs;
use std::time::Instant;

const REQUIRED_FIELDS: &[&str] = &["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

fn is_valid(record : &str) -> bool {
    for req_field in REQUIRED_FIELDS {
        if !record.contains(req_field) {
            return false;
        }
    }
    true
}

fn part1() {
    let a = fs::read_to_string("assets/input.txt").expect("Unable to open file");
    let records : Vec<&str> = a.split("\n\n").collect();
    let mut count = 0;
    for record in records {
        if is_valid(record) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn check_digits(value : &str, exact : u32) -> bool {
    if !(value.len() == exact as usize) {
        return false;
    }
    for c in value.chars() {
        if !c.is_ascii_digit() {
            return false
        }
    }
    true
}

fn check_range(val : u32, min : u32, max : u32) -> bool {
    val >= min && val <= max
}

fn check_color(value : &str) -> bool {
    if !(value.len() == 7 && value.starts_with("#")) {
        return false
    }
    let s = &value[1..];
    for c in s.chars() {
        if !c.is_ascii_hexdigit() {
            return false
        }
    }
    true
}

const EYE_COLOR: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn check_height(value : &str) -> bool {
    if value.ends_with("cm") {
        let value = value.strip_suffix("cm").unwrap();
        if !(check_digits(value, value.len() as u32) && check_range(value.parse::<u32>().unwrap(), 150, 193)) {
            return false;
        }
    } else if value.ends_with("in") {
        let value = value.strip_suffix("in").unwrap();
        if !(check_digits(value, value.len() as u32) && check_range(value.parse::<u32>().unwrap(), 59, 76)) {
            return false;
        }
    } else {
        return false;
    }
    true
}

fn is_valid2(passport : &str) -> bool {
    if !is_valid(passport) {
        return false;
    }
    /*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    */
    let records : Vec<&str> = passport.split_whitespace().collect();
    for record in records {
        let r : Vec<&str> = record.split(':').collect();
        let key = r[0];
        let value = r[1];
        match key {
            "byr" => {
                if !(check_digits(value, 4) && check_range(value.parse::<u32>().unwrap(), 1920, 2002)) {
                    return false;
                }
            }
            "iyr" => {
                if !(check_digits(value, 4) && check_range(value.parse::<u32>().unwrap(), 2010, 2020)) {
                    return false;
                }
            }
            "eyr" => {
                if !(check_digits(value, 4) && check_range(value.parse::<u32>().unwrap(), 2020, 2030)) {
                    return false;
                }
            }
            "hgt" => {
                if !check_height(value) { return false; }
            }
            "hcl" => {
                if !check_color(value) { return false; }
            }
            "ecl" => {
                if !EYE_COLOR.contains(&value) { return false; }
            }
            "pid" => {
                if !check_digits(value, 9) { return false; }
            }
            "cid" => {}
            _ => {
                println!("Unknow key {}", key);
                return false
            }
        }
    }
    true
}

fn part2() {
    let a = fs::read_to_string("assets/input.txt").expect("Unable to open file");
    let passports : Vec<&str> = a.split("\n\n").collect();
    let mut count = 0;
    for passport in passports {
        if is_valid2(passport) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn main() {
    let now = Instant::now();
    part1();
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    part2();
    println!("Part 2 took {:?}", now.elapsed());
}