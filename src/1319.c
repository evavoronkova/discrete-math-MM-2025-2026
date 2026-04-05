#include <assert.h>

#pragma pack(1)
#define SIZE 100000
typedef unsigned short ushort;
ushort parents[SIZE] = {0};
ushort sizes[SIZE];

int findParent(int x) {
    return ( x == parents[x] ? x : (parents[x] = findParent(parents[x])));
}
int makeConnected(int n, int** connections, int connectionsSize, int* connectionsColSize) {
    if (n > connectionsSize + 1)
        return -1;

    for (int i = 0; i < n; ++i) {
        parents[i] = i;
        sizes[i] = 1;
    }

    int amountEqualityClasses = n;
    for (int i = 0; i < connectionsSize; ++i) {
        int rootFirst = findParent(connections[i][0]);
        int rootSecond = findParent(connections[i][1]);

        if (rootFirst != rootSecond) {
            if (sizes[rootFirst] < sizes[rootSecond]) {
                parents[rootFirst] = rootSecond;
                sizes[rootSecond] += sizes[rootFirst];
            }
            else {
                parents[rootSecond] = rootFirst;
                sizes[rootFirst] += sizes[rootSecond];
            }
            amountEqualityClasses--;
        }
    }
    return amountEqualityClasses - 1;
}

void test1() {
    int **connections = (int* []){ (int []){0, 1}, (int []){0, 2}, (int []){1, 2} };
    int res = makeConnected(4, connections, 3, 0);
    assert(res == 1);
}
void test2() {
    int **connections = (int* []){ (int []){0, 1}, (int []){0, 2}, (int []){0, 3}, (int []){1, 2}, (int []){1, 3} };
    int res = makeConnected(6, connections, 5, 0);
    assert(res == 2);
}
void test3() {
    int **connections = (int* []){ (int []){0, 1}, (int []){0, 2}, (int []){0, 3}, (int []){1, 2} };
    int res = makeConnected(6, connections, 4, 0);
    assert(res == -1);
}
int main() {
    test1();
    test2();
    test3();
}
