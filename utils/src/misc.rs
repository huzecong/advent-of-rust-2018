use std::ops::{Add, AddAssign, Sub};
use std::collections::HashMap;

pub fn cumsum<T>(arr: &mut Vec<Vec<T>>, width: usize, height: usize) where
        T: Add<T, Output=T> + AddAssign<T> + Sub<T, Output=T> + Copy {
    for i in 1..width {
        let val = arr[i - 1][0];  // otherwise it's an immutable borrow
        arr[i][0] += val;
    }
    for j in 1..height {
        let val = arr[0][j - 1];
        arr[0][j] += val;
    }
    for i in 1..width {
        for j in 1..height {
            let val = arr[i - 1][j] + arr[i][j - 1] - arr[i - 1][j - 1];
            arr[i][j] += val;
        }
    }
}

pub fn counter<I, T>(iter: I) -> HashMap<T, i32> where
        I: Iterator<Item=T>, T: std::cmp::Eq, T: std::hash::Hash {
    let mut h = HashMap::new();
    for x in iter {
        *h.entry(x).or_default() += 1;
    }
    return h;
}
