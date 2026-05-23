class Solution {

    fun longestCycle(edges: IntArray): Int {

        val n = edges.size

        val used = BooleanArray(n)

        var ans = -1

        for (i in 0 until n) {

            if (used[i]) {
                continue
            }

            var v = i
            var step = 0

            val pos = HashMap<Int, Int>()

            while (v != -1 && !used[v]) {

                used[v] = true

                pos[v] = step++

                v = edges[v]
            }

            if (v != -1 && pos.containsKey(v)) {

                ans = maxOf(
                    ans,
                    step - pos[v]!!
                )
            }
        }

        return ans
    }
}