class Solution {

    fun validArrangement(pairs: Array<IntArray>): Array<IntArray> {

        val g = HashMap<Int, ArrayDeque<Int>>()

        val inDeg = HashMap<Int, Int>()
        val outDeg = HashMap<Int, Int>()

        for (p in pairs) {

            val a = p[0]
            val b = p[1]

            g.putIfAbsent(a, ArrayDeque())
            g[a]!!.add(b)

            outDeg[a] = (outDeg[a] ?: 0) + 1
            inDeg[b] = (inDeg[b] ?: 0) + 1
        }

        var start = pairs[0][0]

        for (v in g.keys) {

            val out = outDeg[v] ?: 0
            val inn = inDeg[v] ?: 0

            if (out - inn == 1) {
                start = v
                break
            }
        }

        val path = mutableListOf<Int>()

        fun dfs(v: Int) {

            val q = g[v]

            while (q != null && q.isNotEmpty()) {

                val to = q.removeFirst()

                dfs(to)
            }

            path.add(v)
        }

        dfs(start)

        path.reverse()

        val ans = Array(path.size - 1) { IntArray(2) }

        for (i in 0 until path.size - 1) {

            ans[i][0] = path[i]
            ans[i][1] = path[i + 1]
        }

        return ans
    }
}
