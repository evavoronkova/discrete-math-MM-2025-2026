class Solution {
    fun equationsPossible(equations: Array<String>): Boolean {
        val parent = IntArray(26) { it }

        fun find(x: Int): Int {
            if (parent[x] != x) parent[x] = find(parent[x])
            return parent[x]
        }

        fun union(x: Int, y: Int) {
            val rootX = find(x)
            val rootY = find(y)
            if (rootX != rootY) parent[rootX] = rootY
        }

        fun charToIdx(c: Char) = c - 'a'

        for (eq in equations) {
            if (eq[1] == '=') {
                val u = charToIdx(eq[0])
                val v = charToIdx(eq[3])
                union(u, v)
            }
        }

        for (eq in equations) {
            if (eq[1] == '!') {
                val u = charToIdx(eq[0])
                val v = charToIdx(eq[3])
                if (find(u) == find(v)) return false
            }
        }

        return true
    }
}