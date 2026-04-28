import kotlin.math.abs

class Solution {
    fun minCostConnectPoints(points: Array<IntArray>): Int {
        val n = points.size
        if (n <= 1) return 0
        val minDist = IntArray(n) { Int.MAX_VALUE }
        val visited = BooleanArray(n)
        minDist[0] = 0
        var totalCost = 0
        for (i in 0..<n) {
            var u = -1
            for (j in 0..<n) {
                if (!visited[j] && (u == -1 || minDist[j] < minDist[u])) {
                    u = j
                }
            }
            visited[u] = true
            totalCost += minDist[u]

            for (v in 0..<n) {
                if (!visited[v]) {
                    val dist = abs(points[u][0] - points[v][0]) + abs(points[u][1] - points[v][1])
                    if (dist < minDist[v]) {
                        minDist[v] = dist
                    }
                }
            }
        }
        return totalCost
    }
}