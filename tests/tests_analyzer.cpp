#include "tests_analyzer.h"
#include "../src/analyzer.h"
#include "../src/general.h"
#include "../src/graph.h"

namespace analyzer_tests {
    void tests() {
        test_CSC_and_fraction_equality();
        test_CC_and_fraction_equality();
    }
    void test_CSC_and_fraction_equality(){
        graph g;
        g.type = Directed;
        vector<pair<int, int>> edges = {
            {1, 2}, {2, 3}, {3, 1},
            {3, 4}, {5, 4}, {5, 6},
            {6, 5}, {7, 4}, {7, 8},
            {8, 9}, {9, 10}, {10, 9}
        };
        for (auto p : edges)
            g.insert(p.first, p.second);

        graph_analyzer analyzer(g);
        auto output = analyzer.get_strongly_connected_components();
        auto fraction = analyzer.get_fraction_of_vertexes_in_max_strongly_connected_component();
        set<set<int>> solution = {{1, 2, 3}, {5, 6}, {9, 10}, {4}, {7}, {8}};
        assert(output == solution);
        assert(fraction == 0.3);
    }
    void test_CC_and_fraction_equality() {
        vector<pair<int, int>> edges = {
            {1, 2}, {2, 3},{3, 4},{1, 3},
            {2, 4}, {5, 6},{8, 7}
        };
        auto types = { Directed, Undirected };
        for (auto type : types) {
            graph g;
            g.type = type;
            for (auto p : edges)
                g.insert(p.first, p.second);

            graph_analyzer analyzer(g);
            auto output = analyzer.get_connected_components();
            auto fraction = analyzer.get_fraction_of_vertexes_in_max_connected_component();
            set<set<int>> solution = {{1, 2, 3, 4}, {5, 6}, {7, 8}};
            assert(output == solution);
            assert(fraction == 0.5);
        }

    }

}