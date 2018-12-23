use std::fs;

fn can_react(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase()
            && (a.is_ascii_uppercase() ^ b.is_ascii_uppercase())
}

fn react(s: &Vec<char>) -> usize {
    let mut previous: Vec<i32> = (-1..(s.len() as i32 - 1)).collect::<Vec<_>>();
    let mut i = 0;
    let mut j = 1;
    let mut removed = 0;
    while j < s.len() {
        debug_assert!(i < j);
        if can_react(s[i], s[j]) {
            let prev = previous[i];
            previous[i + 1] = prev;
            removed += 2;
            if j + 1 < s.len() {
                previous[j + 1] = prev;
            }
            if prev != -1 {
                i = prev as usize;
                j += 1;
            } else {
                i = j + 1;
                j = i + 1;
            }
        } else {
            i = j;
            j = i + 1;
        }
    }
    return s.len() - removed;
}

fn main() {
    let input = fs::read_to_string("day-05/input.txt").ok().unwrap();
    let s: Vec<char> = input.trim().chars().collect();

    // Part 1
    let react_len = react(&s);
    println!("{}", react_len);

    // Part 2
    let min_react_len = (b'a'..=b'z').map(|c| c as char)
            .map(|ch| react(&s.iter()
                    .filter(|c| c.to_ascii_lowercase() != ch)
                    .map(|c| *c)
                    .collect::<Vec<_>>()))
            .min().unwrap();
    println!("{}", min_react_len);
}
