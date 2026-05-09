class Solution {
    fun minOperationsQueries(n: Int, edges: Array<IntArray>, queries: Array<IntArray>): IntArray {
        val LOG = 14
        val WEIGHT_COUNT = 26

        // CSR-граф
        val nodeDegree = IntArray(n)
        for (edge in edges) {
            nodeDegree[edge[0]]++
            nodeDegree[edge[1]]++
        }

        val adjacencyStart = IntArray(n + 1)
        for (node in 0 until n) {
            adjacencyStart[node + 1] = adjacencyStart[node] + nodeDegree[node]
        }

        val adjacencyTarget = IntArray(adjacencyStart[n])
        val adjacencyWeight = IntArray(adjacencyStart[n])
        val insertionCursor = adjacencyStart.copyOfRange(0, n)

        for (edge in edges) {
            val sourceNode = edge[0]
            val targetNode = edge[1]
            val edgeWeight = edge[2]
            adjacencyTarget[insertionCursor[sourceNode]] = targetNode
            adjacencyWeight[insertionCursor[sourceNode]++] = edgeWeight
            adjacencyTarget[insertionCursor[targetNode]] = sourceNode
            adjacencyWeight[insertionCursor[targetNode]++] = edgeWeight
        }
        val nodeDepth = IntArray(n)
        // Таблица предков
        val ancestor = Array(LOG) { IntArray(n) { nodeIndex -> nodeIndex } }
        // prefixCount[node * WEIGHT_COUNT + w] = количество рёбер веса (w+1) на пути корень -> node
        val prefixCount = IntArray(n * WEIGHT_COUNT)

        // BFS
        val bfsQueue = IntArray(n)
        val visited = BooleanArray(n)
        var bfsHead = 0
        var bfsTail = 0
        bfsQueue[bfsTail++] = 0
        visited[0] = true
        while (bfsHead < bfsTail) {
            val currentNode = bfsQueue[bfsHead++]
            for (edgeIndex in adjacencyStart[currentNode] until adjacencyStart[currentNode + 1]) {
                val neighborNode = adjacencyTarget[edgeIndex]
                if (!visited[neighborNode]) {
                    visited[neighborNode] = true
                    nodeDepth[neighborNode] = nodeDepth[currentNode] + 1
                    ancestor[0][neighborNode] = currentNode
                    // Копируем prefix-счётчики от родителя и инкрементируем нужный вес
                    val currentOffset = currentNode * WEIGHT_COUNT
                    val neighborOffset = neighborNode * WEIGHT_COUNT
                    System.arraycopy(prefixCount, currentOffset, prefixCount, neighborOffset, WEIGHT_COUNT)
                    prefixCount[neighborOffset + adjacencyWeight[edgeIndex] - 1]++
                    bfsQueue[bfsTail++] = neighborNode
                }
            }
        }

        // Строим таблицу подъёма
        for (level in 1 until LOG) {
            val currentLevelAncestor = ancestor[level]
            val previousLevelAncestor = ancestor[level - 1]
            for (node in 0 until n) {
                currentLevelAncestor[node] = previousLevelAncestor[previousLevelAncestor[node]]
            }
        }

        val answers = IntArray(queries.size)
        for (queryIndex in queries.indices) {
            val queryNodeA = queries[queryIndex][0]
            val queryNodeB = queries[queryIndex][1]
            var nodeA = queryNodeA
            var nodeB = queryNodeB
            if (nodeDepth[nodeA] < nodeDepth[nodeB]) {
                val temporary = nodeA; nodeA = nodeB; nodeB = temporary
            }

            // Выравнивание глубины
            var depthDifference = nodeDepth[nodeA] - nodeDepth[nodeB]
            var liftLevel = 0
            while (depthDifference != 0) {
                if (depthDifference and 1 != 0) {
                    nodeA = ancestor[liftLevel][nodeA]
                }
                depthDifference = depthDifference ushr 1
                liftLevel++
            }

            // Поиск LCA
            if (nodeA != nodeB) {
                for (searchLevel in LOG - 1 downTo 0) {
                    if (ancestor[searchLevel][nodeA] != ancestor[searchLevel][nodeB]) {
                        nodeA = ancestor[searchLevel][nodeA]
                        nodeB = ancestor[searchLevel][nodeB]
                    }
                }
                nodeA = ancestor[0][nodeA] // nodeA теперь LCA
            }
            val lcaNode = nodeA

            // Длина пути через глубины
            val totalEdgeCount = nodeDepth[queryNodeA] + nodeDepth[queryNodeB] - 2 * nodeDepth[lcaNode]

            // Максимальная частота веса через prefix-суммы
            val offsetA = queryNodeA * WEIGHT_COUNT
            val offsetB = queryNodeB * WEIGHT_COUNT
            val offsetLCA = lcaNode * WEIGHT_COUNT
            var maxWeightFrequency = 0
            for (weight in 0 until WEIGHT_COUNT) {
                val weightCount = prefixCount[offsetA + weight] + prefixCount[offsetB + weight] -
                        2 * prefixCount[offsetLCA + weight]
                if (weightCount > maxWeightFrequency) {
                    maxWeightFrequency = weightCount
                }
            }
            answers[queryIndex] = totalEdgeCount - maxWeightFrequency
        }
        return answers
    }
}
