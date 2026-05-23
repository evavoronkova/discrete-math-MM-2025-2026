class Solution {

    fun longestCycle(edges: IntArray): Int {

        val n = edges.size

        val vis = IntArray(n)

        var timer = 1
        var ans = -1

        for (i in 0 until n) {

            if (vis[i] != 0) {
                continue
            }

            var v = i
            val start = timer

            while (v != -1 && vis[v] == 0) {

                vis[v] = timer++
                v = edges[v]
            }

            if (v != -1 && vis[v] >= start) {
                ans = maxOf(ans, timer - vis[v])
            }
        }

        return ans
    }
}