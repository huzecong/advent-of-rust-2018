#include <algorithm>
#include <vector>

using namespace std;

const int N = 1100;

struct edge {
    int next, node;
} e[N * 100];
int head[N + 1], tot = 0;

inline void addedge(int a, int b) {
    e[++tot].next = head[a];
    head[a] = tot, e[tot].node = b;
}

struct Point {
    int x, y, z, w;
} p[N];

inline int distance(const Point &p, const Point &q) {
    return abs(p.x - q.x) + abs(p.y - q.y) + abs(p.z - q.z) + abs(p.w - q.w);
}

bool v[N];

void bfs(int x) {
    static int q[N];
    int h = 0, t = 0;
    q[t++] = x;
    v[x] = true;
    while (h < t) {
        int cur = q[h++];
        for (int i = head[cur]; i; i = e[i].next) {
            int node = e[i].node;
            if (v[node]) continue;
            q[t++] = node, v[node] = true;
        }
    }
}

int main() {
    freopen("input.txt", "r", stdin);

    int n = 0;
    while (true) {
        int x, y, z, w;
        if (scanf("%d,%d,%d,%d", &x, &y, &z, &w) == EOF) break;
        p[n++] = {x, y, z, w};
    }

    for (int i = 0; i < n; ++i)
        for (int j = i + 1; j < n; ++j)
            if (distance(p[i], p[j]) <= 3) {
                addedge(i, j);
                addedge(j, i);
            }

    int cnt = 0;
    for (int i = 0; i < n; ++i)
        if (!v[i]) bfs(i), ++cnt;

    printf("%d\n", cnt);

    return 0;
}
