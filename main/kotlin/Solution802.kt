class Solution {
    fun eventualSafeNodes(graph: Array<IntArray>): List<Int> {
        val n = graph.size
        val visit = BooleanArray(n)
        val inStack = BooleanArray(n)
        for (i in 0..<n) {
            dfs(i, graph, visit, inStack)
        }
        val safeNodes = ArrayList<Int>()
        for (i in 0..<n) {
            if (!inStack[i]) {
                safeNodes.add(i)
            }
        }
        return safeNodes
    }

    fun dfs(node: Int, adj: Array<IntArray>, visit: BooleanArray, inStack: BooleanArray): Boolean {
        if (inStack[node]) {
            return true
        }
        if (visit[node]) {
            return false
        }
        visit[node] = true
        inStack[node] = true
        for (neighbor in adj[node]) {
            if (dfs(neighbor, adj, visit, inStack)) {
                return true
            }
        }
        inStack[node] = false
        return false
    }
}