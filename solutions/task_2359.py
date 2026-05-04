class Solution:
    def closestMeetingNode(self, edges: list[int], node1: int, node2: int) -> int:
        n = len(edges)
        
        def get_dist(start):
            dist = [-1] * n
            dist[start] = 0
            cur = start
            while edges[cur] != -1 and dist[edges[cur]] == -1:
                dist[edges[cur]] = dist[cur] + 1
                cur = edges[cur]
            return dist
        
        d1 = get_dist(node1)
        d2 = get_dist(node2)
        
        ans = -1
        min_max_dist = float('inf')
        
        for i in range(n):
            if d1[i] != -1 and d2[i] != -1:
                cur_max = max(d1[i], d2[i])
                if cur_max < min_max_dist or (cur_max == min_max_dist and i < ans):
                    min_max_dist = cur_max
                    ans = i
        return ans