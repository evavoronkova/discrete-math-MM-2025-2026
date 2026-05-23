class Solution {

    fun shortestAlternatingPaths(
        n: Int,
        redEdges: Array<IntArray>,
        blueEdges: Array<IntArray>
    ): IntArray {

        val red = Array(n) { mutableListOf<Int>() }
        val blue = Array(n) { mutableListOf<Int>() }

        for (e in redEdges) {
            red[e[0]].add(e[1])
        }

        for (e in blueEdges) {
            blue[e[0]].add(e[1])
        }

        val ans = IntArray(n) { -1 }

        val used = Array(n) { BooleanArray(2) }

        val q = ArrayDeque<IntArray>()

        q.add(intArrayOf(0, 0, 0))
        q.add(intArrayOf(0, 1, 0))

        used[0][0] = true
        used[0][1] = true

        while (q.isNotEmpty()) {

            val cur = q.removeFirst()

            val v = cur[0]
            val color = cur[1]
            val dist = cur[2]

            if (ans[v] == -1 || dist < ans[v]) {
                ans[v] = dist
            }

            if (color == 0) {

                for (to in blue[v]) {

                    if (!used[to][1]) {

                        used[to][1] = true

                        q.add(
                            intArrayOf(
                                to,
                                1,
                                dist + 1
                            )
                        )
                    }
                }

            } else {

                for (to in red[v]) {

                    if (!used[to][0]) {

                        used[to][0] = true

                        q.add(
                            intArrayOf(
                                to,
                                0,
                                dist + 1
                            )
                        )
                    }
                }
            }
        }

        return ans
    }
}