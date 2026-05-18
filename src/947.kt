class Solution {
    fun removeStones(stones: Array<IntArray>): Int {
        val offset = 10001

        val parent = IntArray(20002) { it }
        val size = IntArray(20002) { 1 }

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
        }

        for ((r, c) in stones) {
            union(r,c + offset)
        }

        val components = stones.map { find(it[0]) }.toSet().size

        return stones.size - components
    }
}