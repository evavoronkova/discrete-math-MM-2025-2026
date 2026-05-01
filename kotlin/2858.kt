class Solution {
    fun minEdgeReversals(n: Int, edges: Array<IntArray>): IntArray {
        val adj = Array(n) { _ -> mutableListOf<Pair<Int, Int>>() }
        for (edge in edges) {
            val u = edge[0]
            val v = edge[1]
            adj[u].add(Pair(v, 0))
            adj[v].add(Pair(u, 1))
        }

        val answer = IntArray(n)
        var rootCost = 0
        val stack1 = ArrayDeque<Pair<Int, Int>>()
        stack1.addLast(Pair(0, -1))

        while (stack1.isNotEmpty()) {
            val pair = stack1.removeLast()
            val u = pair.first
            val p = pair.second

            for (neighbor in adj[u]) {
                val v = neighbor.first
                val w = neighbor.second
                if (v != p) {
                    rootCost += w
                    stack1.addLast(Pair(v, u))
                }
            }
        }
        answer[0] = rootCost

        val stack2 = ArrayDeque<Pair<Int, Int>>()
        stack2.addLast(Pair(0, -1))

        while (stack2.isNotEmpty()) {
            val pair = stack2.removeLast()
            val u = pair.first
            val p = pair.second

            for (neighbor in adj[u]) {
                val v = neighbor.first
                val w = neighbor.second
                if (v != p) {
                    answer[v] = answer[u] + if (w == 0) 1 else -1
                    stack2.addLast(Pair(v, u))
                }
            }
        }
        return answer
    }
}