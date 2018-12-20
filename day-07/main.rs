use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

struct Edge(usize, usize);

fn parse(s: &str) -> Edge {
    let parts = s.trim().split_whitespace().collect::<Vec<_>>();
    let f = |idx: usize| parts[idx].chars().next().unwrap() as usize - 'A' as usize;
    Edge(f(1), f(7))
}

#[derive(Eq, PartialEq)]
struct Val<T: Ord>(T);

impl<T: Ord> Ord for Val<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: Ord> PartialOrd for Val<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MinHeap<T: Ord> {
    heap: BinaryHeap<Val<T>>
}

impl<T: Ord> MinHeap<T> {
    fn new() -> MinHeap<T> {
        MinHeap { heap: BinaryHeap::<Val<T>>::new() }
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn push(&mut self, item: T) {
        self.heap.push(Val(item));
    }

    fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Val(x)| x)
    }
}

struct DAG<'a> {
    n: usize,
    queue: MinHeap<usize>,
    in_deg: Vec<usize>,
    out_edges: &'a Vec<Vec<usize>>,
}

impl<'a> DAG<'a> {
    fn new(out_edges: &'a Vec<Vec<usize>>) -> DAG {
        let n = out_edges.len();
        let mut in_deg = vec![0; n];
        for edges in out_edges.iter() {
            for &x in edges.iter() {
                in_deg[x] += 1;
            }
        }
        let mut queue = MinHeap::<usize>::new();
        for (i, &deg) in in_deg.iter().enumerate() {
            if deg == 0 { queue.push(i); }
        }
        DAG { n, queue, in_deg, out_edges }
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn pop(&mut self) -> usize {
        self.queue.pop().unwrap()
    }

    fn remove(&mut self, item: usize) {
        for &y in self.out_edges[item].iter() {
            self.in_deg[y] -= 1;
            if self.in_deg[y] == 0 { self.queue.push(y); }
        }
    }
}

impl<'a> Iterator for DAG<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let x = self.pop();
        self.remove(x);
        return Some(x);
    }

    fn count(self) -> usize where Self: Sized {
        self.n
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").ok().unwrap();
    let edges: Vec<Edge> = input.lines().map(parse).collect();
    let n = edges.iter().map(|Edge(a, b)| a.max(b)).max().unwrap() + 1;

    // Part 1
    let mut out_edges = vec![Vec::<usize>::new(); n];
    for &Edge(a, b) in edges.iter() {
        out_edges[a].push(b);
    }

    let char_seq = DAG::new(&out_edges)
            .map(|x| (x as u8 + 'A' as u8) as char)
            .collect::<String>();
    println!("{}", char_seq);

    // Part 2
    let mut work_queue = MinHeap::<(usize, usize)>::new();
    let n_workers = 5;
    let mut dag = DAG::new(&out_edges);
    let mut free_workers = n_workers;
    let mut current_time = 0;

    while free_workers > 0 && !dag.is_empty() {
        let x = dag.pop();
        work_queue.push((61 + x, x));
        free_workers -= 1;
    }

    while !work_queue.is_empty() {
        let (time, x) = work_queue.pop().unwrap();
        free_workers += 1;
        current_time = time;
        dag.remove(x);
        while free_workers > 0 && !dag.is_empty() {
            let x = dag.pop();
            work_queue.push((time + 61 + x, x));
            free_workers -= 1;
        }
    }

    println!("{}", current_time);
}
