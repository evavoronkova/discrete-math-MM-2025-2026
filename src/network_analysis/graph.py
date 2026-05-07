from dataclasses import dataclass, field
from typing import Dict, Iterable, List, Set, Tuple


@dataclass
class Graph:
    directed: bool = False
    out_adj: Dict[str, Set[str]] = field(default_factory=dict)
    in_adj: Dict[str, Set[str]] = field(default_factory=dict)

    def add_node(self, node: str) -> None:
        self.out_adj.setdefault(node, set())
        self.in_adj.setdefault(node, set())

    def add_edge(self, source: str, target: str) -> None:
        self.add_node(source)
        self.add_node(target)
        self.out_adj[source].add(target)
        self.in_adj[target].add(source)
        if not self.directed:
            self.out_adj[target].add(source)
            self.in_adj[source].add(target)

    @property
    def nodes(self) -> List[str]:
        return list(self.out_adj.keys())

    def node_count(self) -> int:
        return len(self.out_adj)

    def edge_count(self) -> int:
        total = sum(len(neighbors) for neighbors in self.out_adj.values())
        return total if self.directed else total // 2

    def neighbors(self, node: str, weak: bool = True) -> Set[str]:
        if not self.directed or weak:
            return self.out_adj[node] | self.in_adj[node]
        return set(self.out_adj[node])

    def undirected_neighbors(self, node: str) -> Set[str]:
        return self.out_adj[node] | self.in_adj[node]

    def degree(self, node: str) -> int:
        return len(self.undirected_neighbors(node))

    def induced_subgraph(self, nodes: Iterable[str]) -> "Graph":
        node_set = set(nodes)
        subgraph = Graph(directed=self.directed)
        for node in node_set:
            subgraph.add_node(node)
        for source in node_set:
            for target in self.out_adj[source]:
                if target in node_set:
                    subgraph.add_edge(source, target)
        return subgraph

    def remove_nodes(self, nodes_to_remove: Iterable[str]) -> "Graph":
        removed = set(nodes_to_remove)
        kept = [node for node in self.nodes if node not in removed]
        return self.induced_subgraph(kept)

    def reversed(self) -> "Graph":
        reversed_graph = Graph(directed=self.directed)
        for node in self.nodes:
            reversed_graph.add_node(node)
        for source in self.nodes:
            for target in self.out_adj[source]:
                reversed_graph.add_edge(target, source)
        return reversed_graph

    def edge_list(self) -> List[Tuple[str, str]]:
        edges: List[Tuple[str, str]] = []
        for source in self.nodes:
            for target in self.out_adj[source]:
                if self.directed or source <= target:
                    edges.append((source, target))
        return edges
