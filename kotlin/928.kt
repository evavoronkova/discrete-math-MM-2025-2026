class Solution {
    fun minMalwareSpread(graph: Array<IntArray>, initial: IntArray): Int {
        val n = graph.size
        val sortedInitial = initial.sorted()
        var minInfected = n + 1
        var resultNode = sortedInitial[0]

        for (removeNode in sortedInitial) {
            var infectedCount = 0
            val visited = BooleanArray(n)
            val queue = IntArray(n)
            var head = 0
            var tail = 0

            for (node in initial) {
                if (node == removeNode) continue
                if (!visited[node]) {
                    visited[node] = true
                    queue[tail++] = node
                    infectedCount++
                }
            }
            while (head < tail) {
                val u = queue[head++]
                for (v in 0 until n) {
                    if (graph[u][v] == 1 && v != removeNode && !visited[v]) {
                        visited[v] = true
                        queue[tail++] = v
                        infectedCount++
                    }
                }
            }
            if (infectedCount < minInfected) {
                minInfected = infectedCount
                resultNode = removeNode
            }
        }
        return resultNode
    }
}