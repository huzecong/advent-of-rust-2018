from collections import defaultdict

dir = dict((d, i) for i, d in enumerate('NESW'))
delta = [(-1, 0), (0, 1), (1, 0), (0, -1)]

blocks = defaultdict(lambda: [False] * 4)


def walk(x, y, d):
    nx = x + delta[d][0]
    ny = y + delta[d][1]
    blocks[(x, y)][d] = True
    blocks[(nx, ny)][d ^ 2] = True  # reverse direction
    return nx, ny


def simulate(init_x, init_y, regex):
    stack = []
    total = set()
    current = {(init_x, init_y)}
    for ch in regex:
        if ch in ['(', '^']:
            stack.append((total, current.copy()))
            total = set()
        elif ch in [')', '$']:
            candidates = total | current
            total, current = stack.pop(-1)
            current |= candidates
        elif ch == '|':
            total |= current
            current = stack[-1][1].copy()
        else:
            d = dir[ch]
            current = {walk(x, y, d) for x, y in current}
    assert len(stack) == 0


def print_map(min_x, max_x, min_y, max_y):
    print('#' * ((max_y - min_y + 1) * 2 + 1))
    for x in range(min_x, max_x + 1):
        print('#', end='')
        for y in range(min_y, max_y + 1):
            print('X' if x == 0 and y == 0 else '.', end='')
            if y < max_y:
                print('|' if blocks[(x, y)][1] else '#', end='')
        print('#')
        if x < max_x:
            print('#', end='')
            for y in range(min_y, max_y + 1):
                print('-' if blocks[(x, y)][2] else '#', end='')
                print('#', end='')
            print()
    print('#' * ((max_y - min_y + 1) * 2 + 1))


def main():
    with open('input.txt', 'r') as f:
        regex = f.read().strip()

    simulate(0, 0, regex)

    min_x = min(x for x, _ in blocks.keys())
    max_x = max(x for x, _ in blocks.keys())
    min_y = min(y for _, y in blocks.keys())
    max_y = max(y for _, y in blocks.keys())

    def valid(x, y):
        return min_x <= x <= max_x and min_y <= y <= max_y

    queue = [(0, 0)]
    dist = {(0, 0): 0}
    while len(queue) > 0:
        x, y = queue.pop(0)
        for d in dir.values():
            if not blocks[(x, y)][d]:
                continue
            nx, ny = walk(x, y, d)
            if not valid(nx, ny) or (nx, ny) in dist:
                continue
            dist[(nx, ny)] = dist[(x, y)] + 1
            queue.append((nx, ny))
    # print_map(min_x, max_x, min_y, max_y)

    # Part 1
    print(max(dist.values()))

    # Part 2
    print(sum(int(d >= 1000) for d in dist.values()))


if __name__ == '__main__':
    main()
