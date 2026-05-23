import kotlin.math.min

class Solution {
    fun minimumCost(source: String, target: String, original: Array<String>, changed: Array<String>, cost: IntArray): Long {
        val n = source.length
        val INF = 1e15.toLong()
        val rulesByLen = mutableMapOf<Int, MutableList<Triple<String, String, Int>>>()
        for (i in original.indices) {
            rulesByLen.getOrPut(original[i].length) { mutableListOf() }.add(Triple(original[i], changed[i], cost[i]))
        }
        val costMap = mutableMapOf<Int, Map<String, Map<String, Long>>>()
        for ((len, triples) in rulesByLen) {
            val nodes = triples.flatMap { listOf(it.first, it.second) }.toSet().toList()
            val idMap = nodes.withIndex().associate { it.value to it.index }
            val m = nodes.size
            val dist = Array(m) { LongArray(m) { INF } }
            for (i in 0 until m) dist[i][i] = 0L
            for ((u, v, c) in triples) {
                val i = idMap[u]!!
                val j = idMap[v]!!
                dist[i][j] = min(dist[i][j], c.toLong())
            }
            for (k in 0 until m) {
                for (i in 0 until m) {
                    if (dist[i][k] == INF) continue
                    for (j in 0 until m) {
                        if (dist[k][j] == INF) continue
                        dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])
                    }
                }
            }
            val resMap = mutableMapOf<String, MutableMap<String, Long>>()
            for (i in 0 until m) {
                for (j in 0 until m) {
                    if (dist[i][j] != INF) {
                        resMap.getOrPut(nodes[i]) { mutableMapOf() }[nodes[j]] = dist[i][j]
                    }
                }
            }
            costMap[len] = resMap
        }
        val dp = LongArray(n + 1) { INF }
        dp[0] = 0L
        val lengths = costMap.keys.sorted()
        for (i in 1..n) {
            if (source[i - 1] == target[i - 1]) {
                dp[i] = dp[i - 1]
            }
            for (len in lengths) {
                if (i >= len && dp[i - len] != INF) {
                    val s = source.substring(i - len, i)
                    val t = target.substring(i - len, i)
                    val c = costMap[len]?.get(s)?.get(t) ?: INF
                    if (c != INF) {
                        dp[i] = min(dp[i], dp[i - len] + c)
                    }
                }
            }
        }
        return if (dp[n] >= INF) -1L else dp[n]
    }
}
