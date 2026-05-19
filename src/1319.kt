class Solution {
    fun makeConnected(n: Int, connections: Array<IntArray>): Int {
        if (connections.size < n - 1) return -1

        val parent = IntArray(n) { it }
        val size = IntArray(n) { 1 }
        var components = n

        fun find(x: Int): Int {
            if (parent[x] != x) {
                parent[x] = find(parent[x])
            }
            return parent[x]
        }

        fun union(x: Int, y: Int) {
            var rootX = find(x)
            var rootY = find(y)

            if (rootX == rootY) return

            if (size[rootX] < size[rootY]) {
                val temp = rootX
                rootX = rootY
                rootY = temp
            }

            parent[rootY] = rootX
            size[rootX] += size[rootY]
            components--
        }

        for ((r, c) in connections) {
            union(r, c)
        }

        return components - 1
    }
}