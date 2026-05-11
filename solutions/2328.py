def countPaths(grid):
    MOD = (10**9) + 7
    row = len(grid)
    col = len(grid[0])
    data = [[-1] * col for _ in range(row)]

    def dfs(i, j):
        if data[i][j] != -1:
            return data[i][j]
        
        paths = 1
        directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

        for dx, dy in directions:
            ni = i + dx
            nj = j + dy

            if 0 <= ni < row and 0 <= nj < col and grid[ni][nj] > grid[i][j]:
                paths = (paths + dfs(ni, nj)) % MOD
            
        data[i][j] = paths
        return paths
    
    
    total_paths = 0
    for i in range(row):
        for j in range(col):
            total_paths = (total_paths + dfs(i, j)) % MOD

    return total_paths


