#include <cassert>
#include <queue>
#include <vector>
using namespace std;
class Solution {
public:
    vector<int> shortestDistanceAfterQueries(int n, vector<vector<int>>& queries) {
        vector<vector<int>> graph = vector<vector<int>>(500);
        vector<int> minPath = vector<int>(500, 1000);
        minPath[0] = 0;
        for (int i = 0; i < graph.size(); ++i) {
            graph[i].push_back(i + 1);
        }
        vector<int> solution;
        for (auto query : queries) {
            graph[query[0]].push_back(query[1]);
            if (minPath[query[0]] + 1 > minPath[query[1]] && minPath[query[0]] != 1000) {
                solution.push_back(solution[solution.size() - 1]);
                continue;
            }
            bool isExit = false;
            queue<int> q;
            vector<bool> visited = vector(500, false);
            q.push(0);
            while (!q.empty()) {
                auto node = q.front();
                q.pop();
                for (int i : graph[node]) {
                    if (visited[i])
                        continue;
                    visited[i] = true;
                    auto newNode = i;
                    minPath[i] = min(minPath[i], minPath[node] + 1);
                    if (i == n - 1) {
                        solution.push_back(minPath[node] + 1);
                        isExit = true;
                        break;
                    }
                    q.push(i);
                }
                if (isExit) break;
            }
        }

        return solution;
    }
};
auto sol = Solution();
void test1() {
    vector<vector<int>> queries = {{2, 4}, {0, 2},{0, 4}};
    vector<int> res = sol.shortestDistanceAfterQueries(5, queries);
    vector<int> solution = {3, 2, 1};
    assert(res == solution);
}
void test2() {
    vector<vector<int>> queries = {{5, 7}, {0, 6}};
    vector<int> res = sol.shortestDistanceAfterQueries(8, queries);
    vector<int> solution = {6, 2};
    assert(res == solution);
}
int main() {
    test1();
    test2();
}