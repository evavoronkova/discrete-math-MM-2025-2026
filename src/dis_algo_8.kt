class Solution {

    fun validArrangement(pairs: Array<IntArray>): Array<IntArray> {

        val g = HashMap<Int, MutableList<Int>>()

        val inDeg = HashMap<Int, Int>()
        val outDeg = HashMap<Int, Int>()

        for (p in pairs) {

            val a = p[0]
            val b = p[1]

            g.computeIfAbsent(a) { mutableListOf() }.add(b)

            outDeg[a] = (outDeg[a] ?: 0) + 1
            inDeg[b] = (inDeg[b] ?: 0) + 1
        }

        var start = pairs[0][0]

        for ((v, out) in outDeg) {

            val inn = inDeg[v] ?: 0

            if (out - inn == 1) {
                start = v
                break
            }
        }

        val ptr = HashMap<Int, Int>()

        val st = IntArray(pairs.size + 1)
        var top = 0

        st[top++] = start

        val path = IntArray(pairs.size + 1)
        var sz = 0

        while (top > 0) {

            val v = st[top - 1]

            val list = g[v]
            val idx = ptr[v] ?: 0

            if (list != null && idx < list.size) {

                st[top++] = list[idx]

                ptr[v] = idx + 1

            } else {

                path[sz++] = v
                top--
            }
        }

        val ans = Array(pairs.size) { IntArray(2) }

        var j = sz - 1

        for (i in 0 until pairs.size) {

            ans[i][0] = path[j]
            ans[i][1] = path[j - 1]

            j--
        }

        return ans
    }
}