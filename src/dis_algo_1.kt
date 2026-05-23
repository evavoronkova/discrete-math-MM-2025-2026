class Solution {
    fun findCircleNum(isConnected: Array<IntArray>): Int {
        val n = isConnected.size
        val used = BooleanArray(n)
        var cnt = 0
        val queue = java.util.ArrayDeque<Int>()

        for (i in 0 until n) {
            if (!used[i]) {
                cnt++
                queue.add(i)
                used[i] = true

                while (queue.isNotEmpty()) {
                    val v = queue.poll()
                    for (to in 0 until n) {
                        if (isConnected[v][to] == 1 && !used[to]) {
                            used[to] = true
                            queue.add(to)
                        }
                    }
                }
            }
        }

        return cnt
    }
}