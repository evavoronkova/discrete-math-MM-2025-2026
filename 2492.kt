class Solution {
    fun minScore(n: Int, roads: Array<IntArray>): Int {
        // Ответ = минимальный вес ребра во всей компоненте связности вершины 1
        val numberOfRoads = roads.size

        // Считаем степени вершин для CSR
        val vertexDegree = IntArray(n + 1)
        for (currentRoad in roads) {
            vertexDegree[currentRoad[0]]++
            vertexDegree[currentRoad[1]]++
        }

        // Префиксные суммы - позиция старта каждой вершины в плоском массиве
        val start = IntArray(n + 2)
        for (i in 1..n) start[i + 1] = start[i] + vertexDegree[i]

        // Плоские CSR-массивы (граф неориентированный, каждое ребро в обе стороны)
        val adjacencyNeighbor = IntArray(2 * numberOfRoads)
        val adjacencyDistance = IntArray(2 * numberOfRoads)
        // position - курсоры заполнения для каждого сегмента CSR
        val position = start.copyOfRange(0, n + 1)

        for (currentRoad in roads) {
            val a = currentRoad[0]; val b = currentRoad[1]; val roadDistance = currentRoad[2]
            adjacencyNeighbor[position[a]] = b;  adjacencyDistance[position[a]] = roadDistance;  position[a]++
            adjacencyNeighbor[position[b]] = a;  adjacencyDistance[position[b]] = roadDistance;  position[b]++
        }

        // BFS на массивах с head/tail
        val visited = BooleanArray(n + 1)
        val queue = IntArray(n + 1)
        var head = 0; var tail = 0
        queue[tail++] = 1
        visited[1] = true

        var minDistance = Int.MAX_VALUE

        // Обходим компоненту вершины 1 и обновляем минимальный вес ребра
        while (head < tail) {
            val node = queue[head++]
            var i = start[node]
            val end = start[node + 1]
            while (i < end) {
                val neighbor = adjacencyNeighbor[i]
                val edgeDistance = adjacencyDistance[i]
                // каждое ребро увидим дважды, но min от этого не меняется
                if (edgeDistance < minDistance) {
                    minDistance = edgeDistance
                }
                if (!visited[neighbor]) {
                    visited[neighbor] = true
                    queue[tail++] = neighbor
                }
                i++
            }
        }

        return minDistance
    }
}
