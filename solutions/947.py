from collections import defaultdict


def removeStones(stones):
    rows = defaultdict(list)
    cols = defaultdict(list)
    visited = set()
    groups = 0

    for i in range(len(stones)):
        r, c = stones[i]
        rows[r].append(i)
        cols[c].append(i)

    def dfs(i):
        visited.add(i)
        r, c = stones[i]

        for j in rows[r]:
            if j not in visited:
                dfs(j)

        for j in cols[c]:
            if j not in visited:
                dfs(j)
    
    for i in range(len(stones)):
        if i not in visited:
            dfs(i)
            groups += 1

    return len(stones) - groups
