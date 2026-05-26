class Solution {
    class State(val time: Int, val r: Int, val c: Int, val parity: Int)

    fun minTimeToReach(moveTime: Array<IntArray>): Int {
        val n = moveTime.size
        val m = moveTime[0].size

        val dist = IntArray(n * m * 2) { Int.MAX_VALUE }
        dist[0] = 0

        val pq = java.util.PriorityQueue<State>(compareBy { it.time })
        pq.add(State(0, 0, 0, 0))

        val dr = intArrayOf(-1, 1, 0, 0)
        val dc = intArrayOf(0, 0, -1, 1)

        while (pq.isNotEmpty()) {
            val cur = pq.poll()
            val time = cur.time
            val r = cur.r
            val c = cur.c
            val parity = cur.parity

            val dIdx = (r * m + c) * 2 + parity
            if (time > dist[dIdx]) continue
            if (r == n - 1 && c == m - 1) return time

            val nextParity = 1 - parity
            val duration = if (parity == 0) 1 else 2

            for (i in 0..3) {
                val nr = r + dr[i]
                val nc = c + dc[i]
                if (nr in 0 until n && nc in 0 until m) {
                    val nDIdx = (nr * m + nc) * 2 + nextParity
                    val constraint = moveTime[nr][nc]
                    val start = if (time > constraint) time else constraint
                    val arrival = start + duration

                    if (arrival < dist[nDIdx]) {
                        dist[nDIdx] = arrival
                        pq.add(State(arrival, nr, nc, nextParity))
                    }
                }
            }
        }
        return -1
    }
}