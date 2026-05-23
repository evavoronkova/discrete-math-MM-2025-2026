class Solution {

    fun minScore(n: Int, roads: Array<IntArray>): Int {

        val g = Array(n + 1) { mutableListOf<Pair<Int, Int>>() }

        for (r in roads) {

            val a = r[0]
            val b = r[1]
            val w = r[2]

            g[a].add(Pair(b, w))
            g[b].add(Pair(a, w))
        }

        val used = BooleanArray(n + 1)

        var ans = Int.MAX_VALUE

        fun dfs(v: Int) {

            used[v] = true

            for ((to, w) in g[v]) {

                ans = minOf(ans, w)

                if (!used[to]) {
                    dfs(to)
                }
            }
        }

        dfs(1)

        return ans
    }
}