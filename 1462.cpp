#include <cassert>
#include <vector>
using namespace std;

#define N 100
bool ps[N][N] = {false};
class Solution {
public:
    vector<bool> checkIfPrerequisite(int numCourses, vector<vector<int>>& prerequisites, vector<vector<int>>& queries) {
        for (int i = 0; i < numCourses; i++)
            for (int j = 0; j < numCourses; j++)
                ps[i][j] = false;

        for (auto p: prerequisites)
            ps[p[0]][p[1]] = true;

        for (int k = 0; k < numCourses; k++) {
            for (int i = 0; i < numCourses; i++) {
                for (int j = 0; j < numCourses; j++) {
                    ps[i][j] = ps[i][j] || (ps[i][k] && ps[k][j]);
                }
            }
        }
        vector<bool> solution;
        for (auto query : queries) {
            solution.push_back(ps[query[0]][query[1]]);
        }
        return solution;
    }
};
auto sol = Solution();
void test1() {
    vector<vector<int>> prerequisites = {{1, 0}};
    vector<vector<int>> queries = {{0, 1}, {1, 0}};
    vector<bool> res = sol.checkIfPrerequisite(2, prerequisites, queries);
    vector<bool> solution = {false, true};
    assert(res == solution);
}
void test2() {
    vector<vector<int>> prerequisites = {};
    vector<vector<int>> queries = {{1, 0}, {0, 1}};
    vector<bool> res = sol.checkIfPrerequisite(2, prerequisites, queries);
    vector<bool> solution = {false, false};
    assert(res == solution);
}
int main() {
    test1();
    test2();
}