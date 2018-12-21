#[macro_use]
extern crate custom_derive;

use std::fs;

trait PrimitiveEnum: Sized {
    fn next_enum(&self) -> Option<Self>;
    fn prev_enum(&self) -> Option<Self>;
    fn first_enum() -> Self;
    fn last_enum() -> Self;
}

// Implementation reference: crate `enum_derive`
macro_rules! first_enum {
    (
        ($name:ident) ($a:ident $($rest:ident)*)
    ) => {
        $name::$a
    };
}

macro_rules! last_enum {
    (
        ($name:ident) ($a:ident $($rest:ident)+)
    ) => {
        last_enum! { ($name) ($($rest)*) }
    };

    (
        ($name:ident) ($a:ident)
    ) => {
        $name::$a
    };
}

macro_rules! next_enum {
    (
        ($name:ident, $self_:ident) ($($val:ident)*)
    ) => {
        next_enum! { @arms ($name, $self_) ($($val)*) -> () }
    };

    (
        @arms ($name:ident, $self_:ident) ($a:ident) -> ($($body:tt)*)
    ) => {
        match *$self_ {
            $($body)*
            $name::$a => ::std::option::Option::None,
        }
    };

    (
        @arms ($name:ident, $self_:ident) ($a:ident $b:ident $($rest:ident)*) -> ($($body:tt)*)
    ) => {
        next_enum! {
            @arms ($name, $self_) ($b $($rest)*) -> (
                $($body)*
                $name::$a => ::std::option::Option::Some($name::$b),
            )
        }
    };
}


macro_rules! prev_enum {
    (
        ($name:ident, $self_:ident) ($($val:ident)*)
    ) => {
        prev_enum! { @arms ($name, $self_) (::std::option::Option::None) ($($val)*) -> () }
    };

    (
        @arms ($name:ident, $self_:ident) ($prev:expr) ($a:ident) -> ($($body:tt)*)
    ) => {
        match *$self_ {
            $($body)*
            $name::$a => $prev,
        }
    };

    (
        @arms ($name:ident, $self_:ident) ($prev:expr) ($a:ident $($rest:ident)+) -> ($($body:tt)*)
    ) => {
        prev_enum! {
            @arms ($name, $self_) (::std::option::Option::Some($name::$a)) ($($rest)*) -> (
                $($body)*
                $name::$a => $prev,
            )
        }
    };
}

macro_rules! PrimitiveEnum {
    (
        () pub enum $name:ident { $($val:ident),* }
    ) => {
        PrimitiveEnum! { [pub] ($name) ($($val)*) }
    };

    (
        () enum $name:ident { $($val:ident),* }
    ) => {
        PrimitiveEnum! { [] ($name) ($($val)*) }
    };

    (
        [$($pub_:tt)?] ($name:ident) ($($val:ident)*)
    ) => {
        $($pub_)? impl PrimitiveEnum for $name {
            fn next_enum(&self) -> Option<Self> {
                next_enum! { ($name, self) ($($val)*) }
            }

            fn prev_enum(&self) -> Option<Self> {
                prev_enum! { ($name, self) ($($val)*) }
            }

            fn first_enum() -> Self {
                first_enum! { ($name) ($($val)*) }
            }

            fn last_enum() -> Self {
                last_enum! { ($name) ($($val)*) }
            }
        }
    };
}

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

fn next_enum<E: PrimitiveEnum>(e: &E) -> E {
    match e.next_enum() {
        Some(e) => e,
        None => E::first_enum(),
    }
}

fn prev_enum<E: PrimitiveEnum>(e: &E) -> E {
    match e.prev_enum() {
        Some(e) => e,
        None => E::last_enum(),
    }
}

fn turn(d: Direction, t: Turn) -> Direction {
    match t {
        Turn::STRAIGHT => d,
        Turn::LEFT => prev_enum(&d),
        Turn::RIGHT => next_enum(&d),
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").ok().unwrap();
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
                '+' => next_enum(&t),
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
