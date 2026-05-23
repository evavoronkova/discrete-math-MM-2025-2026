class Solution {
    fun findSmallestSetOfVertices(n: Int, edges: List<List<Int>>): List<Int> {
        // В DAG минимальное множество стартов = все вершины с in-degree 0
        // (с in-degree > 0 достижима от предка, с in-degree = 0 ниоткуда не достижима, значит обязательна)
        val inDegrees = IntArray(n)
        for (edge in edges) {
            inDegrees[edge[1]]++
        }

        // Собираем все вершины с нулевой входящей степенью
        val result = ArrayList<Int>()
        for (vertex in 0 until n) {
            if (inDegrees[vertex] == 0) {
                result.add(vertex)
            }
        }

        return result
    }
}
