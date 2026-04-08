#include <cassert>
#include <cmath>
#include <cstring>
#include <format>
#include <vector>
#include <string>
#include <unordered_map>
using namespace std;

using position = pair<int, int>;

class Solution {
public:
    bool canMouseWin(vector<string>& grid, int catJump, int mouseJump) {
        this->mouseJump = mouseJump;
        this->catJump = catJump;
        this->grid = grid;
        position mousePos, catPos;
        emptyCells = 0;
        for (int i = 0; i < grid.size(); i++) {
            for (int j = 0; j < grid[0].size(); j++) {
                switch (grid[i][j]) {
                    case 'M': mousePos = position(i, j); break;
                    case 'C': catPos = position(i, j); break;
                    case 'F': foodPos = position(i, j); break;
                    case '.': ++emptyCells; break;
                }
            }
        }
        memset(contains, 0, sizeof(contains));
        memset(contains, 0, sizeof(cache));
        return dp(mousePos, catPos, 0);
    }
    bool isWall(position pos) {
        return grid[pos.first][pos.second] == '#';
    }
    bool isValidCell(position pos) {
        return (0 <= pos.first && pos.first < grid.size()) && (0 <= pos.second && pos.second < grid[0].size());
    }
    int contains[450000];
    int cache[450000];
    bool dp(position mousePos, position catPos, int turn) {
        if (turn > 2 * emptyCells + 1) return false;

        if (mousePos == foodPos) return true;
        if (catPos == foodPos || catPos == mousePos) return false;

        const auto state = mousePos.first + mousePos.second * 8
                            + catPos.first * 64 + catPos.second * 512 + turn * 4096;

        if (contains[state]) return cache[state];
        if (turn % 2 == 0) {
            for (auto dir : dirs) {
                int mx = 0;
                for (int i = 1; i <= mouseJump; i++) {
                    position newPos = position(mousePos.first + dir.first * i, mousePos.second + dir.second * i);
                    if (!isValidCell(newPos) || isWall(newPos)) break;
                    mx = i;
                }
                for (int i = mx; i >= 0; i--) {
                    position newMousePos = position(mousePos.first + dir.first * i, mousePos.second + dir.second * i);
                    if (!isValidCell(newMousePos) || isWall(newMousePos)) break;

                    if (dp(newMousePos, catPos, turn + 1)) {
                        contains[state] = true;
                        cache[state] = true;
                        return true;
                    }
                }
                if (dp(mousePos, catPos, turn + 1)) {
                    cache[state] = true;
                    return true;
                }
            }
            contains[state] = true;
            cache[state] = false;
            return false;
        }
        else {
            for (auto dir : dirs) {
                int mx = 0;
                for (int i = 1; i <= catJump; i++) {
                    position newPos = position(catPos.first + dir.first * i, catPos.second + dir.second * i);
                    if (!isValidCell(newPos) || isWall(newPos)) break;
                    mx = i;
                }

                for (int i = mx; i >= 1; i--) {
                    position newCatPos = position(catPos.first + dir.first * i, catPos.second + dir.second * i);
                    if (!isValidCell(newCatPos) || isWall(newCatPos)) break;

                    if (!dp(mousePos, newCatPos, turn + 1)) {
                        contains[state] = true;
                        cache[state] = false;
                        return false;
                    }
                }
                if (!dp(mousePos, catPos, turn + 1)) {
                    contains[state] = true;
                    cache[state] = false;
                    return false;
                }
            }
            contains[state] = true;
            cache[state] = true;
            return true;
        }
    }
    vector<string> grid;
    int catJump;
    int mouseJump;
    int emptyCells;
    position foodPos;

    const vector<position> dirs = {{0, -1}, {0, 1}, {-1, 0}, {1, 0}};
};
auto sol = Solution();
void test1() {
    vector<string> grid = {
        "####F",
        "#C...",
        "M....",
    };
    int catJump = 1;
    int mouseJump = 2;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
void test2() {
    vector<string> grid = {
        "M.C...F"
    };
    int catJump = 1;
    int mouseJump = 4;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
void test3() {
    vector<string> grid = {
        "M.C...F"
    };
    int catJump = 1;
    int mouseJump = 3;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == false);
}
void test4() {
    vector<string> grid = {
        "F...###",
        ".##.###",
        ".##.###",
        "...C...",
        "###.##.",
        "###.##.",
        "###...M",
    };
    int catJump = 1;
    int mouseJump = 2;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == false);
}
void test5() {
    vector<string> grid = {
        "F...###",
        ".##.###",
        ".##.###",
        "...C...",
        "###.##.",
        "###.##.",
        "###...M",
    };
    int catJump = 1;
    int mouseJump = 6;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
void test6() {
    vector<string> grid = {
        "C...#",
        "...#F",
        "....#",
        "M...."
    };
    int catJump = 2;
    int mouseJump = 5;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == false);
}
void test7() {
    vector<string> grid = {
        "...C.",
        "#####",
        "M...F"
    };
    int catJump = 2;
    int mouseJump = 3;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
void test8() {
    vector<string> grid = {
        "C#......",
        "M..####.",
        "###.....",
        "....####",
        ".####...",
        "......#.",
        "#######.",
        "F......."
    };
    int catJump = 1;
    int mouseJump = 1;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
void test9() {
    vector<string> grid = {
        "FC#..",
        ".....",
        "M#..#"
    };
    int catJump = 1;
    int mouseJump = 1;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == false);
}
void test10() {
    vector<string> grid = {
        "#F..",
        "..#.",
        "..M.",
        "..C."
    };
    int catJump = 3;
    int mouseJump = 3;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
void test11() {
    vector<string> grid = {
        "#....",
        "#.#C#",
        "M#.##",
        "F#..."
    };
    int catJump = 2;
    int mouseJump = 3;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
void test12() {
    vector<string> grid = {
        "........",
        "F...#C.M",
        "........"
    };
    int catJump = 1;
    int mouseJump = 2;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
void test13() {
    vector<string> grid = {
        "CM......",
        "#######.",
        "........",
        ".#######",
        "........",
        "#######.",
        "F.#...#.",
        "#...#..."
    };
    int catJump = 1;
    int mouseJump = 1;
    bool res = sol.canMouseWin(grid, catJump, mouseJump);
    assert(res == true);
}
int main() {
    auto tests = {
        test1, test2, test3,
        test4, test5, test6,
        test7, test8, test9,
        test10, test11, test12,
        test13
    };
    int i = 0;
    for (auto test : tests) {
        clock_t start_tick = clock();
        test();
        clock_t end_tick = clock();
        double diff = (double)(end_tick - start_tick) / CLOCKS_PER_SEC;
        printf("Тест %d, время: %.1f мс\n", ++i, diff * 1000);

    }
}