class Solution {
    fun findTheCity(n: Int, edges: Array<IntArray>, distanceThreshold: Int): Int {
        val INF = 1_000_000_000
        val dist = Array(n) { IntArray(n) { INF } }

        for (i in 0 until n) dist[i][i] = 0

        for ((u, v, w) in edges) {
            dist[u][v] = w
            dist[v][u] = w
        }

        for (k in 0 until n) {
            for (i in 0 until n) {
                if (dist[i][k] == INF) continue
                for (j in 0 until n) {
                    if (dist[k][j] != INF && dist[i][k] + dist[k][j] < dist[i][j]) {
                        dist[i][j] = dist[i][k] + dist[k][j]
                    }
                }
            }
        }

        var minReachable = n
        var resultCity = -1

        for (i in 0 until n) {
            var count = 0
            for (j in 0 until n) {
                if (i != j && dist[i][j] <= distanceThreshold) {
                    count++
                }
            }
            if (count <= minReachable) {
                minReachable = count
                resultCity = i
            }
        }

        return resultCity
    }
}