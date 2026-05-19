class Solution {
    fun findRedundantConnection(edges: Array<IntArray>): IntArray {
        val n = edges.size
        val parent = IntArray(n + 1) { it }

        fun find(x: Int): Int {
            if (parent[x] != x) parent[x] = find(parent[x])
            return parent[x]
        }

        fun union(x: Int, y: Int): Boolean {
            val rootX = find(x)
            val rootY = find(y)

            if (rootX == rootY) return true
            parent[rootX] = rootY
            return false
        }

        for ((x, y) in edges) {
            if (union(x, y)) return intArrayOf(x, y)
        }

        return intArrayOf()
    }
}