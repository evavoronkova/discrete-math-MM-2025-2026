class Solution {
    fun findCheapestPrice(n: Int, flights: Array<IntArray>, src: Int, dst: Int, k: Int): Int {
        var dist = IntArray(n) { Int.MAX_VALUE }
        dist[src] = 0

        repeat(k + 1) {
            val tempDist = dist.copyOf()

            for (flight in flights) {
                val (from, to, price) = flight
                if (dist[from] != Int.MAX_VALUE) {
                    tempDist[to] = minOf(tempDist[to], dist[from] + price)
                }
            }

            dist = tempDist
        }

        return if (dist[dst] == Int.MAX_VALUE) -1 else dist[dst]
    }
}