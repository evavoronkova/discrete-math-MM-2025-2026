class Solution {
    fun getAncestors(n: Int, edges: Array<IntArray>): List<List<Int>> {
        val graph = Array(n) { mutableListOf<Int>() }
        val inDegree = IntArray(n)
        for (edge in edges) {
            val from = edge[0]
            val to = edge[1]
            graph[from].add(to)
            inDegree[to]++
        }
        val queue = java.util.LinkedList<Int>()
        for (i in 0..<n) {
            if (inDegree[i] == 0) {
                queue.offer(i)
            }
        }
        val ancestors = Array(n) { java.util.TreeSet<Int>() }
        val topoOrder = mutableListOf<Int>()
        while (!queue.isEmpty()) {
            val u = queue.poll()
            topoOrder.add(u)
            for (v in graph[u]) {
                ancestors[v].add(u)
                ancestors[v].addAll(ancestors[u])
                inDegree[v]--
                if (inDegree[v] == 0) {
                    queue.offer(v)
                }
            }
        }
        return ancestors.map { it.toList() }
    }
}