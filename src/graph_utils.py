from collections import defaultdict, deque


def load_graph(filepath, delimiter=None, is_directed=False):
    """
    Читает граф из файла. Возвращает граф, обратный граф и множество ВСЕХ узлов.
    """
    # АВТОМАТИЧЕСКОЕ ОПРЕДЕЛЕНИЕ РАЗДЕЛИТЕЛЯ
    if delimiter is None:
        if filepath.endswith('.csv'):
            delimiter = ','  # Для CSV используем запятую
        else:
            delimiter = None # Для .txt останется None (будет разбивать по пробелам/табам)

    graph = defaultdict(set)
    rev_graph = defaultdict(set)
    all_nodes = set()  

    with open(filepath, 'r') as f:
        for line in f:
            if line.startswith('#') or line.startswith('%') or not line.strip():
                continue
            # Если delimiter None, split() работает как split(None) - по пробелам
            # Если delimiter ',', split() разобьет по запятым
            parts = line.strip().split(delimiter)
            
            if len(parts) >= 2:
                try:
                    u, v = int(parts[0]), int(parts[1])
                    if u != v:
                        all_nodes.add(u)
                        all_nodes.add(v)

                        graph[u].add(v)
                        graph[v].add(u)
                        if is_directed:
                            rev_graph[v].add(u)  # Обратный граф нужен только для сильной связности
                except ValueError:
                    continue

    if is_directed:
        return graph, rev_graph, all_nodes
    return graph, None, all_nodes


def get_weakly_connected_components(graph, rev_graph=None, all_nodes=None):
    """
    Находит компоненты слабой связности.
    Для орграфа нужно смотреть и прямые, и обратные ребра!
    """
    visited = set()
    components = []

    # Для орграфа мы должны ходить по обеим направлениям
    nodes_to_iterate = all_nodes if all_nodes else list(graph.keys())

    for node in nodes_to_iterate:
        if node not in visited:
            # BFS
            queue = deque([node])
            visited.add(node)
            comp_nodes = set()

            while queue:
                u = queue.popleft()
                comp_nodes.add(u)
                # Идем по прямым ребрам
                for v in graph.get(u, set()):
                    if v not in visited:
                        visited.add(v)
                        queue.append(v)
                # Если граф ориентированный, идем и по обратным!
                if rev_graph:
                    for v in rev_graph.get(u, set()):
                        if v not in visited:
                            visited.add(v)
                            queue.append(v)

            components.append(comp_nodes)
    return components


def get_strongly_connected_components(graph, rev_graph, all_nodes):
    """
    Итеративный алгоритм Косарайю (БЕЗ рекурсии, чтобы не было RecursionError).
    """
    visited = set()
    order = []

    # Итеративный DFS 1 (прямой граф)
    for node in all_nodes:
        if node not in visited:
            stack = [(node, False)]  # (вершина, флаг "обработаны ли соседи")
            while stack:
                u, processed = stack.pop()
                if processed:
                    order.append(u)
                    continue
                if u in visited:
                    continue
                visited.add(u)
                # Кладем обратно с флагом True, чтобы добавить в order ПОСЛЕ соседей
                stack.append((u, True))
                for v in graph.get(u, set()):
                    if v not in visited:
                        stack.append((v, False))

    # Итеративный DFS 2 (обратный граф)
    visited.clear()
    components = []

    for node in reversed(order):
        if node not in visited:
            comp = set()
            stack = [node]
            visited.add(node)

            while stack:
                u = stack.pop()
                comp.add(u)
                for v in rev_graph.get(u, set()):
                    if v not in visited:
                        visited.add(v)
                        stack.append(v)

            components.append(comp)

    return components


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
