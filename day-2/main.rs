use std::collections::HashMap;
use std::fs;

fn counter<I, T>(iter: I) -> HashMap<T, i32> where
        I: Iterator<Item=T>, T: std::cmp::Eq, T: std::hash::Hash {
    let mut h = HashMap::new();
    for x in iter {
        *h.entry(x).or_default() += 1;
    }
    return h;
}

fn main() {
    let input = fs::read_to_string("input.txt").ok().unwrap();  // temporary bound to the scope
    let strs: Vec<&str> = input.split_terminator("\n").collect();

    // Part 1
    let counts = strs.iter().map(|s| counter(s.chars())).collect::<Vec<_>>();
    let twos = counts.iter().filter(|h| h.iter().any(|(_, &v)| v == 2)).count();
    let threes = counts.iter().filter(|h| h.iter().any(|(_, &v)| v == 3)).count();
    println!("{}", twos * threes);

    // Part 2
    let mut found = false;
    for (i, sa) in strs.iter().enumerate() {
        for sb in strs[i..].iter() {
            let common = sa.chars().zip(sb.chars()).filter(|(a, b)| a == b);
            if common.count() == sa.len() - 1 {
                let common_str = sa.chars().zip(sb.chars()).filter(|(a, b)| a == b)  // had to do it again because of ownership transfer
                        .map(|(a, _)| a.to_string()).collect::<Vec<String>>().concat();
                println!("{}", common_str);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}
