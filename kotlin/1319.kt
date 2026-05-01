class Solution {
    fun makeConnected(n: Int, connections: Array<IntArray>): Int {
        if (connections.size < n - 1) return -1
        val parent = IntArray(n) { it }
        var components = n
        fun find(x: Int): Int {
            var root = x
            while (parent[root] != root) root = parent[root]
            var curr = x
            while (curr != root) {
                val next = parent[curr]
                parent[curr] = root
                curr = next
            }
            return root
        }
        for (conn in connections) {
            val rootA = find(conn[0])
            val rootB = find(conn[1])
            if (rootA != rootB) {
                parent[rootA] = rootB
                components--
            }
        }
        return components - 1
    }
}