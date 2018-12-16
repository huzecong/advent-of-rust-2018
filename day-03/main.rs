use std::fs;

struct Rectange {
    id: i32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn parse_numeric<'a, T, I>(iter: I) -> Vec<T> where
        T: std::str::FromStr,
        I: Iterator<Item=&'a str> {
    iter.map(|x| x.parse::<T>().ok().unwrap()).collect()
}

fn parse(s: &str) -> Rectange {
    let parts = s.split_whitespace().collect::<Vec<&str>>();
    let id = parts[0][1..].parse::<i32>().unwrap();
    let x_y = parse_numeric::<usize, _>(parts[2][..(parts[2].len() - 1)].split(","));
    let w_h = parse_numeric::<usize, _>(parts[3].split("x"));
    Rectange { id: id, x: x_y[0], y: x_y[1], w: w_h[0], h: w_h[1] }
}

fn cumsum(arr: &mut Vec<Vec<i32>>, width: usize, height: usize) {
    for i in 1..width {
        arr[i][0] += arr[i - 1][0];
    }
    for j in 1..height {
        arr[0][j] += arr[0][j - 1];
    }
    for i in 1..width {
        for j in 1..height {
            arr[i][j] += arr[i - 1][j] + arr[i][j - 1] - arr[i - 1][j - 1];
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").ok().unwrap();  // temporary bound to the scope
    let rects: Vec<Rectange> = input.split_terminator("\n").map(parse).collect();

    // Part 1
    let width = rects.iter().map(|rect| rect.x + rect.w).max().unwrap() as usize;
    let height = rects.iter().map(|rect| rect.y + rect.h).max().unwrap() as usize;
    let mut counts = vec![vec![0; width + 1]; height + 1];  // prefix array
    for rect in rects.iter() {
        let right = rect.x + rect.w;
        let down = rect.y + rect.h;
        counts[rect.x][rect.y] += 1;
        counts[right][rect.y] -= 1;
        counts[rect.x][down] -= 1;
        counts[right][down] += 1;
    }
    cumsum(&mut counts, width, height);
    let mut overlap = 0;
    for i in 0..width {
        for j in 1..height {
            if counts[i][j] > 1 {
                overlap += 1;
            }
        }
    }
    println!("{}", overlap);

    // Part 2
    cumsum(&mut counts, width, height);
    let get_counts = |x: usize, y: usize| -> i32 {
        if x > 0 && y > 0 {
            counts[x - 1][y - 1]
        } else { 0 }
    };
    for rect in rects.iter() {
        let right = rect.x + rect.w;
        let down = rect.y + rect.h;
        let sum = get_counts(right, down) - get_counts(right, rect.y) - get_counts(rect.x, down) + get_counts(rect.x, rect.y);
        let area = (rect.w * rect.h) as i32;
        if sum == area {
            println!("{}", rect.id);
            break;
        }
    }
}
