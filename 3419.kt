class Solution {
    fun minMaxWeight(n: Int, edges: Array<IntArray>, threshold: Int): Int {
        val reverseAdjacency = Array(n) { ArrayList<IntArray>(4) }
        for (edge in edges) {
            val (a, b, w) = edge
            reverseAdjacency[b].add(intArrayOf(a, w))
        }
        for (i in 0 until n) {
            reverseAdjacency[i].sortBy { it[1] }
        }

        val visited = BooleanArray(n)
        val queue = IntArray(n)
        val outUsed = IntArray(n)

        fun canAchieve(maxWeight: Int): Boolean {
            visited.fill(false)
            outUsed.fill(0)
            var head = 0
            var tail = 0
            queue[tail++] = 0
            visited[0] = true
            var visitedCount = 1

            while (head < tail) {
                val currentVertex = queue[head++]
                for (next in reverseAdjacency[currentVertex]) {
                    val neighbor = next[0]
                    val w = next[1]
                    if (w > maxWeight) {
                        break
                    }
                    if (outUsed[neighbor] >= threshold) {
                        continue
                    }
                    outUsed[neighbor]++
                    if (!visited[neighbor]) {
                        visited[neighbor] = true
                        queue[tail++] = neighbor
                        visitedCount++
                    }
                }
            }
            return visitedCount == n
        }

        val uniqueSortedWeights = IntArray(edges.size) { edges[it][2] }.also { it.sort() }
        if (!canAchieve(uniqueSortedWeights.last())) {
            return -1
        }
        var lowerBound = 0
        var upperBound = uniqueSortedWeights.size - 1
        while (lowerBound < upperBound) {
            val middleIndex = (lowerBound + upperBound) / 2
            if (canAchieve(uniqueSortedWeights[middleIndex])) {
                upperBound = middleIndex
            }
            else {
                lowerBound = middleIndex + 1
            }
        }
        return uniqueSortedWeights[lowerBound]
    }
}
