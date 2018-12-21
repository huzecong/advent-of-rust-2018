use std::fs;

fn compute_score(x: usize, y: usize, serial_id: i32) -> i32 {
    (((x + 10) * y + serial_id as usize) * (x + 10) / 100 % 10) as i32 - 5
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
    let input = fs::read_to_string("input.txt").ok().unwrap();
    let serial_id = input.trim().parse::<i32>().ok().unwrap();
    const SIZE: usize = 300;

    assert_eq!(compute_score(122, 79, 57), -5);
    assert_eq!(compute_score(217, 196, 39), 0);
    assert_eq!(compute_score(101, 153, 71), 4);

    // Part 1
    let mut scores = vec![vec![0i32; SIZE + 1]; SIZE + 1];
    for x in 1..=SIZE {
        for y in 1..=SIZE {
            scores[x][y] = compute_score(x, y, serial_id);
        }
    }
    cumsum(&mut scores, SIZE + 1, SIZE + 1);

    let sum_scores = |x: usize, y: usize, w: usize, h: usize| -> i32 {
        scores[x + w - 1][y + h - 1] - scores[x - 1][y + h - 1] - scores[x + w - 1][y - 1] + scores[x - 1][y - 1]
    };

    // Part 1
    let mut best: Option<(i32, usize, usize)> = None;
    for x in 1..=(SIZE - 2) {
        for y in 1..=(SIZE - 2) {
            let sum = sum_scores(x, y, 3, 3);
            best = best.max(Some((sum, x, y)));
        }
    }
    let (_, x, y) = best.unwrap();
    println!("{},{}", x, y);

    // Part 2
    let mut best: Option<(i32, usize, usize, usize)> = None;
    for x in 1..=SIZE {
        for y in 1..=SIZE {
            for s in 1..=(SIZE - x.max(y) + 1) {
                let sum = sum_scores(x, y, s, s);
                best = best.max(Some((sum, x, y, s)));
            }
        }
    }
    let (_, x, y, s) = best.unwrap();
    println!("{},{},{}", x, y, s);
}
