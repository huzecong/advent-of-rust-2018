use std::collections::hash_map::{DefaultHasher, HashMap};
use std::fs;
use std::hash::{Hash, Hasher};

const N_ROUNDS: usize = 10;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Board {
    board: Vec<Vec<char>>,
    r: usize,
    c: usize,
}

const DIRS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

impl Board {
    fn new(board: Vec<Vec<char>>) -> Board {
        let r = board.len();
        let c = board[0].len();
        Board { board, r, c }
    }

    fn iter_adjacent(&self, x: usize, y: usize) -> impl Iterator<Item=(usize, usize)> + '_ {
        DIRS.iter().filter_map(move |(dx, dy)|
                if (x == 0 && *dx < 0) || (x == self.r - 1 && *dx > 0)
                        || (y == 0 && *dy < 0) || (y == self.c - 1 && *dy > 0) { None } else {
                    Some((((x as i32) + *dx) as usize, ((y as i32) + *dy) as usize))
                })
    }

    fn count_adjacent(&self, x: usize, y: usize, ch: char) -> usize {
        self.iter_adjacent(x, y).filter(|(x, y)| self.board[*x][*y] == ch).count()
    }

    fn exec_round(&mut self) {
        let mut board = self.board.clone();
        for x in 0..self.r {
            for y in 0..self.c {
                board[x][y] = match self.board[x][y] {
                    '.' => if self.count_adjacent(x, y, '|') >= 3 { '|' } else { '.' },
                    '|' => if self.count_adjacent(x, y, '#') >= 3 { '#' } else { '|' },
                    '#' => if self.count_adjacent(x, y, '#') >= 1 && self.count_adjacent(x, y, '|') >= 1 { '#' } else { '.' },
                    _ => panic!(),
                }
            }
        }
        self.board = board;
    }

    fn count_resource(&self) -> usize {
        let mut wood_cnt = 0;
        let mut lumber_cnt = 0;
        for x in 0..self.r {
            for y in 0..self.c {
                match self.board[x][y] {
                    '#' => lumber_cnt += 1,
                    '|' => wood_cnt += 1,
                    _ => (),
                }
            }
        }
        return wood_cnt * lumber_cnt;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for x in 0..self.r {
            println!("{}", self.board[x].iter().collect::<String>());
        }
        println!();
    }
}

fn main() {
    let input = fs::read_to_string("day-18/input.txt").unwrap();
    let board = input.lines().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let board = Board::new(board);

    // Part 1
    let mut cur_board = board.clone();
    for _ in 0..N_ROUNDS {
        cur_board.exec_round();
    }
    println!("{}", cur_board.count_resource());

    // Part 2
    let mut h = HashMap::<u64, usize>::new();
    let mut cur_board = board.clone();
    let mut rounds = 0;
    let cycle_start;
    let cycle_length;
    loop {
        let mut hasher = DefaultHasher::new();
        cur_board.hash(&mut hasher);
        let hash = hasher.finish();

        let prev = h.get(&hash);
        if let Some(r) = prev {
            cycle_start = *r;
            cycle_length = rounds - *r;
            break;
        } else {
            h.insert(hash, rounds);
            cur_board.exec_round();
            rounds += 1;
        }
    }

    let total_rounds = 1000000000;
    let rem_rounds = (total_rounds - cycle_start) % cycle_length;
    for _ in 0..rem_rounds {
        cur_board.exec_round();
    }
    println!("{}", cur_board.count_resource());
}
