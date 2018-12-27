use std::fs;

const N_REGS: usize = 6;

type Registers = [i32; N_REGS];

struct Instruction(String, i32, i32, i32);

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

fn parse_instruction(s: &str) -> Instruction {
    let mut iter = s.split_whitespace();
    let op = iter.next().unwrap().to_string();
    let params = iter.map(parse_i32).collect::<Vec<_>>();
    Instruction(op, params[0], params[1], params[2])
}

fn exec(ins: &Instruction, regs: Registers) -> Registers {
    let mut new_regs = clone(regs.iter().map(|x| *x));
    let op_type = &ins.0;
    let (a, b, c) = (ins.1, ins.2, ins.3 as usize);
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

fn exec_program(program: &Vec<Instruction>, ip_reg: usize,
                init_regs: Registers, breakpoint: Option<usize>) -> Registers {
    let mut regs = clone(init_regs.iter().map(|x| *x));
    let mut ip = 0i32;
    while ip >= 0 && ip < program.len() as i32 {
        regs[ip_reg] = ip;
        regs = exec(&program[ip as usize], regs);
        if breakpoint.is_some() && breakpoint.unwrap() == ip as usize {
            return regs;
        }
        ip = regs[ip_reg] + 1;
    }
    return regs;
}

fn sum_divisors(x: usize) -> usize {
    let mut sum = 0;
    for i in 1..((x as f64).sqrt() as usize) {
        if x % i != 0 { continue; }
        sum += i;
        let d = x / i;
        if d != i { sum += d; }
    }
    return sum;
}

fn main() {
    let input = fs::read_to_string("day-19/input.txt").unwrap();
    let mut lines = input.lines();
    let ip_reg = lines.next().unwrap()
            .split_whitespace().skip(1).next().unwrap()
            .parse::<usize>().unwrap();
    let program: Vec<Instruction> = lines.map(parse_instruction).collect::<Vec<_>>();

    // Part 1
    let regs = [0; N_REGS];
    let regs = exec_program(&program, ip_reg, regs, Some(1));
    println!("{}", sum_divisors(regs[5] as usize));

    // Part 2
    let mut regs = [0; N_REGS];
    regs[0] = 1;
    let regs = exec_program(&program, ip_reg, regs, Some(1));
    println!("{}", sum_divisors(regs[5] as usize));
}
