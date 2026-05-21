class Solution {
    private var result = 0

    fun minReorder(n: Int, connections: Array<IntArray>): Int {
        val graph = Array(n){ mutableListOf<Int>() }
        for((a, b) in connections){
            graph[a].add((b shl 1) or 1)
            graph[b].add(a shl 1)
        }
        dfs(graph, 0, -1)
        return result
    }

    private fun dfs(graph: Array<MutableList<Int>>, root: Int, parent: Int){
        for(vert in graph[root]){
            if((vert shr 1) == parent) continue
            if((vert and 1) == 1) result++
            dfs(graph, vert shr 1, root)
        }
    }
}
