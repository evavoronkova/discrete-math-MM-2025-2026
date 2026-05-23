class Solution {
    fun findCircleNum(isConnected: Array<IntArray>): Int {
        val n = isConnected.size
        val visited = BooleanArray(n)
        var count = 0

        fun dfs(v: Int) {
            visited[v] = true
            for (u in 0 until n) {
                if (isConnected[v][u] == 1 && !visited[u]) {
                    dfs(u)
                }
            }
        }

        for (i in 0 until n) {
            if (!visited[i]) {
                count++
                dfs(i)
            }
        }
        return count
    }
}