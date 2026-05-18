class Solution:
    def minMalwareSpread(self, graph: list[list[int]], initial: list[int]) -> int:
        n = len(graph)
        initial_set = set(initial)
        
        visited = [False] * n
        comp_id = [-1] * n
        comp_sizes = []
        
        from collections import deque
        
        comp_idx = 0
        for i in range(n):
            if i not in initial_set and not visited[i]:
                q = deque([i])
                visited[i] = True
                size = 0
                while q:
                    u = q.popleft()
                    comp_id[u] = comp_idx
                    size += 1
                    for v in range(n):
                        if graph[u][v] == 1 and v not in initial_set and not visited[v]:
                            visited[v] = True
                            q.append(v)
                comp_sizes.append(size)
                comp_idx += 1
        
        infect_count = [0] * comp_idx
        infect_sources = [set() for _ in range(comp_idx)]
        
        for infected in initial:
            infected_comps = set()
            for v in range(n):
                if v not in initial_set and graph[infected][v] == 1:
                    cid = comp_id[v]
                    if cid != -1:
                        infected_comps.add(cid)
            for cid in infected_comps:
                infect_count[cid] += 1
                infect_sources[cid].add(infected)
        
        saved = [0] * n
        for cid in range(comp_idx):
            if infect_count[cid] == 1:
                infected_node = next(iter(infect_sources[cid]))
                saved[infected_node] += comp_sizes[cid]
        
        initial.sort()
        best_node = initial[0]
        for node in initial:
            if saved[node] > saved[best_node]:
                best_node = node
        
        return best_node