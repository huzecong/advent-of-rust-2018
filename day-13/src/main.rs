#[macro_use]
extern crate utils;

use std::fs;

use utils::PrimitiveEnum;

custom_derive! {
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, PrimitiveEnum)]
enum Direction { LEFT, UP, RIGHT, DOWN }
}

custom_derive! {
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, PrimitiveEnum)]
enum Turn { LEFT, STRAIGHT, RIGHT }
}


#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Cart {
    p: Point,
    dir: Direction,
    turn: Turn,
}

fn turn(d: Direction, t: Turn) -> Direction {
    match t {
        Turn::STRAIGHT => d,
        Turn::LEFT => utils::prev_enum(&d),
        Turn::RIGHT => utils::next_enum(&d),
    }
}

fn main() {
    let input = fs::read_to_string("day-13/input.txt").ok().unwrap();
    let mut map = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let (r, c) = (map.len(), map.first().unwrap().len());

    let mut carts: Vec<Cart> = vec![];
    for x in 0..r {
        for y in 0..c {
            let d = match map[x][y] {
                '>' => Some(Direction::RIGHT),
                '<' => Some(Direction::LEFT),
                '^' => Some(Direction::UP),
                'v' => Some(Direction::DOWN),
                _ => None,
            };
            d.map(|d| carts.push(Cart { p: Point { x, y }, dir: d, turn: Turn::LEFT }));
            match map[x][y] {
                '>' | '<' => map[x][y] = '-',
                '^' | 'v' => map[x][y] = '|',
                _ => (),
            };
        }
    }

    // Parts 1 & 2
    let mut first_crash: Option<Point> = None;
    let final_pos: Option<Point>;
    loop {
        let mut remove = vec![false; carts.len()];
        for i in 0..carts.len() {
            if remove[i] { continue; }
            let Cart { p: Point { x, y }, dir: d, turn: t } = carts[i];
            let new_point = match d {
                Direction::LEFT => Point { x, y: y - 1 },
                Direction::RIGHT => Point { x, y: y + 1 },
                Direction::UP => Point { x: x - 1, y },
                Direction::DOWN => Point { x: x + 1, y },
            };
            let new_dir = match map[new_point.x][new_point.y] {
                '/' => match d {
                    Direction::LEFT | Direction::RIGHT => turn(d, Turn::LEFT),
                    Direction::UP | Direction::DOWN => turn(d, Turn::RIGHT),
                },
                '\\' => match d {
                    Direction::LEFT | Direction::RIGHT => turn(d, Turn::RIGHT),
                    Direction::UP | Direction::DOWN => turn(d, Turn::LEFT),
                },
                '+' => turn(d, t),
                _ => d,
            };
            let new_turn = match map[new_point.x][new_point.y] {
                '+' => utils::next_enum(&t),
                _ => t,
            };
            for cart in &carts[(i + 1)..] {
                if cart.p == new_point {}
            }
            let crash_cart = carts.iter().enumerate()
                    .zip(remove.iter())
                    .filter_map(|((i, c), &r)| if !r && c.p == new_point { Some(i) } else { None })
                    .next();
            if crash_cart.is_some() {
                if first_crash.is_none() { first_crash = Some(new_point); }
                remove[i] = true;
                remove[crash_cart.unwrap()] = true;
            } else {
                carts[i] = Cart { p: new_point, dir: new_dir, turn: new_turn };
            }
        }
        carts = carts.into_iter()
                .zip(remove.iter())
                .filter_map(|(c, &r)| if r { None } else { Some(c) })
                .collect::<Vec<_>>();
        if carts.len() == 1 {
            final_pos = Some(carts[0].p);
            break;
        }
        carts.sort();
    }

    first_crash.map(|p| println!("{},{}", p.y, p.x));
    final_pos.map(|p| println!("{},{}", p.y, p.x));
}
