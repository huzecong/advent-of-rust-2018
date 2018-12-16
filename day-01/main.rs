use std::collections::HashSet;
use std::fs;
use std::io;

fn read_sequence<T: std::str::FromStr>(path: &str) -> Result<Vec<T>, io::Error> {
    let xs = fs::read_to_string(path)?
        .lines()
        .map(|s| s.trim().parse::<T>().ok().unwrap())
        .collect::<Vec<T>>();
    return Ok(xs);
}

fn main() {
    let xs = read_sequence::<i32>("input.txt").ok().unwrap();

    // Part 1
    let sum: i32 = xs.iter().sum();
    println!("{}", sum);

    // Part 2
    let mut h = HashSet::new();
    h.insert(0);
    let mut sum = 0;
    for x in xs.iter().cycle() {
        sum += x;
        if h.contains(&sum) {
            println!("{}", sum);
            break;
        }
        h.insert(sum);
    }
}
