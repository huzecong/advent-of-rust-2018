use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Rule {
    pattern: String,
    outcome: char,
}

fn parse(s: &str) -> Rule {
    let parts = s.split_whitespace().collect::<Vec<_>>();
    let pattern = parts[0].chars().collect::<String>();
    let outcome = parts[2].chars().next().unwrap();
    Rule { pattern, outcome }
}

fn main() {
    const M: usize = 120;
    let rounds1 = 20;

    let input = fs::read_to_string("day-12/input.txt").ok().unwrap();
    let state = input.lines().next().unwrap()
            .split_whitespace().skip(2).next().unwrap();
    let rules = input.lines().skip(2).map(parse).collect::<Vec<_>>();
    let mut h = HashMap::<String, char>::new();
    for rule in rules {
        h.insert(rule.pattern, rule.outcome);
    }

    // Part 1
    let mut state = [".".repeat(M + 2), state.to_string(), ".".repeat(M + 2)]
            .join("")
            .chars().collect::<Vec<_>>();
    for _round in 1..=M {
        let mut new_state = vec!['.'; state.len()];
        for i in 2..(state.len() - 2) {
            let key = state[(i - 2)..(i + 3)].iter().collect::<String>();
            new_state[i] = *h.get(&key).unwrap_or(&'.');
        }
        state = new_state;
//        println!("{} {}", _round + 1, state.iter().collect::<String>());
        if _round == rounds1 {
            let sum: i32 = state.iter()
                    .enumerate()
                    .filter_map(|(i, &ch)| if ch == '#' { Some(i as i32 - M as i32 - 2) } else { None })
                    .sum();
            println!("{}", sum);
        }
    }

    // Part 2
    let rounds2: i64 = 50000000000;
    let sum: i64 = state.iter()
            .enumerate()
            .filter_map(|(i, &ch)| if ch == '#' {
                Some(i as i64 - M as i64 - 2 + (rounds2 - M as i64))
            } else { None })
            .sum();
    println!("{}", sum);
}
