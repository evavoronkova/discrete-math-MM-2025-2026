#include <algorithm>
#include <cassert>
#include <vector>
using namespace std;

#define N 61
struct region {
    int x1 = N, y1 = N, x2 = 0, y2 = 0;
    int color = 0;
};
vector<vector<int>> grid;
class Solution {
public:
    static int getMinOnSection(int a, int b, int c, int d) {
        return max(a, c);
    }
    static int getMaxOnSection(int a, int b, int c, int d) {
        return min(b, d);
    }
    static bool regionComparer(const region &a, const region &b) {
        // Случай, когда один из прямоугольников целиком лежит в другом
        if (a.x1 <= b.x1 && b.x2 <= a.x2 && a.y1 <= b.y1 && b.y2 <= a.y2) return true;
        if (b.x1 <= a.x1 && a.x2 <= b.x2 && b.y1 <= a.y1 && a.y2 <= b.y2) return false;

        // Случай, когда прямоугольники не пересекаются
        if (a.x2 < b.x1 || a.y2 < b.y1 || b.x2 < a.x1 || b.y2 < a.y1) return false;

        // Иначе они пересекаются
        int iMin = getMinOnSection(a.x1, a.x2, b.x1, b.x2);
        int jMin = getMinOnSection(a.y1, a.y2, b.y1, b.y2);
        int iMax = getMaxOnSection(a.x1, a.x2, b.x1, b.x2);
        int jMax = getMaxOnSection(a.y1, a.y2, b.y1, b.y2);
        for (int i = iMin; i <= iMax; i++) {
            for (int j = jMin; j <= jMax; j++) {
                if (grid[i][j] == b.color) return true;
                else if (grid[i][j] == a.color) return false;
            }
        }
        return false;
    }
    bool isPrintable(vector<vector<int>>& targetGrid) {
        grid = targetGrid;
        regions = vector<region>(N);
        for (int i = 0; i < N; i++)
            regions[i].color = i;
        for (int i = 0; i < targetGrid.size(); i++) {
            for (int j = 0; j < targetGrid[0].size(); j++) {
                int col = targetGrid[i][j];
                n = max(n, col);

                region& reg = regions[col];
                reg.x1 = min(reg.x1, i);
                reg.y1 = min(reg.y1, j);
                reg.x2 = max(reg.x2, i);
                reg.y2 = max(reg.y2, j);
            }
        }
        stable_sort(regions.begin(), regions.end(), regionComparer);
        int newGrid[N][N];
        for (int i = 0; i < n; i++) {
            region& reg = regions[i];
            for (int x = reg.x1; x <= reg.x2; x++) {
                for (int y = reg.y1; y <= reg.y2; y++) {
                    newGrid[x][y] = reg.color;
                }
            }
        }
        bool isIdentical = true;
        for (int i = 0; i < targetGrid.size(); i++) {
            for (int j = 0; j < targetGrid[0].size(); j++) {
                if (newGrid[i][j] != targetGrid[i][j]) {
                    isIdentical = false;
                    // printf("x=%d, y=%d: Color %d != %d\n", i, j, targetGrid[i][j], newGrid[i][j]);
                    goto solve;
                }
            }
        }
        solve:
        return isIdentical;
    }
    int n;

    vector<region> regions;
};
auto sol = Solution();
void test1() {
    vector<vector<int>> targetGrid = {{1, 1, 1, 1}, {1, 2, 2, 1}, {1, 2, 2, 1}, {1, 1, 1, 1}};
    int res = sol.isPrintable(targetGrid);
    assert(res == true);
}
void test2() {
    vector<vector<int>> targetGrid = {{1, 1, 1, 1},{1, 1, 3, 3},{1, 1, 3, 4},{5, 5, 1, 4}};
    int res = sol.isPrintable(targetGrid);
    assert(res == true);
}
void test3() {
    vector<vector<int>> targetGrid = {{1, 2, 1}, {2, 1, 2}, {1, 2, 1}};
    int res = sol.isPrintable(targetGrid);
    assert(res == false);
}
void test4() {
    vector<vector<int>> targetGrid ={
        {6,2,2,5},
        {2,2,2,5},
        {2,2,2,5},
        {4,3,3,4}
    };
    int res = sol.isPrintable(targetGrid);
    assert(res == true);
}
void test5() {
    vector<vector<int>> targetGrid = {
        {1,1,1,1,1,1,3,3,3},
        {1,1,1,1,1,1,3,3,3},
        {1,1,2,2,2,4,5,5,3},
        {1,1,2,2,2,2,5,5,2},
        {1,1,2,2,2,2,2,2,2},
        {1,1,1,1,1,6,6,6,6}
    };
    int res = sol.isPrintable(targetGrid);
    assert(res == true);
}
int main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}