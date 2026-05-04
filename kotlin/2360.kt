class Solution {
    fun longestCycle(edges: IntArray): Int {
        val n = edges.size
        val visited = IntArray(n) { -1 }
        var time = 0
        var maxCycle = -1

        for (i in 0 until n) {
            if (visited[i] != -1) continue
            var curr = i
            val startTime = time
            while (curr != -1 && visited[curr] == -1) {
                visited[curr] = time++
                curr = edges[curr]
            }
            if (curr != -1 && visited[curr] >= startTime) {
                val cycleLen = time - visited[curr]
                if (cycleLen > maxCycle) maxCycle = cycleLen
            }
        }
        return maxCycle
    }
}