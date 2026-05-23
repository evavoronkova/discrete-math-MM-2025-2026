class Solution {
    fun remainingMethods(n: Int, k: Int, invocations: Array<IntArray>): IntArray {
        // head[v] = индекс последнего ребра из v, next[e] = индекс предыдущего
        val edgeCount = invocations.size
        val head = IntArray(n) { -1 }
        val to = IntArray(edgeCount)
        val next = IntArray(edgeCount)
        for (i in 0 until edgeCount) {
            val a = invocations[i][0]
            val b = invocations[i][1]
            to[i] = b
            next[i] = head[a]
            head[a] = i
        }

        // DFS на стеке: помечаем все вершины достижимые из k
        val suspicious = BooleanArray(n)
        val stack = IntArray(n)
        var stackTop = 0
        suspicious[k] = true
        stack[stackTop] = k
        stackTop++
        while (stackTop > 0) {
            stackTop--
            val currentVertex = stack[stackTop]
            var edgeIndex = head[currentVertex]
            // идём по связному списку рёбер из currentVertex
            while (edgeIndex != -1) {
                val neighborNode = to[edgeIndex]
                if (!suspicious[neighborNode]) {
                    suspicious[neighborNode] = true
                    stack[stackTop] = neighborNode
                    stackTop++
                }
                edgeIndex = next[edgeIndex]
            }
        }

        // Если есть ребро не подозрительный -> подозрительный, удалить нельзя, возвращаем все методы
        for (i in 0 until edgeCount) {
            val a = invocations[i][0]
            val b = invocations[i][1]
            if (!suspicious[a] && suspicious[b]) {
                return IntArray(n) { it }
            }
        }
        // Иначе возвращаем только не подозрительные методы
        val result = IntArray(n)
        var resultIndex = 0
        for (i in 0 until n) {
            if (!suspicious[i]) {
                result[resultIndex] = i
                resultIndex++
            }
        }
        return result.copyOf(resultIndex)
    }
}
