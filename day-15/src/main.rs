#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

use std::collections::VecDeque;
use std::fs;

custom_derive! {
#[derive(Copy, Clone, IterVariants(DirectionVariants))]
enum Direction { LEFT, UP, RIGHT, DOWN }
}

type Point = (usize, usize);

trait MinPointBy {
    fn min_point_by<T, F1>(self, f: F1) -> Option<Point> where
            T: Ord,
            F1: Fn(Point) -> Option<T>;
}

impl<I> MinPointBy for I where
        I: Iterator<Item=Point> {
    fn min_point_by<T, F>(self, f: F) -> Option<Point> where
            T: Ord,
            F: Fn(Point) -> Option<T> {
        self.filter_map(|p| f(p).map(|v| (v, p)))
                .min().map(|(_, p)| p)
    }
}

struct State {
    r: usize,
    c: usize,
    map: Vec<Vec<char>>,
    moved: Vec<Vec<bool>>,
    hp: Vec<Vec<i32>>,
    elf_died: bool,
}

impl State {
    fn new(map: &Vec<Vec<char>>) -> State {
        let map = map.clone();
        let r = map.len();
        let c = map.first().unwrap().len();
        let moved = vec![vec![false; c]; r];
        let mut hp = vec![vec![0; c]; r];
        for i in 0..r {
            for j in 0..c {
                if map[i][j] == 'G' || map[i][j] == 'E' { hp[i][j] = 200; }
            }
        }
        State { r, c, map, moved, hp, elf_died: false }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.r {
            println!("{}", self.map[i].iter().collect::<String>());
        }
        println!();
    }

    fn move_unit(&self, (x, y): Point, d: Direction) -> Option<Point> {
        match d {
            Direction::LEFT => if y == 0 { None } else { Some((x, y - 1)) },
            Direction::UP => if x == 0 { None } else { Some((x - 1, y)) },
            Direction::RIGHT => if y == self.c - 1 { None } else { Some((x, y + 1)) },
            Direction::DOWN => if x == self.r - 1 { None } else { Some((x + 1, y)) },
        }
    }

    fn new_round(&mut self) {
        self.moved = vec![vec![false; self.c]; self.r];
    }

    fn get_target_type(ch: char) -> Option<char> {
        match ch {
            'G' => Some('E'),
            'E' => Some('G'),
            _ => None,
        }
    }

    fn adjacent(&self, pos: Point) -> impl Iterator<Item=Point> + '_ {
        Direction::iter_variants().filter_map(move |d| self.move_unit(pos, d))
    }

    fn is_adjacent_to(&self, pos: Point, target_type: char) -> bool {
        self.adjacent(pos).any(|(x, y)| self.map[x][y] == target_type)
    }

    fn floodfill(&self, (x, y): Point) -> Vec<Vec<u32>> {
        let mut dist = vec![vec![std::u32::MAX; self.c]; self.r];
        let mut queue = VecDeque::<Point>::new();
        dist[x][y] = 0;
        queue.push_back((x, y));
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            let d = dist[x][y];
            for (x, y) in self.adjacent((x, y)) {
                if dist[x][y] == std::u32::MAX && self.map[x][y] != '#' {
                    dist[x][y] = d + 1;
                    // only expand for spaces
                    if self.map[x][y] == '.' { queue.push_back((x, y)); }
                }
            }
        }
        return dist;
    }

    fn exec_move(&mut self, from: Point, to: Point) {
        let (fx, fy) = from;
        let (tx, ty) = to;
        debug_assert!(self.map[tx][ty] == '.');
        debug_assert!(self.hp[tx][ty] == 0);
        self.map[tx][ty] = self.map[fx][fy];
        self.map[fx][fy] = '.';
        self.hp[tx][ty] = self.hp[fx][fy];
        self.hp[fx][fy] = 0;
    }

    fn exec_die(&mut self, (x, y): Point) {
        debug_assert!(self.map[x][y] == 'G' || self.map[x][y] == 'E');
        debug_assert!(self.hp[x][y] <= 0);
        if self.map[x][y] == 'E' { self.elf_died = true; }
        self.map[x][y] = '.';
        self.hp[x][y] = 0;
    }

    fn exec_turn(&mut self, (x, y): Point, atk: i32) -> bool {
        let target_type = State::get_target_type(self.map[x][y]).unwrap();

        let mut pos = (x, y);
        if !self.is_adjacent_to(pos, target_type) {
            let dist = self.floodfill(pos);
            let mut best_dist = std::u32::MAX;
            let mut t_pos: Option<Point> = None;
            let mut found = false;
            for i in 0..self.r {
                for j in 0..self.c {
                    if self.map[i][j] == target_type {
                        found = true;
                    } else if self.map[i][j] == '.' && self.is_adjacent_to((i, j), target_type) {
                        if best_dist > dist[i][j] {
                            best_dist = dist[i][j];
                            t_pos = Some((i, j));
                        }
                    }
                }
            }
            if !found { return false; }
            if best_dist == std::u32::MAX { return true; } // no reachable targets, unit does nothing

            // need to move
            let dist = self.floodfill(t_pos.unwrap());
            let m_pos = self.adjacent(pos)
                    .min_point_by(|(x, y)| if self.map[x][y] == '.' { Some(dist[x][y]) } else { None })
                    .unwrap();
            self.exec_move(pos, m_pos);
            pos = m_pos;
        }
        match pos { (x, y) => self.moved[x][y] = true };

        // find target to attack
        let target = self.adjacent(pos)
                .min_point_by(|(x, y)| if self.map[x][y] == target_type { Some(self.hp[x][y]) } else { None });
        match target {
            Some((x, y)) => {
                self.hp[x][y] -= atk;
                if self.hp[x][y] <= 0 { self.exec_die((x, y)); }
            }
            _ => (),
        }

        return true;
    }
}

fn perform_check(map: &Vec<Vec<char>>, elf_atk: i32, ignore_death: bool) -> Option<usize> {
    let mut state = State::new(map);

    let mut rounds = 0;
    'outer: loop {
        let mut end = false;
        state.new_round();
        for i in 0..state.r {
            for j in 0..state.c {
                if !state.moved[i][j] && (state.map[i][j] == 'G' || state.map[i][j] == 'E') {
                    let atk = if state.map[i][j] == 'G' { 3 } else { elf_atk };
                    end |= !state.exec_turn((i, j), atk);
                    if !ignore_death && state.elf_died { return None; }
                    if end { break 'outer; }
                }
            }
        }
        rounds += 1;
    }

    let mut total_hp = 0;
    for i in 0..state.r {
        for j in 0..state.c {
            total_hp += state.hp[i][j];
        }
    }
    return Some(rounds * total_hp as usize);
}

fn main() {
    let input = fs::read_to_string("day-15/input.txt").ok().unwrap();
    let map: Vec<Vec<char>> = input.lines()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

    // Part 1
    let outcome = perform_check(&map, 3, true).unwrap();
    println!("{}", outcome);

    // Part 2
    let mut atk = 4;
    loop {
        let outcome = perform_check(&map, atk, false);
        if outcome.is_some() {
            println!("{}", outcome.unwrap());
            break;
        }
        atk += 1;
    }
}
