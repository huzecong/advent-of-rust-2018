#include <map>

#include <cstdio>

using namespace std;

const int W = 60;
const int dir[8][2] = {{-1, 0},
                       {0,  -1},
                       {0,  1},
                       {1,  0},
                       {-1, -1},
                       {-1, 1},
                       {1,  -1},
                       {1,  1}};

char new_board[W][W], board[W][W], orig_board[W][W];
int R, C;

void print() {
    for (int i = 1; i <= R; ++i)
        printf("%s\n", board[i] + 1);
    printf("\n");
}

int count_adjacent(int x, int y, char ch) {
    int cnt = 0;
    for (auto d : dir) {
        int nx = x + d[0], ny = y + d[1];
        if (board[nx][ny] == ch) ++cnt;
    }
    return cnt;
}

inline int get_val(int x, int y) {
    if (board[x][y] == '.') return 0;
    if (board[x][y] == '|') return 1;
    return 2;
}

int get_hash(int base, int mod) {
    int h = 0;
    for (int i = 1; i <= R; ++i)
        for (int j = 1; j <= C; ++j)
            h = (int)(((long long)h * base + get_val(i, j)) % mod);
    return h;
}

void exec_round() {
    for (int i = 1; i <= R; ++i)
        for (int j = 1; j <= C; ++j) {
            if (board[i][j] == '.') {
                new_board[i][j] = count_adjacent(i, j, '|') >= 3 ? '|' : '.';
            } else if (board[i][j] == '|') {
                new_board[i][j] = count_adjacent(i, j, '#') >= 3 ? '#' : '|';
            } else if (board[i][j] == '#') {
                new_board[i][j] = count_adjacent(i, j, '#') >= 1 && count_adjacent(i, j, '|') >= 1 ? '#' : '.';
            }
        }
    memcpy(board, new_board, sizeof board);
}

int count_resource() {
    int wood_cnt = 0, lumber_cnt = 0;
    for (int i = 1; i <= R; ++i)
        for (int j = 1; j <= C; ++j) {
            if (board[i][j] == '#') ++lumber_cnt;
            else if (board[i][j] == '|') ++wood_cnt;
        }
    return wood_cnt * lumber_cnt;
}

int main() {
    freopen("input.txt", "r", stdin);

    for (R = 1; scanf("%s", board[R] + 1) != EOF; ++R);
    --R;
    C = static_cast<int>(strlen(board[1] + 1));
    memcpy(orig_board, board, sizeof board);

    // Part 1
    for (int rounds = 1; rounds <= 10; ++rounds)
        exec_round();
    printf("%d\n", count_resource());

    // Part 2
    memcpy(board, orig_board, sizeof board);
    map<pair<int, int>, int> h;
    int cycle_start = -1, cycle_length = -1;
    for (int rounds = 0; ; ++rounds) {
        int hash1 = get_hash(3, 19260817);
        int hash2 = get_hash(131, (int)1e9 + 7);
        auto hash = make_pair(hash1, hash2);
        if (h.find(hash) != h.end()) {
            cycle_start = h[hash];
            cycle_length = rounds - cycle_start;
            break;
        }
        h[hash] = rounds;
        exec_round();
    }
    printf("%d %d\n", cycle_start, cycle_length);
    int total_rounds = 1000000000;
    int rem_rounds = (total_rounds - cycle_start) % cycle_length;
    while (rem_rounds-- > 0)
        exec_round();
    printf("%d\n", count_resource());

    return 0;
}
