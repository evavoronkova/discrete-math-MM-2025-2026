class Solution {
    fun reachableNodes(n: Int, edges: Array<IntArray>, restricted: IntArray): Int {
        // Помечаем запрещённые вершины, потом этот же массив используем как visited для BFS
        val banned = BooleanArray(n)
        for (node in restricted) {
            banned[node] = true
        }

        // граф в виде массивов смежности (2 * edgeCount т.к. неориентированный)
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

        // Для каждого ребра добавляем оба направления
        for (edge in edges) {
            val fromNode = edge[0]
            val toNode = edge[1]
            addDirectedEdge(fromNode, toNode)
            addDirectedEdge(toNode, fromNode)
        }

        // BFS из вершины 0 на массивах (head/tail)
        val queue = IntArray(n)
        var queueHead = 0
        var queueTail = 0
        queue[queueTail] = 0
        queueTail = queueTail + 1
        banned[0] = true  // banned работает и как visited, чтобы не зайти в 0 повторно
        var reachableCount = 0
        while (queueHead < queueTail) {
            val currentNode = queue[queueHead]
            queueHead = queueHead + 1
            reachableCount++
            // идём по связному списку рёбер из currentNode
            var edgeIndex = head[currentNode]
            while (edgeIndex != -1) {
                val neighborNode = to[edgeIndex]
                if (!banned[neighborNode]) {
                    banned[neighborNode] = true  // сразу и visited, и фильтр запрещённых
                    queue[queueTail] = neighborNode
                    queueTail = queueTail + 1
                }
                edgeIndex = next[edgeIndex]
            }
        }
        return reachableCount
    }
}
