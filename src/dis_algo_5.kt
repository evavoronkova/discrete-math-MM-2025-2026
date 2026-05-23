class Solution {

    data class State(
        val t: Long,
        val x: Int,
        val y: Int,
        val p: Int
    )

    fun minTimeToReach(moveTime: Array<IntArray>): Int {

        val n = moveTime.size
        val m = moveTime[0].size

        val inf = Long.MAX_VALUE

        val dist = Array(n) { Array(m) { LongArray(2) { inf } } }

        val pq = java.util.PriorityQueue<State> { a, b ->
            a.t.compareTo(b.t)
        }

        dist[0][0][0] = 0
        pq.add(State(0L, 0, 0, 0))

        val dx = intArrayOf(1, -1, 0, 0)
        val dy = intArrayOf(0, 0, 1, -1)

        while (pq.isNotEmpty()) {

            val cur = pq.poll()

            val t = cur.t
            val x = cur.x
            val y = cur.y
            val p = cur.p

            if (t != dist[x][y][p]) continue

            if (x == n - 1 && y == m - 1) {
                return t.toInt()
            }

            val cost = if (p == 0) 1L else 2L

            for (k in 0 until 4) {

                val nx = x + dx[k]
                val ny = y + dy[k]

                if (nx !in 0 until n || ny !in 0 until m) continue

                val start = maxOf(t, moveTime[nx][ny].toLong())
                val nt = start + cost

                if (nt < dist[nx][ny][1 - p]) {

                    dist[nx][ny][1 - p] = nt

                    pq.add(State(nt, nx, ny, 1 - p))
                }
            }
        }

        return -1
    }
}