class Solution {
    fun findShortestCycle(n: Int, edges: Array<IntArray>): Int {
        val adj = Array(n) { mutableListOf<Int>() }
        for ((u, v) in edges) {
            adj[u].add(v)
            adj[v].add(u)
        }

        var minCycle = Int.MAX_VALUE

        for (start in 0 until n) {
            val dist = IntArray(n) { -1 }
            val parent = IntArray(n) { -1 }
            val queue = ArrayDeque<Int>()

            dist[start] = 0
            queue.add(start)

            while (queue.isNotEmpty()) {
                val u = queue.removeFirst()

                if (dist[u] * 2 + 1 >= minCycle) break

                for (v in adj[u]) {
                    if (dist[v] == -1) {
                        dist[v] = dist[u] + 1
                        parent[v] = u
                        queue.add(v)
                    } else if (v != parent[u]) {
                        minCycle = minOf(minCycle, dist[u] + dist[v] + 1)
                    }
                }
            }
        }
        return if (minCycle == Int.MAX_VALUE) -1 else minCycle
    }
}