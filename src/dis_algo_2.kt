class Solution {

    fun findSafeWalk(grid: List<List<Int>>, health: Int): Boolean {

        val n = grid.size
        val m = grid[0].size

        val dist = Array(n) { IntArray(m) { Int.MAX_VALUE } }

        val dq = ArrayDeque<Pair<Int, Int>>()

        dist[0][0] = grid[0][0]
        dq.add(Pair(0, 0))

        val dx = intArrayOf(1, -1, 0, 0)
        val dy = intArrayOf(0, 0, 1, -1)

        while (dq.isNotEmpty()) {

            val (x, y) = dq.removeFirst()

            for (k in 0 until 4) {

                val nx = x + dx[k]
                val ny = y + dy[k]

                if (nx !in 0 until n || ny !in 0 until m) {
                    continue
                }

                val w = grid[nx][ny]

                if (dist[x][y] + w < dist[nx][ny]) {

                    dist[nx][ny] = dist[x][y] + w

                    if (w == 0) {
                        dq.addFirst(Pair(nx, ny))
                    } else {
                        dq.addLast(Pair(nx, ny))
                    }
                }
            }
        }

        return dist[n - 1][m - 1] < health
    }
}