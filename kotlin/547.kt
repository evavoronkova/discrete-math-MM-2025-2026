class Solution {
    fun findCircleNum(isConnected: Array<IntArray>): Int {
        val n = isConnected.size
        val visited = BooleanArray(n)
        var provinces = 0
        fun dfs(city: Int) {
            visited[city] = true
            for (i in 0 until n) {
                if (isConnected[city][i] == 1 && !visited[i]) {
                    dfs(i)
                }
            }
        }
        for (i in 0 until n) {
            if (!visited[i]) {
                provinces++
                dfs(i)
            }
        }

        return provinces
    }
}