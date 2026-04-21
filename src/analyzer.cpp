#include "analyzer.h"

double graph_analyzer::get_density() const {
    size_t max_edges = (g.amount_vertexes / 2) * (g.amount_vertexes - 1);
    return (double)g.amount_edges / (double)max_edges;
}

size_t graph_analyzer::get_amount_of_connected_components() {
    return get_connected_components().size();
}
size_t graph_analyzer::get_amount_of_strongly_connected_components() {
    return get_strongly_connected_components().size();
}

double graph_analyzer::get_fraction_of_vertexes_in_max_connected_component() {
    auto components = get_connected_components();
    size_t mx = 0;
    for (auto &component : components) {
        mx = max(mx, component.size());
    }
    return (double)mx / (double)g.size();
}

double graph_analyzer::get_fraction_of_vertexes_in_max_strongly_connected_component() {
    auto components = get_strongly_connected_components();
    size_t mx = 0;
    for (auto &component : components) {
        mx = max(mx, component.size());
    }
    return (double)mx / (double)g.size();
}

// CC means Connected Components
void graph_analyzer::CC_undirected_dfs(int v) {
    for (auto o : g[v]) {
        if (CC_comp_id[o] != -1) continue;
        CC_comp_id[o] = CC_comp_id[v];
        CC_undirected_dfs(o);
    }
}
void graph_analyzer::CC_directed_dfs(int v) {
    CC_undirected_dfs(v);
    for (auto o : rg[v]) {
        if (CC_comp_id[o] != -1) continue;
        CC_comp_id[o] = CC_comp_id[v];
        CC_directed_dfs(o);
    }
}
set<set<int>> graph_analyzer::get_connected_components() {
    auto v_list = g.get_vertexes();

    g.calculate_vertexes();
    CC_comp_id.reserve(g.amount_vertexes);
    for (auto v : v_list) CC_comp_id[v] = -1;

    int c = 0;
    if (g.type == Undirected) {
        // For search CC in undirected graph just use DFS
        for (auto v : v_list) {
            if (CC_comp_id[v] != -1) continue;
            CC_comp_id[v] = c++;
            CC_undirected_dfs(v);
        }
    }
    else if (g.type == Directed) {
        // For search CC in dir-graph, firstly, init reversed graph and, secondly, use DFS on both graphs
        if (rg.empty()) rg = g.get_reversed_graph();

        for (auto v : v_list) {
            if (CC_comp_id[v] != -1) continue;
            CC_comp_id[v] = c++;
            CC_directed_dfs(v);
        }
    }

    auto components_list = vector<set<int>>(c);
    for (auto v : v_list) {
        components_list[CC_comp_id[v]].insert(v);
    }
    auto components_set = set<set<int>>();
    for (auto& comp : components_list) {
        components_set.insert(comp);
    }
    components_list.clear();
    CC_comp_id.clear();
    return components_set;
}

// This is implementation of Kosaraju's algorithm
// CSC means Connected Strong Components
void graph_analyzer::CSC_dfs1(int v) {
    CSC_visited[v] = true;
    for (auto o : g[v])
        if (!CSC_visited[o])
            CSC_dfs1(o);
    CSC_order.push_back(v);
}
void graph_analyzer::CSC_dfs2(int v) {
    CSC_visited[v] = true;
    CSC_component.insert(v);
    for (auto o : rg[v])
        if (!CSC_visited[o])
            CSC_dfs2(o);
}

set<set<int>> graph_analyzer::get_strongly_connected_components() {
    auto v_list = g.get_vertexes();
    if (rg.empty()) rg = g.get_reversed_graph();
    for (auto v : v_list) {
        if (!CSC_visited[v])
            CSC_dfs1(v);
    }
    CSC_visited.clear();
    auto components = set<set<int>>();
    size_t n = CSC_order.size();
    for (int i = 0; i < n; i++) {
        int v = CSC_order[n - i - 1];
        if (!CSC_visited[v]) {
            CSC_dfs2(v);
            components.insert(CSC_component);
            CSC_component.clear();
        }
    }
    CSC_visited.clear();
    CSC_order.clear();
    return components;
}
