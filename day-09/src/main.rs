extern crate utils;

use utils::LinkedList;

use std::fs;

fn parse(s: &str) -> (usize, usize) {
    let parts = s.trim().split_whitespace().collect::<Vec<_>>();
    let f = |idx: usize| parts[idx].parse::<usize>().ok().unwrap();
    return (f(0), f(6));
}

fn simulate(n_players:usize, n_rounds:usize) -> Vec<usize> {
    let mut scores = vec![0; n_players];
    let mut list = LinkedList::new(0);
    let mut cur = list.head_mut();
    for round in 1..=n_rounds {
        if round % 23 == 0 {
            let player = (round - 1) % n_players;
            scores[player] += round;
            for _ in 0..7 { cur = cur.prev_mut() }
            scores[player] += cur.val;
            cur = cur.remove();
        } else {
            cur = cur.next_mut().insert_after(round);
        }
//        println!("{:?}", list.iter().collect::<Vec<_>>());
    }
    return scores;
}

fn main() {
    let input = fs::read_to_string("day-09/input.txt").ok().unwrap();
    let (n_players, n_rounds) = parse(&input);

    // Part 1
    let scores = simulate(n_players, n_rounds);
    println!("{}", scores.iter().max().unwrap());

    // Part 2
    let scores = simulate(n_players, n_rounds * 100);
    println!("{}", scores.iter().max().unwrap());
}
