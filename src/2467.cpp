#include <cassert>
#include <climits>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
public:
    int mostProfitablePath(vector<vector<int>>& edges, int bob, vector<int>& amount) {
        int n = amount.size();
        
        graph = vector<vector<int>>(n, vector<int>());
        for (auto & edge : edges) {
            graph[edge[0]].push_back(edge[1]);
            graph[edge[1]].push_back(edge[0]);
        }
        prices = &amount;

        visited = vector<bool>(n, false);
        visited[bob] = true;

        bobPathMap[bob] = 0;
        dfsBob(bob, 1);

        visited = vector<bool>(n, false);
        visited[0] = true;
        int alice = dfsAlice(0, (*prices)[0], 1);
        return alice;
    }
    int dfsAlice(int v, int sum, int step) {
        int maxAdd = INT_MIN;
        for (int newV : graph[v]) {
            if (visited[newV]) continue;
            visited[newV] = true;

            int addSum;
            if (bobPathMap.contains(newV)) {
                int bobStep = bobPathMap[newV];
                if (bobStep == step) addSum = (*prices)[newV] / 2;
                else if (bobStep < step) addSum = 0;
                else addSum = (*prices)[newV];
            }
            else addSum = (*prices)[newV];
            maxAdd = max(maxAdd, dfsAlice(newV, addSum, step + 1));
        }
        return (maxAdd == INT_MIN) ? sum : sum + maxAdd;
    }
    void dfsBob(int v, int step) {
        if (v == 0) {
            bobFoundPath = true;
            return;
        }
        for (int newV : graph[v]) {
            if (visited[newV]) continue;
            visited[newV] = true;

            bobPathMap[newV] = step;
            dfsBob(newV, step + 1);

            if (bobFoundPath) return;
            bobPathMap.erase(newV);
        }
    }
private:
    vector<vector<int>> graph;
    unordered_map<int, int> bobPathMap;
    vector<int> *prices;
    vector<bool> visited;

    bool bobFoundPath = false;
};
auto sol = Solution();
void test1() {
    vector<vector<int>> edges = {{0, 1}, {1, 2}, {1, 3}, {3, 4}};
    vector<int> amount = {-2, 4, 2, -4, 6};
    int res = sol.mostProfitablePath(edges, 3, amount);
    assert(res == 6);
}
void test2() {
    vector<vector<int>> edges = {{0, 1}};
    vector<int> amount = {-7280, 2350};
    int res = sol.mostProfitablePath(edges, 1, amount);
    assert(res == -7280);
}
void test3() {
    vector<vector<int>> edges = {{0, 1}, {1, 2}, {2, 3}};
    vector<int> amount = {70, -120, -500, -1000};
    int res = sol.mostProfitablePath(edges, 3, amount);
    assert(res == -50);
}

int main() {
    test1();
    test2();
    test3();
}