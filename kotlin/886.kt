class Solution {
    fun possibleBipartition(n: Int, dislikes: Array<IntArray>): Boolean {
        val adj = Array(n + 1) { mutableListOf<Int>() }
        for (edge in dislikes) {
            adj[edge[0]].add(edge[1])
            adj[edge[1]].add(edge[0])
        }

        val colors = IntArray(n + 1)
        val queue = ArrayDeque<Int>()

        for (i in 1..n) {
            if (colors[i] != 0) continue

            colors[i] = 1
            queue.add(i)

            while (queue.isNotEmpty()) {
                val curr = queue.removeFirst()
                val nextColor = -colors[curr]
                for (neighbor in adj[curr]) {
                    if (colors[neighbor] == 0) {
                        colors[neighbor] = nextColor
                        queue.add(neighbor)
                    } else if (colors[neighbor] == colors[curr]) {
                        return false
                    }
                }
            }
        }
        return true
    }
}