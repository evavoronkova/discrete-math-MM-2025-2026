class Solution {
    fun findSmallestSetOfVertices(n: Int, edges: List<List<Int>>): List<Int> {
        val inDegrees = IntArray(n)
        for (edge in edges) {
            inDegrees[edge[1]]++
        }

        val result = ArrayList<Int>()
        for (vertex in 0 until n) {
            if (inDegrees[vertex] == 0) {
                result.add(vertex)
            }
        }

        return result
    }
}
