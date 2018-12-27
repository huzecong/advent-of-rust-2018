use std::collections::HashSet;

fn main() {
    let mut h = HashSet::<u32>::new();
    let mut previous = 0;

    let mut r3 = 65536;
    let mut r4 = 16098955;
    loop {
        r4 = (r4 + (r3 & 255)) & 16777215;
        r4 = ((r4 as u64 * 65899) & 16777215) as u32;
        if r3 < 256 {
            if h.len() == 0 {
                // Part 1
                println!("{}", r4);
            }
            if h.contains(&r4) {
                // Part 2
                println!("{}", previous);
                break;
            }
            h.insert(r4);
            previous = r4;
            r3 = r4 | 65536;
            r4 = 16098955;
        } else {
            // find least multiple of 256 > r3
            r3 = r3 / 256;
        }
    }
}
