class Solution {
    fun closestMeetingNode(edges: IntArray, node1: Int, node2: Int): Int {
        val n = edges.size
        val dist1 = getDistances(edges, node1, n)
        val dist2 = getDistances(edges, node2, n)
        var minMaxDist = Int.MAX_VALUE
        var resultNode = -1
        for (i in 0 until n) {
            if (dist1[i] != -1 && dist2[i] != -1) {
                val currentMax = maxOf(dist1[i], dist2[i])
                if (currentMax < minMaxDist) {
                    minMaxDist = currentMax
                    resultNode = i
                }
            }
        }
        return resultNode
    }
    private fun getDistances(edges: IntArray, start: Int, n: Int): IntArray {
        val dist = IntArray(n) { -1 }
        var curr = start
        var d = 0
        while (curr != -1 && dist[curr] == -1) {
            dist[curr] = d++
            curr = edges[curr]
        }
        return dist
    }
}