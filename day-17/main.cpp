#include <algorithm>

#include <cassert>
#include <climits>
#include <cstdio>

using namespace std;

const int W = 2000;

char map[W][W];
int min_x, max_x, min_y, max_y;

void print() {
    // swapped
    for (int y = min_y; y <= max_y; ++y) {
        for (int x = min_x; x <= max_x; ++x)
            printf("%c", map[x][y]);
        printf("\n");
    }
    printf("\n");
}

enum Direction {
    UP,
    LEFT,
    RIGHT
};

bool flood(int x, int y, Direction dir) {
    if (y + 1 > max_y || map[x][y + 1] == '|') {
        map[x][y] = '|';  // fall down
        return false;
    }
    if (map[x][y + 1] == '.') {
        if (!flood(x, y + 1, UP)) {
            map[x][y] = '|';
            return false;
        }
    }
    bool ans = true;
    if (dir == LEFT || dir == UP) {
        if (x + 1 > max_x || (map[x + 1][y] != '#' && !flood(x + 1, y, LEFT)))
            ans = false;
    }
    if (dir == RIGHT || dir == UP) {
        if (x - 1 < min_x || (map[x - 1][y] != '#' && !flood(x - 1, y, RIGHT)))
            ans = false;
    }
    if (dir == UP) {
        char fill = ans ? '~' : '|';
        for (int i = x - 1; i >= min_x && map[i][y] == '.'; --i)
            map[i][y] = fill;
        for (int i = x; i <= max_x && map[i][y] == '.'; ++i)
            map[i][y] = fill;
    }
//    if (dir == UP) print();
    return ans;
}

int main() {
    freopen("input.txt", "r", stdin);

    min_x = min_y = INT_MAX;
    max_x = max_y = 0;

    char ax1, ax2;
    int v, l, r;
    while (scanf("%c=%d, %c=%d..%d\n", &ax1, &v, &ax2, &l, &r) != EOF) {
        if (ax1 == 'x') {
            assert(ax2 == 'y');
            for (int y = l; y <= r; ++y)
                map[v][y] = '#';
            min_x = min(min_x, v);
            max_x = max(max_x, v);
            min_y = min(min_y, l);
            max_y = max(max_y, r);
        } else {
            assert(ax2 == 'x');
            for (int x = l; x <= r; ++x)
                map[x][v] = '#';
            min_x = min(min_x, l);
            max_x = max(max_x, r);
            min_y = min(min_y, v);
            max_y = max(max_y, v);
        }
    }
    --min_x, ++max_x;
    for (int x = min_x; x <= max_x; ++x)
        for (int y = min_y; y <= max_y; ++y)
            if (map[x][y] != '#')
                map[x][y] = '.';
//    print();

    // Parts 1 & 2
    flood(500, min_y - 1, UP);
    print();

    int total = 0, remain = 0;
    for (int x = min_x; x <= max_x; ++x) {
        for (int y = min_y; y <= max_y; ++y)
            if (map[x][y] == '~') {
                ++total, ++remain;
            } else if (map[x][y] == '|') {
                ++total;
            }
    }
    printf("%d\n", total);
    printf("%d\n", remain);
}
