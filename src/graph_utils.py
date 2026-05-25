from collections import defaultdict, deque


def load_graph(filepath, delimiter=None, is_directed=False):
    """
    Читает граф из файла. Поддерживает неориентированные и ориентированные графы.
    """
    graph = defaultdict(set)
    # Для орграфов нам нужен также обратный граф, чтобы искать компоненты сильной связности
    rev_graph = defaultdict(set)

    with open(filepath, 'r') as f:
        for line in f:
            if line.startswith('#') or line.startswith('%') or not line.strip():
                continue
            parts = line.strip().split(delimiter)
            if len(parts) >= 2:
                try:
                    u, v = int(parts[0]), int(parts[1])
                    if u != v:  # Игнорируем петли
                        graph[u].add(v)
                        if is_directed:
                            rev_graph[v].add(u)
                        else:
                            graph[v].add(u)
                except ValueError:
                    continue

    if is_directed:
        return graph, rev_graph
    return graph, None


def bfs_distances(graph, start):
    """
    Обычный BFS. Возвращает словарь расстояний от start до всех достижимых вершин.
    """
    distances = {start: 0}
    queue = deque([start])
    while queue:
        u = queue.popleft()
        for v in graph.get(u, set()):
            if v not in distances:
                distances[v] = distances[u] + 1
                queue.append(v)
    return distances


def get_weakly_connected_components(graph):
    """
    Находит компоненты слабой связности (для неориентированного графа это просто компоненты связности).
    Возвращает список множеств (компонент).
    """
    visited = set()
    components = []
    for node in list(graph.keys()):
        if node not in visited:
            comp_nodes = bfs_distances(graph, node).keys()
            component = set(comp_nodes)
            components.append(component)
            visited.update(component)
    return components


def get_strongly_connected_components(graph, rev_graph):
    """
    Алгоритм Косарайю для поиска компонент сильной связности (SCC).
    """
    visited = set()
    order = []

    def dfs1(u):
        visited.add(u)
        for v in graph.get(u, set()):
            if v not in visited:
                dfs1(v)
        order.append(u)

    for node in graph:
        if node not in visited:
            dfs1(node)

    visited.clear()
    components = []

    def dfs2(u, component):
        visited.add(u)
        component.add(u)
        for v in rev_graph.get(u, set()):
            if v not in visited:
                dfs2(v, component)

    for node in reversed(order):
        if node not in visited:
            comp = set()
            dfs2(node, comp)
            components.append(comp)

    return components
