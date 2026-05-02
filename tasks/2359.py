from typing import List

class Solution:
    def closestMeetingNode(self, edges: List[int], node1: int, node2: int) -> int:
        n = len(edges)

        dist1 = self.get_distances(edges, node1)
        dist2 = self.get_distances(edges, node2)

        answer = -1
        best_distance = 10 ** 9

        for i in range(n):
            if dist1[i] != -1 and dist2[i] != -1:
                current_distance = max(dist1[i], dist2[i])

                if current_distance < best_distance:
                    best_distance = current_distance
                    answer = i

        return answer

    def get_distances(self, edges: List[int], start: int) -> List[int]:
        n = len(edges)
        distances = [-1] * n

        current = start
        distance = 0

        while current != -1 and distances[current] == -1:
            distances[current] = distance
            distance += 1
            current = edges[current]

        return distances