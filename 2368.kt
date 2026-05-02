class Solution {
    fun reachableNodes(n: Int, edges: Array<IntArray>, restricted: IntArray): Int {
        val banned = BooleanArray(n)
        for (node in restricted) {
            banned[node] = true
        }
        
        val edgeCount = edges.size
        val head = IntArray(n) { -1 }
        val to = IntArray(2 * edgeCount)
        val next = IntArray(2 * edgeCount)
        var currentIndex = 0

        fun addDirectedEdge(fromNode: Int, toNode: Int) {
            to[currentIndex] = toNode
            next[currentIndex] = head[fromNode]
            head[fromNode] = currentIndex
            currentIndex++
        }

        for (edge in edges) {
            val fromNode = edge[0]
            val toNode = edge[1]
            addDirectedEdge(fromNode, toNode)
            addDirectedEdge(toNode, fromNode)
        }

        val queue = IntArray(n)
        var queueHead = 0
        var queueTail = 0
        queue[queueTail] = 0
        queueTail = queueTail + 1
        banned[0] = true
        var reachableCount = 0
        while (queueHead < queueTail) {
            val currentNode = queue[queueHead]
            queueHead = queueHead + 1
            reachableCount++
            var edgeIndex = head[currentNode]
            while (edgeIndex != -1) {
                val neighborNode = to[edgeIndex]
                if (!banned[neighborNode]) {
                    banned[neighborNode] = true
                    queue[queueTail] = neighborNode
                    queueTail = queueTail + 1
                }
                edgeIndex = next[edgeIndex]
            }
        }
        return reachableCount
    }
}
