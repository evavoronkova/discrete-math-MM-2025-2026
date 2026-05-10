from collections import deque

class Solution:
    def validateBinaryTreeNodes(self, n: int, leftChild: list[int], rightChild: list[int]) -> bool:
        indegree = [0] * n
        
        for i in range(n):
            if leftChild[i] != -1:
                indegree[leftChild[i]] += 1
            if rightChild[i] != -1:
                indegree[rightChild[i]] += 1
        
        root = -1
        for i in range(n):
            if indegree[i] == 0:
                if root != -1:
                    return False
                root = i
        
        if root == -1:
            return False
        
        visited = [False] * n
        queue = deque([root])
        visited[root] = True
        count = 1
        
        while queue:
            node = queue.popleft()
            for child in [leftChild[node], rightChild[node]]:
                if child != -1:
                    if visited[child]:
                        return False
                    visited[child] = True
                    queue.append(child)
                    count += 1
        
        return count == n