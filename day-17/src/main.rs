use std::fs;

#[derive(Eq, PartialEq)]
enum Direction {
    UP,
    LEFT,
    RIGHT,
}

struct Bar {
    x_l: usize,
    x_r: usize,
    y_l: usize,
    y_r: usize,
}

fn parse_bar(s: &str) -> Bar {
    let p = s.find(", ").unwrap();
    let ax1 = &s[..1];
    let ax2 = &s[(p + 2)..(p + 3)];
    let v = s[2..p].parse::<usize>().unwrap();
    let range = s[(p + 4)..].split("..").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (l, r) = (range[0], range[1]);
    if ax1 == "x" {
        debug_assert!(ax2 == "y");
        Bar { x_l: v, x_r: v, y_l: l, y_r: r }
    } else {
        debug_assert!(ax2 == "x");
        Bar { x_l: l, x_r: r, y_l: v, y_r: v }
    }
}

struct Board {
    map: Vec<Vec<char>>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Board {
    pub fn new(bars: &Vec<Bar>) -> Board {
        let min_x = bars.iter().map(|b| b.x_l).min().unwrap() - 1;
        let max_x = bars.iter().map(|b| b.x_r).max().unwrap() + 1;
        let min_y = bars.iter().map(|b| b.y_l).min().unwrap();
        let max_y = bars.iter().map(|b| b.y_r).max().unwrap();
        let mut map = vec![vec!['.'; max_y + 1]; max_x + 1];
        for Bar { x_l, x_r, y_l, y_r } in bars {
            for x in *x_l..=*x_r {
                for y in *y_l..=*y_r {
                    map[x][y] = '#';
                }
            }
        }
        Board { map, min_x, max_x, min_y, max_y }
    }

    /// Returns whether the water at (x, y) is still water
    fn flood_impl(&mut self, x: usize, y: usize, dir: Direction) -> bool {
        if y + 1 > self.max_y || self.map[x][y + 1] == '|' {
            self.map[x][y] = '|';  // fall down
            return false;
        }
        if self.map[x][y + 1] == '.' && !self.flood_impl(x, y + 1, Direction::UP) {
            self.map[x][y] = '|';
            return false;
        }
        let mut ans = true;
        if dir == Direction::LEFT || dir == Direction::UP {
            if x + 1 > self.max_x || (self.map[x + 1][y] != '#' && !self.flood_impl(x + 1, y, Direction::LEFT)) {
                ans = false;
            }
        }
        if dir == Direction::RIGHT || dir == Direction::UP {
            if x - 1 < self.min_x || (self.map[x - 1][y] != '#' && !self.flood_impl(x - 1, y, Direction::RIGHT)) {
                ans = false;
            }
        }
        if dir == Direction::UP {
            let fill = if ans { '~' } else { '|' };
            for i in (self.min_x..x).rev() {
                if self.map[i][y] != '.' { break; }
                self.map[i][y] = fill;
            }
            for i in x..=self.max_x {
                if self.map[i][y] != '.' { break; }
                self.map[i][y] = fill;
            }
        }
        return ans;
    }

    pub fn flood(&mut self) {
        self.flood_impl(500, self.min_y - 1, Direction::UP);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in self.min_y..=self.max_y {
            println!("{}", (self.min_x..=self.max_x).map(|x| self.map[x][y]).collect::<String>());
        }
        println!();
    }

    pub fn count(&self, c: char) -> usize {
        (self.min_x..=self.max_x)
                .map(|x| (self.min_y..=self.max_y)
                        .filter(|&y| self.map[x][y] == c).count())
                .sum()
    }
}

fn main() {
    let input = fs::read_to_string("day-17/input.txt").unwrap();
    let bars = input.lines().map(parse_bar).collect::<Vec<_>>();

    // Parts 1 & 2
    let mut board = Board::new(&bars);
    board.flood();
//    board.print();

    let remain = board.count('~');
    let total = remain + board.count('|');

    println!("{}", total);
    println!("{}", remain);
}
