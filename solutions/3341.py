import heapq

class Solution:
    def minTimeToReach(self, moveTime):
        n, m = len(moveTime), len(moveTime[0])
        dist = [[float('inf')] * m for _ in range(n)]
        dist[0][0] = 0
        
        q = [(0, 0, 0)]
        
        while q:
            t, r, c = heapq.heappop(q)
            
            if r == n - 1 and c == m - 1:
                return t
                
            if t > dist[r][c]:
                continue
                
            for dr, dc in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                nr, nc = r + dr, c + dc
                
                if 0 <= nr < n and 0 <= nc < m:
                    nt = max(t, moveTime[nr][nc]) + 1
                    
                    if nt < dist[nr][nc]:
                        dist[nr][nc] = nt
                        heapq.heappush(q, (nt, nr, nc))
                        
        return 0