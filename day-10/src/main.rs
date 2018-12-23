use std::fs;
use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug)]
struct Vec2<T: Copy> {
    x: T,
    y: T,
}

impl<T> Add for Vec2<T> where
        T: Add<T, Output=T> + Copy {
    type Output = Self;

    fn add(self, rhs: Self) -> <Self as Add<Self>>::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> AddAssign for Vec2<T> where
        T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vec2<T> where
        T: Sub<T, Output=T> + Copy {
    type Output = Self;

    fn sub(self, rhs: Self) -> <Self as Sub<Self>>::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Debug)]
struct Star {
    pos: Vec2<i64>,
    vel: Vec2<i64>,
}

fn parse(s: &str) -> Star {
    let mut r = 0;
    let mut ps: Vec<Vec2<i64>> = vec![];
    for _ in 0..2 {
        let l = s[r..].find("<").unwrap() + r;
        let mid = s[r..].find(",").unwrap() + r;
        r = s[(r + 1)..].find(">").unwrap() + r + 1;
        let x = s[(l + 1)..mid].trim().parse::<i64>().ok().unwrap();
        let y = s[(mid + 2)..r].trim().parse::<i64>().ok().unwrap();
        ps.push(Vec2 { x, y });
    }
    let pos = ps.remove(0);
    let vel = ps.remove(0);
    Star { pos, vel }
}

fn compute_bounding_box(stars: &Vec<Star>) -> (Vec2<i64>, Vec2<i64>) {
    let mut upper_left = stars.first().unwrap().pos;
    let mut lower_right = stars.first().unwrap().pos;
    for star in stars.iter() {
        upper_left.x = upper_left.x.min(star.pos.x);
        upper_left.y = upper_left.y.min(star.pos.y);
        lower_right.x = lower_right.x.max(star.pos.x);
        lower_right.y = lower_right.y.max(star.pos.y);
    }
    return (upper_left, lower_right);
}

fn box_area(bounds: (Vec2<i64>, Vec2<i64>)) -> i64 {
    let (upper_left, lower_right) = bounds;
    let Vec2 { x: ulx, y: uly } = upper_left;
    let Vec2 { x: lrx, y: lry } = lower_right;
    return (lrx - ulx + 1) * (lry - uly + 1);
}

fn print(stars: &Vec<Star>) {
    let (ul, lr) = compute_bounding_box(stars);
    let mut board = vec![vec![' '; (lr.x - ul.x + 1) as usize]; (lr.y - ul.y + 1) as usize];
    for star in stars.iter() {
        board[(star.pos.y - ul.y) as usize][(star.pos.x - ul.x) as usize] = '█';
    }
    for line in board.iter() {
        println!("{}", line.iter().collect::<String>());
    }
    println!();
}

fn main() {
    let input = fs::read_to_string("day-10/input.txt").ok().unwrap();
    let mut stars = input.lines().map(parse).collect::<Vec<_>>();

    // Parts 1 & 2
    let mut min_bounding_area = box_area(compute_bounding_box(&stars));
    let mut iters = 0;
    loop {
        let new_stars = stars.iter()
                .map(|s| Star { pos: s.pos + s.vel, vel: s.vel })
                .collect::<Vec<_>>();
        let cur_area = box_area(compute_bounding_box(&new_stars));
        if cur_area > min_bounding_area { break; }
        min_bounding_area = cur_area;
        stars = new_stars;
        iters += 1;
    }

    print(&stars);
    println!("{}", iters);
}
