class Solution {
    fun minimumCost(n: Int, edges: Array<IntArray>, query: Array<IntArray>): IntArray {
        val dsu = DSU(n)
        for (e in edges) {
            dsu.union(e[0], e[1])
        }
        val compAnd = IntArray(n) { -1 }
        for (e in edges) {
            val root = dsu.find(e[0])
            if (compAnd[root] == -1) {
                compAnd[root] = e[2]
            } else {
                compAnd[root] = compAnd[root] and e[2]
            }
        }
        val ans = IntArray(query.size)
        for (i in 0..<query.size) {
            val s = query[i][0]
            val t = query[i][1]
            val rs = dsu.find(s)
            val rt = dsu.find(t)
            if (rs != rt) {
                ans[i] = -1
            } else {
                if (compAnd[rs] == -1) {
                    ans[i] = -1
                } else {
                    ans[i] = compAnd[rs]
                }
            }
        }
        return ans
    }
}

class DSU(n: Int) {
    lateinit var parent: IntArray
    lateinit var rank: IntArray

    init {
        parent = IntArray(n) { it }
        rank = IntArray(n) { 0 }
    }

    fun find(x: Int): Int {
        if (parent[x] != x) {
            parent[x] = find(parent[x])
        }
        return parent[x]
    }

    fun union(a: Int, b: Int) {
        val ra = find(a)
        val rb = find(b)
        if (ra == rb) return
        if (rank[ra] < rank[rb]) {
            parent[ra] = rb
        } else if (rank[ra] > rank[rb]) {
            parent[rb] = ra
        } else {
            parent[rb] = ra
            rank[ra]++
        }
    }
}