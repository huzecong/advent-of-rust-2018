#include <queue>

#include <cstdio>

using namespace std;

const int dir[4][2] = {{-1, 0},
                       {0,  -1},
                       {0,  1},
                       {1,  0}};  // U, L, R, D

const int K = 40;
const int R = 8 + K;
const int C = 701 + K;
int map[R][C];
int erosion[R][C];
int depth, n, m;

int dist[R][C][3];

struct Node {
    int x, y, t, d;

    inline bool operator <(const Node &rhs) const {
        return d > rhs.d;
    }
};
priority_queue<Node> q;

int main() {
    freopen("input.txt", "r", stdin);

    scanf("depth: %d\ntarget:%d,%d", &depth, &n, &m);
    erosion[0][0] = depth % 20183;
    for (int i = 1; i < R; ++i)
        erosion[i][0] = (int)(((long long)i * 16807 + depth) % 20183);
    for (int j = 1; j < C; ++j)
        erosion[0][j] = (int)(((long long)j * 48271 + depth) % 20183);
    for (int i = 1; i < R; ++i)
        for (int j = 1; j < C; ++j) {
            if (i == n && j == m) erosion[n][m] = depth % 20183;
            else erosion[i][j] = (int)(((long long)erosion[i - 1][j] * erosion[i][j - 1] + depth) % 20183);
        }
    for (int i = 0; i < R; ++i)
        for (int j = 0; j < C; ++j)
            map[i][j] = erosion[i][j] % 3;

    // Part 1
    int sum = 0;
    for (int i = 0; i <= n; ++i)
        for (int j = 0; j <= m; ++j)
            sum += map[i][j];
    printf("%d\n", sum);

    // Part 2
    memset(dist, 0x3f, sizeof dist);
    dist[0][0][1] = 0;
    q.push({0, 0, 1, 0});
    while (!q.empty()) {
        Node cur = q.top();
        q.pop();
        if (cur.d > dist[cur.x][cur.y][cur.t]) continue;
        for (int nt = 0; nt < 3; ++nt) {
            if (nt == cur.t) continue;
            int nd = cur.d + 7;
            if (dist[cur.x][cur.y][nt] > nd) {
                dist[cur.x][cur.y][nt] = nd;
                q.push({cur.x, cur.y, nt, nd});
            }
        }
        for (auto d : dir) {
            int nx = cur.x + d[0], ny = cur.y + d[1];
            if (nx < 0 || ny < 0 || nx >= R || ny >= C) continue;
            for (int nt = 0; nt < 3; ++nt) {
                if (nt == map[nx][ny] || nt == map[cur.x][cur.y]) continue;
                int nd = cur.d + 1 + (nt == cur.t ? 0 : 7);
                if (dist[nx][ny][nt] > nd) {
                    dist[nx][ny][nt] = nd;
                    q.push({nx, ny, nt, nd});
                }
            }
        }
    }

    int min_dist = dist[n][m][1];
    printf("%d\n", min_dist);

    return 0;
}
