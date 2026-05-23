class Solution {
    fun minOperationsQueries(n: Int, edges: Array<IntArray>, queries: Array<IntArray>): IntArray {
        val LOG = 14            // 2^14 > 10^4, покрывает максимальную глубину дерева
        val WEIGHT_COUNT = 26   // веса от 1 до 26

        // CSR-граф: сначала степени, потом префиксные суммы дадут позиции начала списков
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
        // insertionCursor чтобы заполнять каждый сегмент CSR последовательно
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
        // Таблица предков: ancestor[k][v] = 2^k-й предок v (для двоичного подъёма и LCA)
        val ancestor = Array(LOG) { IntArray(n) { nodeIndex -> nodeIndex } }
        // prefixCount[node * WEIGHT_COUNT + w] = количество рёбер веса (w+1) на пути корень -> node
        val prefixCount = IntArray(n * WEIGHT_COUNT)

        // BFS из корня (0): заполняем глубины, прямых родителей и prefix-счётчики
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
                    // Копируем prefix-счётчики от родителя и инкрементируем нужный вес (System.arraycopy быстрее ручного цикла)
                    val currentOffset = currentNode * WEIGHT_COUNT
                    val neighborOffset = neighborNode * WEIGHT_COUNT
                    System.arraycopy(prefixCount, currentOffset, prefixCount, neighborOffset, WEIGHT_COUNT)
                    prefixCount[neighborOffset + adjacencyWeight[edgeIndex] - 1]++
                    bfsQueue[bfsTail++] = neighborNode
                }
            }
        }

        // Строим таблицу подъёма: 2^k-й предок = 2^(k-1)-й предок от 2^(k-1)-го предка
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
            // делаем nodeA глубже или равной, чтобы потом её поднимать
            if (nodeDepth[nodeA] < nodeDepth[nodeB]) {
                val temporary = nodeA; nodeA = nodeB; nodeB = temporary
            }

            // Выравнивание глубины: разность разбираем по битам, поднимаем nodeA на 2^liftLevel
            var depthDifference = nodeDepth[nodeA] - nodeDepth[nodeB]
            var liftLevel = 0
            while (depthDifference != 0) {
                if (depthDifference and 1 != 0) {
                    nodeA = ancestor[liftLevel][nodeA]
                }
                depthDifference = depthDifference ushr 1
                liftLevel++
            }

            // Поиск LCA: пока вершины разные, поднимаем обе на максимальный 2^k где они ещё различны
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

            // Длина пути: dist(a, b) = depth(a) + depth(b) - 2 * depth(LCA)
            val totalEdgeCount = nodeDepth[queryNodeA] + nodeDepth[queryNodeB] - 2 * nodeDepth[lcaNode]

            // Для каждого веса w: count_on_path(w) = prefix[a] + prefix[b] - 2 * prefix[LCA]
            // Ответ = всего рёбер минус максимальная частота одного веса (самый частый оставляем)
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
