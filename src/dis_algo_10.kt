class Solution {

    fun longestCycle(edges: IntArray): Int {

        val n = edges.size

        val used = BooleanArray(n)

        val dist = IntArray(n) { -1 }

        var ans = -1

        for (i in 0 until n) {

            if (used[i]) {
                continue
            }

            var v = i
            var step = 0

            while (v != -1 && !used[v]) {

                used[v] = true
                dist[v] = step++

                v = edges[v]
            }

            if (v != -1 && dist[v] != -1) {
                ans = maxOf(ans, step - dist[v])
            }

            v = i

            while (v != -1 && dist[v] != -1) {

                val nxt = edges[v]

                dist[v] = -1

                v = nxt
            }
        }

        return ans
    }
}