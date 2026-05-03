import kotlin.math.min

class Solution {
    fun findShortestCycle(n: Int, edges: Array<IntArray>): Int {
        val graph = Array(n) { mutableListOf<Int>() }
        for (edge in edges) {
            val u = edge[0]
            val v = edge[1]
            graph[u].add(v)
            graph[v].add(u)
        }
        var shortestCycleLength = Int.MAX_VALUE
        for (start in 0..<n) {
            val dist = IntArray(n) { -1 }
            val parent = IntArray(n) { -1 }
            val queue = java.util.LinkedList<Int>()
            dist[start] = 0
            queue.offer(start)
            while (!queue.isEmpty()) {
                val curr = queue.poll()
                for (neighbor in graph[curr]) {
                    if (dist[neighbor] == -1) {
                        dist[neighbor] = dist[curr] + 1
                        parent[neighbor] = curr
                        queue.offer(neighbor)
                    } else if (neighbor != parent[curr]) {
                        val cycleLength = dist[curr] + dist[neighbor] + 1
                        shortestCycleLength = min(shortestCycleLength, cycleLength)
                    }
                }
            }
        }
        return if (shortestCycleLength == Int.MAX_VALUE) -1 else shortestCycleLength
    }
}