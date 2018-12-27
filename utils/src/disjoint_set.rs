pub struct DisjointSet {
    size: usize,
    parent: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> DisjointSet {
        debug_assert!(size > 0);
        DisjointSet { size, parent: (0..size).collect::<Vec<_>>() }
    }

    pub fn find(&mut self, x: usize) -> usize {
        debug_assert!(x < self.size);
        return if x == self.parent[x] { x } else {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
            p
        };
    }

    pub fn merge(&mut self, x: usize, y: usize) {
        debug_assert!(x < self.size && y < self.size);
        let x = self.find(x);
        let y = self.find(y);
        self.parent[x] = y;
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        debug_assert!(x < self.size && y < self.size);
        return self.find(x) == self.find(y);
    }
}
