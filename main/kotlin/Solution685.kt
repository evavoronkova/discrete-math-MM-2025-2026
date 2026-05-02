class Solution {
    fun findRedundantDirectedConnection(edges: Array<IntArray>): IntArray {
        val n = edges.size
        val parent = IntArray(n + 1)
        var cand1: IntArray? = null
        var cand2: IntArray? = null
        for (edge in edges) {
            val u = edge[0]
            val v = edge[1]
            if (parent[v] == 0) {
                parent[v] = u
            } else {
                cand1 = intArrayOf(parent[v], v)
                cand2 = intArrayOf(u, v)
                edge[1] = 0
                break
            }
        }
        for (i in 1..n) {
            parent[i] = i
        }
        for (edge in edges) {
            if (edge[1] == 0) continue
            val u = edge[0]
            val v = edge[1]
            if (find(parent, u) == v) {
                if (cand1 == null) {
                    return edge
                } else {
                    return cand1
                }
            }
            union(parent, u, v)
        }
        return cand2 ?: intArrayOf()
    }

    fun find(parent: IntArray, x: Int): Int {
        if (parent[x] != x) {
            parent[x] = find(parent, parent[x])
        }
        return parent[x]
    }

    fun union(parent: IntArray, x: Int, y: Int) {
        val rootX = find(parent, x)
        val rootY = find(parent, y)
        if (rootX != rootY) {
            parent[rootY] = rootX
        }
    }
}