use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
enum Last {
    Once(u32),
    Twice(u32, u32)
}

fn update_last_spoken(last_spoken : &mut HashMap<u32, Last>, spoken : u32, turn : u32) {
    match last_spoken.get(&spoken) {
        Some(Last::Once(last)) => {
            last_spoken.insert(spoken, Last::Twice(*last, turn as u32));
        }
        Some(Last::Twice(_l1, l2)) => {
            last_spoken.insert(spoken, Last::Twice(*l2, turn as u32));
        }
        None => {
            last_spoken.insert(spoken, Last::Once(turn as u32));
        }
    }
}

fn part1(input : &Vec<u32>, target : usize) {
    let mut last_spoken : HashMap<u32, Last> = HashMap::new();
    let mut last = *input.last().unwrap();
    for turn in input.len()..=target {
        let turn = (turn + 1) as u32;
        match last_spoken.get(&last) {
            None => {
                update_last_spoken(&mut last_spoken, 0, turn);
                last = 0;
            }
            Some(Last::Once(l)) => {
                match input.iter().position(|x| *x == last) {
                    Some(p) => {
                        let v = *l - (p + 1) as u32;
                        update_last_spoken(&mut last_spoken, v, turn);
                        last = v;
                    }
                    None => {
                        update_last_spoken(&mut last_spoken, 0, turn);
                        last = 0;
                    }
                }
            }
            Some(Last::Twice(l1, l2)) => {
                let v = l2 - l1;
                update_last_spoken(&mut last_spoken, v, turn);
                last = v;
            }
        }
        if turn == (target as u32) {
            println!("{}", last);
        }

    }
}

fn main() {
    let now = Instant::now();
    let data = "2,0,1,7,4,14,18";
    let input : Vec<u32> = data.split(",").map(|x| x.parse().unwrap()).collect();
    println!("Read input took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&input, 2020);
    println!("Part 1     took {:?}", now.elapsed());

    let now = Instant::now();
    part1(&input, 30000000); // This is actually part 2; Just too lazy to change the name
    println!("Part 2     took {:?}", now.elapsed());
}
