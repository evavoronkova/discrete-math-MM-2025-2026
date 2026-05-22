class Solution {
    fun minSwapsCouples(row: IntArray): Int {
        val n = row.size / 2
        val parent = IntArray(n) { it }
        var components = n

        fun find(x: Int): Int {
            if (parent[x] != x) parent[x] = find(parent[x])
            return parent[x]
        }

        fun union(x: Int, y: Int) {
            val rx = find(x)
            val ry = find(y)
            if (rx != ry) {
                parent[rx] = ry
                components--
            }
        }

        for (i in 0..<n) {
            union(row[2 * i] / 2, row[2 * i + 1] / 2)
        }

        return n - components
    }
}