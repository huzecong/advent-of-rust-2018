#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

use std::fs;

use utils::MinHeap;

const K: usize = 50;

type Point = (usize, usize);

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct State {
    p: Point,
    tool: usize,
}

custom_derive! {
#[derive(Copy, Clone, IterVariants(DirectionVariants))]
enum Direction { LEFT, UP, RIGHT, DOWN, STAY }
}

struct Board {
    r: usize,
    c: usize,
}

impl Board {
    fn move_unit(&self, (x, y): Point, d: Direction) -> Option<Point> {
        match d {
            Direction::LEFT => if y == 0 { None } else { Some((x, y - 1)) },
            Direction::UP => if x == 0 { None } else { Some((x - 1, y)) },
            Direction::RIGHT => if y == self.c - 1 { None } else { Some((x, y + 1)) },
            Direction::DOWN => if x == self.r - 1 { None } else { Some((x + 1, y)) },
            Direction::STAY => Some((x, y)),
        }
    }

    fn adjacent(&self, pos: Point) -> impl Iterator<Item=Point> + '_ {
        Direction::iter_variants().filter_map(move |d| self.move_unit(pos, d))
    }
}

fn main() {
    let input = fs::read_to_string("day-22/input.txt").unwrap();
    let mut lines = input.lines();
    let depth = lines.next().unwrap()[7..].parse::<usize>().unwrap();
    let target = lines.next().unwrap()[8..]
            .split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (t_r, t_c) = (target[0], target[1]);

    let rows = t_r + K;
    let cols = t_c + K;
    let mut erosion = vec![vec![0; cols]; rows];
    let mut map = erosion.clone();
    for i in 0..rows {
        for j in 0..cols {
            erosion[i][j] = (match (i, j) {
                (0, 0) => 0,
                (i, 0) => i * 16807,
                (0, j) => j * 48271,
                (i, j) if (i, j) == (t_r, t_c) => 0,
                (i, j) => erosion[i - 1][j] * erosion[i][j - 1],
            } + depth) % 20183;
            map[i][j] = erosion[i][j] % 3;
        }
    };

    // Part 1
    let mut sum = 0;
    for i in 0..=t_r { for j in 0..=t_c { sum += map[i][j]; } }
    println!("{}", sum);

    // Part 2
    let board = Board { r: rows, c: cols };
    let mut dist = vec![vec![vec![std::u32::MAX; 3]; cols]; rows];
    dist[0][0][1] = 0;
    let mut queue = MinHeap::<(u32, State)>::new();
    queue.push((0, State { p: (0, 0), tool: 1 }));

    while !queue.is_empty() {
        let (cur_dist, state) = queue.pop().unwrap();
        if cur_dist > dist[state.p.0][state.p.1][state.tool] { continue; }

        for (x, y) in board.adjacent(state.p) {
            for tool in 0..3 {
                if tool == map[x][y] || tool == map[state.p.0][state.p.1] { continue; }
                let new_dist = cur_dist
                        + if (x, y) == state.p { 0 } else { 1 }
                        + if tool == state.tool { 0 } else { 7 };
                if dist[x][y][tool] > new_dist {
                    dist[x][y][tool] = new_dist;
                    queue.push((new_dist, State { p: (x, y), tool }));
                }
            }
        }
    }

    let min_dist = dist[t_r][t_c][1];
    println!("{}", min_dist);
}
