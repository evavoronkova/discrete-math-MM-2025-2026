class Solution {
    fun minimumTime(n: Int, edges: Array<IntArray>, disappear: IntArray): IntArray {
        val graph = Array(n) { mutableListOf<Pair<Int, Int>>() }
        for (edge in edges) {
            val u = edge[0]
            val v = edge[1]
            val length = edge[2]
            graph[u].add(v to length)
            graph[v].add(u to length)
        }
        val answer = IntArray(n) { -1 }
        val dist = LongArray(n) { Long.MAX_VALUE }
        dist[0] = 0L
        val priorityQ = java.util.PriorityQueue<Pair<Long, Int>>(compareBy { it.first })
        priorityQ.offer(0L to 0)

        while (priorityQ.isNotEmpty()) {
            val (time, node) = priorityQ.poll()
            if (time > dist[node]) continue
            if (time >= disappear[node]) continue
            if (answer[node] == -1) {
                answer[node] = time.toInt()
            }
            for ((neighbor, weight) in graph[node]) {
                val newTime = time + weight
                if (newTime < dist[neighbor] && newTime < disappear[neighbor]) {
                    dist[neighbor] = newTime
                    priorityQ.offer(newTime to neighbor)
                }
            }
        }
        return answer
    }
}