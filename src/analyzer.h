#ifndef ANALYZERS_H
#define ANALYZERS_H

#include <vector>

#include "general.h"
#include "graph.h"

using namespace std;

class graph_analyzer{
public:
    graph& g;
    graph_analyzer(graph& graph) : g(graph) {}

    double get_density() const;

    set<set<int>> get_connected_components();
    set<set<int>> get_strongly_connected_components();

    size_t get_amount_of_connected_components();
    double get_fraction_of_vertexes_in_max_connected_component();

    size_t get_amount_of_strongly_connected_components();
    double get_fraction_of_vertexes_in_max_strongly_connected_component();
private:
    // For searching connected components
    unordered_map<int, int> CC_comp_id;
    graph rg; // reversed graph

    void CC_undirected_dfs(int v);
    void CC_directed_dfs(int v);

    // For searching strongly connected components
    unordered_map<int, bool> CSC_visited;
    vector<int> CSC_order;
    set<int> CSC_component;
    void CSC_dfs1(int v);
    void CSC_dfs2(int v);

};

#endif // ANALYZERS_H
