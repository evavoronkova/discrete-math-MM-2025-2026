def equationsPossible(equations: list[str]) -> bool:
    graph = {chr(i): [] for i in range(ord('a'), ord('z') + 1)}
    
    for eq in equations:
        if eq[1] == '=':
            u, v = eq[0], eq[3]
            graph[u].append(v)
            graph[v].append(u)
            
    color = {}
    
    def dfs(node, current_color):
        color[node] = current_color
        for neighbor in graph[node]:
            if neighbor not in color:
                dfs(neighbor, current_color)
                
    component_id = 0
    for node in graph:
        if node not in color:
            dfs(node, component_id)
            component_id += 1
            
    for eq in equations:
        if eq[1] == '!':
            u, v = eq[0], eq[3]
            if color[u] == color[v]:
                return False
                
    return True
