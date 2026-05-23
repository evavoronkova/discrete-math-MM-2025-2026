class Solution {
    fun minMaxWeight(n: Int, edges: Array<IntArray>, threshold: Int): Int {
        // Строим обратный граф: для ребра a -> b кладём (a, w) в reverseAdjacency[b]
        val reverseAdjacency = Array(n) { ArrayList<IntArray>(4) }
        for (edge in edges) {
            val (a, b, w) = edge
            reverseAdjacency[b].add(intArrayOf(a, w))
        }
        // Сортируем по весу, чтобы делать break когда вес уже превысил порог
        for (i in 0 until n) {
            reverseAdjacency[i].sortBy { it[1] }
        }

        val visited = BooleanArray(n)
        val queue = IntArray(n)       // BFS-очередь на массиве (head/tail)
        val outUsed = IntArray(n)     // счётчик уже потраченных исходящих рёбер у вершины

        // Проверка: при данном maxWeight достижимы ли все вершины с учётом threshold
        fun canAchieve(maxWeight: Int): Boolean {
            visited.fill(false)
            outUsed.fill(0)
            var head = 0
            var tail = 0
            queue[tail++] = 0
            visited[0] = true
            var visitedCount = 1

            // BFS по обратному графу из 0
            while (head < tail) {
                val currentVertex = queue[head++]
                for (next in reverseAdjacency[currentVertex]) {
                    val neighbor = next[0]
                    val w = next[1]
                    // список отсортирован, дальше только большие веса
                    if (w > maxWeight) {
                        break
                    }
                    // neighbor это источник ребра в исходном графе, проверяем его квоту
                    if (outUsed[neighbor] >= threshold) {
                        continue
                    }
                    outUsed[neighbor]++
                    if (!visited[neighbor]) {
                        visited[neighbor] = true
                        queue[tail++] = neighbor
                        visitedCount++
                    }
                }
            }
            // дошли до всех n вершин -> условие выполнимо
            return visitedCount == n
        }

        // Сортируем все веса, будем бинарно искать ответ по ним
        val uniqueSortedWeights = IntArray(edges.size) { edges[it][2] }.also { it.sort() }
        // если даже с максимальным весом не получается - ответа нет
        if (!canAchieve(uniqueSortedWeights.last())) {
            return -1
        }
        // Бинарный поиск по ответу: минимальный вес при котором canAchieve == true
        var lowerBound = 0
        var upperBound = uniqueSortedWeights.size - 1
        while (lowerBound < upperBound) {
            val middleIndex = (lowerBound + upperBound) / 2
            if (canAchieve(uniqueSortedWeights[middleIndex])) {
                upperBound = middleIndex
            }
            else {
                lowerBound = middleIndex + 1
            }
        }
        return uniqueSortedWeights[lowerBound]
    }
}
