#include <cassert>
#include <vector>
#include <queue>
using namespace std;

class Solution {
public:
    vector<int> eventualSafeNodes(vector<vector<int>>& graph) {
        vector<vector<short>> reversedGraph = vector<vector<short>>(graph.size());
        vector<short> edgesAmount;
        queue<short> q;

        for (short i = 0; i < graph.size(); ++i) {
            if (graph[i].empty())
                q.push(i);
            edgesAmount.push_back(graph[i].size());
            for (int j : graph[i]) {
                reversedGraph[j].push_back(i);
            }
        }
        while (!q.empty()) {
            short finalNode = q.front();
            q.pop();
            for (short otherNode : reversedGraph[finalNode]) {
                --edgesAmount[otherNode];
                if (edgesAmount[otherNode] == 0) {
                    q.push(otherNode);
                }
            }
        }
        vector<int> lastNodes;
        for (int i = 0; i < edgesAmount.size(); ++i) {
            if (edgesAmount[i] == 0)
                lastNodes.push_back(i);
        }
        return lastNodes;
    }
};
auto sol = Solution();
void test1() {
    vector<vector<int>> graph = {{1,2}, {2,3}, {5}, {0}, {5}, {}, {}};

    auto res = sol.eventualSafeNodes(graph);
    vector<int> solution = {2, 4, 5, 6};
    assert(res == solution);
}
void test2() {
    vector<vector<int>> graph = {{1,2,3,4}, {1,2}, {3, 4}, {0, 4}, {}};

    auto res = sol.eventualSafeNodes(graph);
    vector<int> solution = {4};
    assert(res == solution);
}
void test3() {
    vector<vector<int>> graph = {{}, {0,2,3,4}, {3}, {4}, {}};

    auto res = sol.eventualSafeNodes(graph);
    vector<int> solution = {0, 1, 2, 3, 4};
    assert(res == solution);
}

int main() {
    test1();
    test2();
    test3();
    return 0;
}
