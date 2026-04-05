#include <cassert>
#include <vector>
using namespace std;
class Solution {
public:
    vector<int> remainingMethods(int n, int k, vector<vector<int>>& invocations) {
        graph = vector<vector<int>>(n);
        isOuterFuncFound = false;

        for (auto & invocation : invocations)
            graph[invocation[0]].push_back(invocation[1]);
        firstVisited = vector<bool>(n, false);
        firstVisited[k] = true;
        dfs(k);
        secondVisited = vector<bool>(n, false);

        for (int i = 0; i < n; i++) {
            if (!firstVisited[i]) {
                secondVisited[i] = true;
                dfsChecker(i);
                if (isOuterFuncFound) break;
            }
        }
        vector<int> solution;
        if (isOuterFuncFound) {
            for (int i = 0; i < n; i++)
                solution.push_back(i);
        }
        else {
            for (int i = 0; i < n; i++) {
                if (!firstVisited[i])
                    solution.push_back(i);
            }
        }
        return solution;
    }
    void dfs(int v) {
        for (int nextV: graph[v]) {
            if (firstVisited[nextV]) continue;
            firstVisited[nextV] = true;
            dfs(nextV);
        }
    }
    void dfsChecker(int v) {
        for (int nextV: graph[v]) {
            if (secondVisited[nextV]) continue;
            secondVisited[nextV] = true;
            if (firstVisited[nextV]) {
                isOuterFuncFound = true;
                return;
            }
            dfsChecker(nextV);

            if (isOuterFuncFound)
                return;
        }
    }
private:
    vector<bool> firstVisited;
    vector<bool> secondVisited;
    vector<vector<int>> graph;
    bool isOuterFuncFound = false;
};
auto sol = Solution();
void test1() {
    vector<vector<int>> invocations = {{1, 2}, {0, 1}, {3, 2}};
    vector<int> res = sol.remainingMethods(4, 1, invocations);
    vector<int> solution = {0, 1, 2, 3};
    assert(res == solution);
}
void test2() {
    vector<vector<int>> invocations = {{1, 2}, {0, 1}, {0, 2}, {3, 4}};
    vector<int> res = sol.remainingMethods(5, 0, invocations);
    vector<int> solution = {3, 4};
    assert(res == solution);
}
void test3() {
    vector<vector<int>> invocations = {{1, 2}, {0, 1}, {2, 0}};
    vector<int> res = sol.remainingMethods(3, 2, invocations);
    vector<int> solution = {};
    assert(res == solution);
}

int main() {
    test1();
    test2();
    test3();
}