class Solution {
    fun makeConnected(n: Int, connections: Array<IntArray>): Int {
        if (connections.size < n - 1) {
            return -1
        }

        val parent = IntArray(n)
        val rank = IntArray(n)
        for (i in 0..<n) {
            parent[i] = i
            rank[i] = 1
        }
        var components = n
        for (i in connections) {
            var rootA = find(i[0], parent)
            var rootB = find(i[1], parent)
            if (rootA != rootB) {
                if (rank[rootA] < rank[rootB]) {
                    val temp = rootA
                    rootA = rootB
                    rootB = temp
                }
                parent[rootB] = rootA
                if (rank[rootB] == rank[rootA]) {
                    rank[rootA]++
                }
                components--
            }
        }
        return components - 1
    }

    fun find(x: Int, parent: IntArray): Int {
        if (parent[x] != x) {
            parent[x] = find(parent[x], parent)
        }
        return parent[x]
    }
}