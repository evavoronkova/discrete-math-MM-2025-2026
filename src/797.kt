class Solution {
    private val result = mutableListOf<List<Int>>()

    fun allPathsSourceTarget(graph: Array<IntArray>): List<List<Int>> {
        val n = graph.size
        val path = mutableListOf(0)
        dfs(graph, 0, n - 1, path)
        return result
    }


    private fun dfs(graph: Array<IntArray>, from: Int, to: Int, path: MutableList<Int>){
        if(from == to){
            result.add(path.toList())
            return
        }
        for(vert in graph[from]){
            path.add(vert)
            dfs(graph, vert, to, path)
            path.removeAt(path.size - 1)
        }
    }
}
