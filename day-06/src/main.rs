use std::fs;

struct Point(i32, i32);

fn parse(s: &str) -> Point {
    let parts = s.trim().split_whitespace().collect::<Vec<_>>();
    let x = parts[0][..parts[0].len() - 1].parse::<i32>().ok().unwrap();
    let y = parts[1].parse::<i32>().ok().unwrap();
    Point(x, y)
}

fn min_max<'a, T, I>(mut iter: I) -> (T, T) where
        I: Iterator<Item=&'a T>,
        T: 'a + Ord + Copy {
    let init = *iter.next().unwrap();
    iter.fold((init, init), |(min, max), &x| (min.min(x), max.max(x)))
}

fn nearest_point(x: i32, y: i32, p: &Vec<Point>) -> i32 {
    let distance = p.iter()
            .map(|Point(px, py)| (x - *px).abs() + (y - *py).abs())
            .collect::<Vec<_>>();
    let nearest = *distance.iter().min().unwrap();
    let candidates = distance.iter()
            .enumerate()
            .filter_map(|(i, x)| if *x == nearest { Some(i) } else { None })
            .collect::<Vec<_>>();
    return if candidates.len() == 1 {
        *candidates.first().unwrap() as i32
    } else { -1 };
}

fn compute_ways(mut xs: Vec<i32>, max_dist: usize) -> Vec<u32> {
    let mut ways = vec![0; max_dist + 1];
    let n = xs.len();
    let sum_pos = xs.iter().sum::<i32>();
    xs.sort();

    let sum_dist_l = (sum_pos - xs[0] * n as i32) as usize;
    for x in ((sum_dist_l + n)..=max_dist).step_by(n) {
        ways[x] += 1;
    }

    let sum_dist_r = (xs.last().unwrap() * n as i32 - sum_pos) as usize;
    for x in (sum_dist_r..=max_dist).step_by(n) {
        ways[x] += 1;
    }

    let mut sum_dist = sum_dist_l;
    for i in 1..n {
        let (x_l, x_r) = (xs[i - 1], xs[i]);
        for _ in x_l..x_r {
            ways[sum_dist as usize] += 1;
            sum_dist += i;
            sum_dist -= n - i;
        }
    }
    debug_assert!(sum_dist == sum_dist_r);

    return ways;
}

fn main() {
    let input = fs::read_to_string("day-06/input.txt").ok().unwrap();
    let p: Vec<Point> = input.lines().map(parse).collect();
    let xs = p.iter().map(|Point(x, _)| *x).collect::<Vec<_>>();
    let ys = p.iter().map(|Point(_, y)| *y).collect::<Vec<_>>();

    // Part 1
    let (min_x, max_x) = min_max(xs.iter());
    let (min_y, max_y) = min_max(ys.iter());
    let mut infinite = vec![false; p.len()];
    let mut size = vec![0; p.len()];

    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            let nearest = nearest_point(x, y, &p);
            if nearest != -1 {
                if x < min_x || x > max_x || y < min_y || y > max_y {
                    infinite[nearest as usize] = true;
                } else {
                    size[nearest as usize] += 1;
                }
            }
        }
    }
    let largest_finite = size.iter().zip(infinite.iter())
            .filter_map(|(s, inf)| if *inf { None } else { Some(s) })
            .max().unwrap();
    println!("{}", largest_finite);

    // Part 2
    let max_distance = 10000 - 1;
    let ways_x = compute_ways(xs.clone(), max_distance);
    let mut ways_y = compute_ways(ys.clone(), max_distance);
    for i in 1..=max_distance {
        ways_y[i] += ways_y[i - 1];
    }
    let total_area: u32 = (0..=max_distance)
            .map(|x| ways_x[x] * ways_y[max_distance - x])
            .sum();
    println!("{}", total_area);
}
