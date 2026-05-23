class Solution {
    fun reachableNodes(edges: Array<IntArray>, maxMoves: Int, n: Int): Int {
        // Считаем степени вершин для точного размера массивов (CSR-формат)
        val nodeDegree = IntArray(n)
        for (edge in edges) {
            nodeDegree[edge[0]]++
            nodeDegree[edge[1]]++
        }

        // Списки смежности фиксированного размера
        val adjacencyList = Array(n) { nodeIndex -> IntArray(nodeDegree[nodeIndex]) }
        val fillPointer = IntArray(n)
        for (edge in edges) {
            val sourceNode = edge[0]
            val destinationNode = edge[1]
            val subNodeCount = edge[2]
            // Упаковываем (subNodeCount, сосед) в один Int (n до 3000 влезает в 12 бит)
            adjacencyList[sourceNode][fillPointer[sourceNode]++] = (subNodeCount shl 12) or destinationNode
            adjacencyList[destinationNode][fillPointer[destinationNode]++] = (subNodeCount shl 12) or sourceNode
        }

        // Дейкстра по исходным вершинам, вес ребра u -> v равен subNodeCount + 1
        val distanceFromStart = IntArray(n) { Int.MAX_VALUE }
        distanceFromStart[0] = 0
        val priorityQueue = java.util.PriorityQueue<Long>()
        priorityQueue.add(0L) // distance=0, node=0
        while (priorityQueue.isNotEmpty()) {
            val packedEntry = priorityQueue.poll()
            // упаковка в Long: старшие биты - расстояние, младшие 12 бит - номер вершины
            val currentDistance = (packedEntry ushr 12).toInt()
            val currentNode = (packedEntry and 0xFFFL).toInt()
            // пропускаем устаревшие записи из кучи
            if (currentDistance > distanceFromStart[currentNode]) {
                continue
            }
            for (packedNeighbor in adjacencyList[currentNode]) {
                val neighborNode = packedNeighbor and 0xFFF
                val subNodeCount = packedNeighbor ushr 12
                val newDistance = currentDistance + subNodeCount + 1
                if (newDistance < distanceFromStart[neighborNode]) {
                    distanceFromStart[neighborNode] = newDistance
                    priorityQueue.add((newDistance.toLong() shl 12) or neighborNode.toLong())
                }
            }
        }

        // Считаем исходные вершины, до которых доехали за маршрут не больше maxMoves
        var totalReachableNodes = 0
        for (nodeIndex in 0 until n) {
            if (distanceFromStart[nodeIndex] <= maxMoves) {
                totalReachableNodes++
            }
        }

        // Для каждого ребра считаем промежуточные вершины с двух сторон, но не больше subNodeCount
        for (edge in edges) {
            val sourceNode = edge[0]
            val destinationNode = edge[1]
            val subNodeCount = edge[2]
            val reachableFromSource = if (distanceFromStart[sourceNode] <= maxMoves) {
                maxMoves - distanceFromStart[sourceNode]
            } else {
                0
            }
            val reachableFromDestination = if (distanceFromStart[destinationNode] <= maxMoves) {
                maxMoves - distanceFromStart[destinationNode]
            } else {
                0
            }

            // min() защищает от двойного счёта когда обе стороны перекрывают всё ребро
            totalReachableNodes += minOf(subNodeCount, reachableFromSource + reachableFromDestination)
        }
        return totalReachableNodes
    }
}
