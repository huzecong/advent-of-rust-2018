#include <queue>

#include <cstdio>

using namespace std;

const int W = 40;
const int dir[4][2] = {{-1, 0},
                       {0,  -1},
                       {0,  1},
                       {1,  0}};  // U, L, R, D
const int INF = 0x3f3f3f3f;

char map[W][W], orig_map[W][W];
int hp[W][W], R, C, dist[W][W];
bool moved[W][W];

void print() {
    for (int i = 1; i <= R; ++i)
        printf("%s\n", map[i] + 1);
    printf("\n");
}

inline char get_target_type(char ch) {
    if (ch == 'G') return 'E';
    return 'G';
}

inline bool valid(int x, int y) {
    return x > 0 && x <= R && y > 0 && y <= C;
}

void floodfill(int x, int y) {
    queue<pair<int, int>> q;
    q.emplace(x, y);
    memset(dist, 0x3f, sizeof dist);
    dist[x][y] = 0;
    while (!q.empty()) {
        tie(x, y) = q.front();
        q.pop();
        for (auto d : dir) {
            int nx = x + d[0], ny = y + d[1];
            if (valid(nx, ny) && map[nx][ny] != '#' && dist[nx][ny] == INF) {
                dist[nx][ny] = dist[x][y] + 1;
                // only expand for spaces
                if (map[nx][ny] == '.') q.emplace(nx, ny);
            }
        }
    }
}

void exec_move(int fx, int fy, int tx, int ty) {
    assert(map[tx][ty] == '.');
    assert(hp[tx][ty] == 0);
    hp[tx][ty] = hp[fx][fy];
    hp[fx][fy] = 0;
    map[tx][ty] = map[fx][fy];
    map[fx][fy] = '.';
}

bool elf_died = false;  // bad design, but convenient

void exec_die(int x, int y) {
    assert(map[x][y] == 'G' || map[x][y] == 'E');
    assert(hp[x][y] <= 0);
    if (map[x][y] == 'E') elf_died = true;
    map[x][y] = '.';
    hp[x][y] = 0;
}

inline bool is_adjacent(int x, int y, char target_type) {
    for (auto d : dir) {
        int nx = x + d[0], ny = y + d[1];
        if (valid(nx, ny) && map[nx][ny] == target_type)
            return true;
    }
    return false;
}

// returns whether combat should end
bool exec_turn(int x, int y, int atk) {
    char target_type = get_target_type(map[x][y]);

    if (!is_adjacent(x, y, target_type)) {
        floodfill(x, y);
        int best_val = INF, tx = -1, ty = -1;
        bool found = false;
        for (int i = 1; i <= R; ++i)
            for (int j = 1; j <= C; ++j)
                if (map[i][j] == target_type) {
                    found = true;
                } else if (map[i][j] == '.' && is_adjacent(i, j, target_type)) {
                    // checking here is not correct: it could be the case when all enemies are surrounded by allies
//                    found = true;
                    if (dist[i][j] < best_val) {
                        best_val = dist[i][j];
                        tx = i, ty = j;
                    }
                }
        if (!found) {
//            print();
            return false;  // no targets on board, combat ends
        }
        if (best_val == INF) return true;  // no reachable targets, unit does nothing

        // need to move
        floodfill(tx, ty);
        best_val = INF;
        int mx = -1, my = -1;
        for (auto d : dir) {
            int nx = x + d[0], ny = y + d[1];
            if (valid(nx, ny) && map[nx][ny] == '.' && dist[nx][ny] < best_val) {
                best_val = dist[nx][ny];
                mx = nx, my = ny;
            }
        }
        assert(mx != -1 && my != -1);
        exec_move(x, y, mx, my);
        x = mx, y = my;
    }
    moved[x][y] = true;

    // find target to attack
    int best_val = INF;
    int mx = -1, my = -1;
    for (auto d : dir) {
        int nx = x + d[0], ny = y + d[1];
        if (valid(nx, ny) && map[nx][ny] == target_type && hp[nx][ny] < best_val) {
            best_val = hp[nx][ny];
            mx = nx, my = ny;
        }
    }
    if (best_val < INF) {
        if ((hp[mx][my] -= atk) <= 0)
            exec_die(mx, my);
    }

    return true;
}

int perform_check(int elf_atk, bool ignore_death = false) {
    memcpy(map, orig_map, sizeof map);
    for (int i = 1; i <= R; ++i) {
        for (int j = 1; j <= C; ++j)
            if (map[i][j] == 'G' || map[i][j] == 'E') {
                hp[i][j] = 200;
            } else hp[i][j] = 0;
    }

    int rounds = 0;
    elf_died = false;
    while (true) {
//        print();
        bool end = false;
        memset(moved, 0, sizeof moved);
        for (int i = 1; !end && i <= R; ++i)
            for (int j = 1; !end && j <= C; ++j)
                if (!moved[i][j] && (map[i][j] == 'G' || map[i][j] == 'E')) {
                    int atk = (map[i][j] == 'G') ? 3 : elf_atk;
                    end |= !exec_turn(i, j, atk);
                    if (!ignore_death && elf_died) return -1;
                }
        if (end) break;
        ++rounds;
    }
    int total_hp = 0;
    for (int i = 1; i <= R; ++i) {
        for (int j = 1; j <= C; ++j)
            total_hp += hp[i][j];
    }
//    print();
//    printf("%d %d\n", rounds, total_hp);
    return rounds * total_hp;
}

int main() {
    freopen("input.txt", "r", stdin);

    for (R = 1; scanf("%s", orig_map[R] + 1) != EOF; ++R);
    --R;
    C = static_cast<int>(strlen(orig_map[1] + 1));

    // Part 1
    printf("%d\n", perform_check(3, true));

    // Part 2
    for (int atk = 4; ; ++atk) {
        int result = perform_check(atk);
        if (result != -1) {
//            printf("%d\n", atk);
            printf("%d\n", result);
            break;
        }
    }

    return 0;
}
