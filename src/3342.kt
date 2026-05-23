class Solution {
    class State(val time: Int, val r: Int, val c: Int, val parity: Int)

    fun minTimeToReach(moveTime: Array<IntArray>): Int {
        val n = moveTime.size
        val m = moveTime[0].size

        val dist = Array(n) { Array(m) { intArrayOf(Int.MAX_VALUE, Int.MAX_VALUE) } }
        dist[0][0][0] = 0

        val pq = java.util.PriorityQueue<State>(compareBy { it.time })
        pq.add(State(0, 0, 0, 0))

        val directions = arrayOf(0 to 1, 0 to -1, 1 to 0, -1 to 0)

        while (pq.isNotEmpty()) {
            val curState = pq.poll()
            val time = curState.time
            val r = curState.r
            val c = curState.c
            val parity = curState.parity

            if (time > dist[r][c][parity]) continue

            if (r == n - 1 && c == m - 1) return time

            val nextParity = 1 - parity
            val moveDuration = if (parity == 0) 1 else 2

            for ((dr, dc) in directions) {
                val nr = r + dr
                val nc = c + dc

                if (nr in 0..<n && nc in 0..<m) {
                    val startTime = maxOf(time, moveTime[nr][nc])
                    val arrivalTime = startTime + moveDuration

                    if (arrivalTime < dist[nr][nc][nextParity]) {
                        dist[nr][nc][nextParity] = arrivalTime
                        pq.add(State(arrivalTime, nr, nc, nextParity))
                    }
                }
            }
        }
        return -1
    }
}