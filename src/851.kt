class Solution {
    fun loudAndRich(richer: Array<IntArray>, quiet: IntArray): IntArray {
        val n = quiet.size
        val graph = Array(n) { mutableListOf<Int>() }
        for ((a, b) in richer) {
            graph[b].add(a)
        }
        val ans = IntArray(n) { -1 }
        fun dfs(u: Int): Int {
            if (ans[u] != -1) return ans[u]
            var best = u
            for (v in graph[u]) {
                val cand = dfs(v)
                if (quiet[cand] < quiet[best]) best = cand
            }
            ans[u] = best
            return best
        }
        for (i in 0 until n) dfs(i)
        return ans
    }
}
