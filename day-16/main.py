import operator

op_types = ['addr', 'addi',
            'mulr', 'muli',
            'banr', 'bani',
            'borr', 'bori',
            'setr', 'seti',
            'gtir', 'gtri', 'gtrr',
            'eqir', 'eqri', 'eqrr']

op_lambda = {
    'add': operator.add,
    'mul': operator.mul,
    'ban': operator.and_,
    'bor': operator.or_,
    'gt': operator.gt,
    'eq': operator.eq,
}


def convert(line):
    return [int(x) for x in ''.join(line.split()[1:])[1:-1].split(',')]


def parse(line):
    return [int(_) for _ in line.split()]


def exec_op(op_type, register, op_code):
    _, a, b, c = op_code
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


def main():
    with open('input.txt') as f:
        lines = f.read().split('\n')
    samples = []
    program = []
    for x in range(0, len(lines), 4):
        if not lines[x].startswith("Before:"):
            # ugly input handling
            program = [parse(l) for l in lines[x:] if l.strip() != '']
            break
        before = convert(lines[x])
        op = parse(lines[x + 1])
        after = convert(lines[x + 2])
        samples.append((before, op, after))

    # Part 1
    count = 0
    possible_ops = [set(op_types) for _ in range(16)]
    for before, op, after in samples:
        valid_ops = []
        for op_type in op_types:
            after_exec = exec_op(op_type, before, op)
            if after_exec == after:
                valid_ops.append(op_type)
        if len(valid_ops) >= 3:
            count += 1
        possible_ops[op[0]] &= set(valid_ops)

    print(count)

    # Part 2
    op_map = [None for _ in range(16)]
    while any(op is None for op in op_map):
        for idx, p_set in enumerate(possible_ops):
            if len(p_set) == 1:
                op_map[idx] = list(p_set)[0]
        confirmed_ops = [x for x in op_map if x is not None]
        for p_set in possible_ops:
            p_set -= set(confirmed_ops)
    # print(op_map)
    registers = [0 for _ in range(4)]
    for op in program:
        registers = exec_op(op_map[op[0]], registers, op)
    # print(registers)
    print(registers[0])


if __name__ == '__main__':
    main()
