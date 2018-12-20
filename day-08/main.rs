#![feature(box_syntax)]

use std::fs;

struct Node {
    size: usize,
    children: Vec<Box<Node>>,
    metadata: Vec<u32>,
}

impl Node {
    fn new(s: &[u32]) -> Node {
        let n_child = s[0] as usize;
        let n_meta = s[1] as usize;
        let mut size = 2;
        let mut children: Vec<Box<Node>> = vec![];
        for _ in 0..n_child {
            let child = Node::new(&s[size..]);
            size += child.size;
            children.push(box child);
        }
        let metadata = s[size..(size + n_meta)].to_vec();
        size += n_meta;
        Node { size, children, metadata }
    }
}

fn sum_meta(node: &Node) -> u32 {
    let sum = node.metadata.iter().fold(0, |a, &b| a + b);
    sum + node.children.iter().map(|x| sum_meta(x)).sum::<u32>()
}

fn sum_value(node: &Node) -> u32 {
    if node.children.len() == 0 {
        node.metadata.iter().fold(0, |a, &b| a + b)
    } else {
        let mut count = vec![0; node.children.len()];
        for &x in node.metadata.iter() {
            let x = x as usize;
            if x <= count.len() { count[x - 1] += 1; }
        }
        node.children.iter().zip(count.iter())
                .map(|(c, &x)| if x == 0 {0} else {
                    x * sum_value(c)
                }).sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").ok().unwrap();
    let tree_seq = input.trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().ok().unwrap())
            .collect::<Vec<_>>();
    let root = Node::new(&tree_seq);

    // Part 1
    let sum_meta = sum_meta(&root);
    println!("{}", sum_meta);

    // Part 2
    let sum_value = sum_value(&root);
    println!("{}", sum_value);
}
