use utils::DisjointSet;
use std::fs;

struct Point(i32, i32, i32, i32);

fn parse(s: &str) -> Point {
    let xs = s.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    Point(xs[0], xs[1], xs[2], xs[3])
}

fn distance(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn main() {
    let input = fs::read_to_string("day-25/input.txt").unwrap();
    let points = input.lines().map(parse).collect::<Vec<_>>();

    let mut union = DisjointSet::new(points.len());
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            if distance(&points[i], &points[j]) <= 3 {
                union.merge(i, j);
            }
        }
    }

    let n_components = (0..points.len()).filter(|&i| union.find(i) == i).count();
    println!("{}", n_components);
}
