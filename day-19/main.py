import math
import operator

op_lambda = {
    'add': operator.add,
    'mul': operator.mul,
    'ban': operator.and_,
    'bor': operator.or_,
    'gt': operator.gt,
    'eq': operator.eq,
}


def parse(line):
    parts = line.split()
    return (parts[0],) + tuple(int(_) for _ in parts[1:])


def exec_op(op_code, register):
    op_type, a, b, c = op_code
    register = register.copy()
    if op_type[:3] in ['add', 'mul', 'ban', 'bor']:
        a = register[a]
        b = register[b] if op_type[3] == 'r' else b
        register[c] = op_lambda[op_type[:3]](a, b)
    elif op_type[:3] == 'set':
        a = register[a] if op_type[3] == 'r' else a
        register[c] = a
    elif op_type[:2] in ['gt', 'eq']:
        a = register[a] if op_type[2] == 'r' else a
        b = register[b] if op_type[3] == 'r' else b
        register[c] = int(op_lambda[op_type[:2]](a, b))
    return register


def exec_program(program, ip_reg, init_registers, breakpoint=None):
    registers = init_registers.copy()
    ip = 0
    while 0 <= ip < len(program):
        op = program[ip]
        registers[ip_reg] = ip
        registers = exec_op(op, registers)
        if breakpoint is not None and ip == breakpoint:
            return registers
        ip = registers[ip_reg] + 1
    return registers


def sum_divisors(x):
    s = 0
    for i in range(1, int(math.sqrt(x))):
        if x % i == 0:
            s += i
            if x // i != i:
                s += x // i
    return s

def main():
    with open('input.txt') as f:
        lines = f.read().strip().split('\n')
    program = [parse(line) for line in lines[1:]]
    ip_reg = int(lines[0].split()[1])

    # Part 1
    registers = exec_program(program, ip_reg, [0] * 6, breakpoint=1)
    print(sum_divisors(registers[5]))

    # Part 2
    registers = exec_program(program, ip_reg, [1] + [0] * 5, breakpoint=1)
    print(sum_divisors(registers[5]))


if __name__ == '__main__':
    main()
