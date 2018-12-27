use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
use std::fs;

type Point = (i32, i32);

struct Board {
    blocks: HashMap<Point, [bool; 4]>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    dist: HashMap<Point, usize>,
}

const DIRS: [char; 4] = ['N', 'W', 'S', 'E'];

fn move_delta(ch: char) -> Point {
    match ch {
        'N' => (-1, 0),
        'W' => (0, -1),
        'S' => (1, 0),
        'E' => (0, 1),
        _ => panic!(),
    }
}

fn dir_idx(ch: char) -> usize {
    match ch {
        'N' => 0,
        'W' => 1,
        'S' => 2,
        'E' => 3,
        _ => panic!(),
    }
}

impl Board {
    fn new(regex: &str) -> Board {
        let mut blocks = HashMap::<Point, [bool; 4]>::new();
        let mut stack = vec![];
        let mut total = HashSet::new();
        let mut current = HashSet::new();
        current.insert((0, 0));

        let mut update = |x, y, d| match blocks.get_mut(&(x, y)) {
            Some(valid) => valid[d] = true,
            None => {
                let mut valid = [false; 4];
                valid[d] = true;
                blocks.insert((x, y), valid);
            }
        };

        for ch in regex.chars() {
            match ch {
                '(' | '^' => {
                    stack.push((total, current.clone()));
                    total = HashSet::new();
                }
                ')' | '$' => {
                    let candidates = total.union(&current).cloned().collect();
                    let prev_state = stack.pop().unwrap();
                    total = prev_state.0;
                    current = prev_state.1;
                    current = current.union(&candidates).cloned().collect();
                }
                '|' => {
                    total = total.union(&current).cloned().collect();
                    current = stack.last().unwrap().1.clone();
                }
                'N' | 'S' | 'W' | 'E' => {
                    let (dx, dy) = move_delta(ch);
                    let d = dir_idx(ch);
                    current = current.iter().map(|(x, y)| {
                        let (nx, ny) = (x + dx, y + dy);
                        update(*x, *y, d);
                        update(nx, ny, d ^ 2);
                        (nx, ny)
                    }).collect();
                }
                _ => panic!(),
            }
        }
        let min_x = blocks.keys().map(|(x, _)| *x).min().unwrap();
        let max_x = blocks.keys().map(|(x, _)| *x).max().unwrap();
        let min_y = blocks.keys().map(|(_, y)| *y).min().unwrap();
        let max_y = blocks.keys().map(|(_, y)| *y).max().unwrap();

        let mut dist = HashMap::new();
        dist.insert((0, 0), 0);
        let mut queue = [(0, 0)].iter().cloned().collect::<VecDeque<_>>();
        while queue.len() > 0 {
            let (x, y) = queue.pop_front().unwrap();
            let cur_dist = dist[&(x, y)];
            for &d in DIRS.iter() {
                if !blocks[&(x, y)][dir_idx(d)] { continue; }
                let (dx, dy) = move_delta(d);
                let (nx, ny) = (x + dx, y + dy);
                if nx < min_x || nx > max_x || ny < min_y || ny > max_y
                        || dist.contains_key(&(nx, ny)) { continue; }
                dist.insert((nx, ny), cur_dist + 1);
                queue.push_back((nx, ny));
            }
        }

        Board { blocks, min_x, max_x, min_y, max_y, dist }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{}", "#".repeat(((self.max_y - self.min_y + 1) * 2 + 1) as usize));
        for x in self.min_x..=self.max_x {
            print!("#");
            for y in self.min_y..=self.max_y {
                print!("{}", if x == 0 && y == 0 { "X" } else { "." });
                if y < self.max_y {
                    print!("{}", if self.blocks[&(x, y)][dir_idx('S')] { '|' } else { '#' });
                }
            }
            println!("#");
            if x < self.max_x {
                print!("#");
                for y in self.min_y..=self.max_y {
                    print!("{}", if self.blocks[&(x, y)][dir_idx('E')] { '-' } else { '#' });
                    print!("#");
                }
                println!();
            }
        }
        println!("{}", "#".repeat(((self.max_y - self.min_y + 1) * 2 + 1) as usize));
    }
}

fn main() {
    let input = fs::read_to_string("day-20/input.txt").unwrap();
    let regex = input.trim();

    let board = Board::new(regex);
//    board.print();

    // Part 1
    let max_dist = board.dist.values().max().unwrap();
    println!("{}", max_dist);

    // Part 2
    let n_far_pos = board.dist.values().filter(|&&d| d >= 1000).count();
    println!("{}", n_far_pos);
}
