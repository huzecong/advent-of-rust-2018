#![feature(const_slice_len)]

use std::collections::HashSet;
use std::fs;

const N_REGS: usize = 4;

type Registers = [i32; N_REGS];
type Instruction = [i32; N_REGS];

struct Sample {
    before: Registers,
    after: Registers,
    ins: Instruction,
}

const OP_TYPES: [&str; 16] = [
    "addr", "addi",
    "mulr", "muli",
    "banr", "bani",
    "borr", "bori",
    "setr", "seti",
    "gtir", "gtri", "gtrr",
    "eqir", "eqri", "eqrr",
];
const N_OPS: usize = OP_TYPES.len();

fn clone<I>(iter: I) -> [i32; N_REGS] where
        I: Iterator<Item=i32> {
    let mut arr = [0; N_REGS];
    for (i, x) in iter.enumerate() {
        arr[i] = x;
    }
    return arr;
}

fn parse_i32(s: &str) -> i32 {
    s.parse::<i32>().ok().unwrap()
}

fn parse_registers(s: &str) -> Registers {
    clone(s[9..(s.len() - 1)].split(", ").map(parse_i32))
}

fn parse_instruction(s: &str) -> Instruction {
    clone(s.split_whitespace().map(parse_i32))
}

fn parse_sample(s: &str) -> Sample {
    let lines = s.lines().collect::<Vec<_>>();
    Sample {
        before: parse_registers(lines[0]),
        after: parse_registers(lines[2]),
        ins: parse_instruction(lines[1]),
    }
}

fn exec(op_type: &str, regs: Registers, ins: Instruction) -> Registers {
    let mut new_regs = clone(regs.iter().map(|x| *x));
    let (a, b, c) = (ins[1], ins[2], ins[3] as usize);
    match &op_type[..2] {
        "ad" | "mu" | "ba" | "bo" => {
            let a = regs[a as usize];
            let b = if &op_type[3..4] == "r" { regs[b as usize] } else { b };
            new_regs[c] = match &op_type[..3] {
                "add" => a + b,
                "mul" => a * b,
                "ban" => a & b,
                "bor" => a | b,
                _ => { panic!(); }
            };
        }
        "se" => {
            let a = if &op_type[3..4] == "r" { regs[a as usize] } else { a };
            new_regs[c] = a;
        }
        "gt" | "eq" => {
            let a = if &op_type[2..3] == "r" { regs[a as usize] } else { a };
            let b = if &op_type[3..4] == "r" { regs[b as usize] } else { b };
            new_regs[c] = match &op_type[..2] {
                "gt" => (a > b) as i32,
                "eq" => (a == b) as i32,
                _ => { panic!(); }
            };
        }
        _ => { panic!(); }
    };
    return new_regs;
}

fn main() {
    let input = fs::read_to_string("day-16/input.txt").ok().unwrap();
    let input_parts = input.split("\n\n\n").collect::<Vec<_>>();

    let samples: Vec<Sample> = input_parts[0].split("\n\n").map(parse_sample).collect::<Vec<_>>();
    let program: Vec<Instruction> = input_parts[1].lines().map(parse_instruction).collect::<Vec<_>>();

    // Part 1
    let mut n_valid_samples = 0;
    let mut valid_ops = [[true; N_OPS]; N_OPS];
    for sample in samples.iter() {
        let mut n_valid = 0;
        let op_code = sample.ins[0] as usize;
        for (i, op_type) in OP_TYPES.iter().enumerate() {
            let regs_after = exec(op_type, sample.before, sample.ins);
            if regs_after == sample.after {
                n_valid += 1;
            } else {
                valid_ops[op_code][i] = false;
            }
        }
        if n_valid >= 3 { n_valid_samples += 1; }
    }
    println!("{}", n_valid_samples);

    // Part 2
    let mut valid_ops = valid_ops.iter()
            .map(|valid| valid.iter().enumerate()
                    .filter_map(|(i, &x)| if x { Some(i) } else { None })
                    .collect::<HashSet<_>>())
            .collect::<Vec<_>>();
    let mut op_map = [None as Option<usize>; N_OPS];
    while op_map.iter().any(|x| x.is_none()) {
        let mut confirmed_ops = HashSet::<usize>::new();
        for op_code in 0..N_OPS {
            if valid_ops[op_code].len() == 1 {
                let op_idx = *valid_ops[op_code].iter().next().unwrap();
                op_map[op_code] = Some(op_idx);
                confirmed_ops.insert(op_idx);
            }
        }
        for i in 0..valid_ops.len() {
            valid_ops[i] = valid_ops[i].difference(&confirmed_ops).cloned().collect();
        }
    }
    let op_map = op_map.iter().map(|x| x.unwrap()).collect::<Vec<_>>();

    let mut regs = [0; N_REGS];
    for ins in program {
        regs = exec(OP_TYPES[op_map[ins[0] as usize]], regs, ins);
    }
    println!("{}", regs[0]);
}
