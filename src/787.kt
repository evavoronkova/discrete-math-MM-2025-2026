class Solution {
    val listOfPaths = mutableListOf<MutableList<Int>>()

    fun findCheapestPrice(n: Int, flights: Array<IntArray>, src: Int, dst: Int, k: Int): Int {
        val dist = IntArray(n) { Int.MAX_VALUE }
        dist[src] = 0
        repeat(k + 1){
            val prev = dist.clone()
            for((u, v, price) in flights){
                if(prev[u] != Int.MAX_VALUE){
                    dist[v] = if(dist[v] > prev[u] + price) prev[u] + price else dist[v]
                }
            }
        }
        return if (dist[dst] == Int.MAX_VALUE) -1 else dist[dst]
    }
}
