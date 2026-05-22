class Solution {
    fun longestIncreasingPath(matrix: Array<IntArray>): Int {
        val m = matrix.size
        val n = matrix[0].size
        val dp = Array(m) { IntArray(n) }
        val directions = arrayOf(0 to 1, 0 to -1, 1 to 0, -1 to 0)

        fun dfs(r: Int, c: Int): Int {
            if (dp[r][c] != 0) return dp[r][c]

            var maxLen = 1
            for ((dr, dc) in directions) {
                val nr = r + dr
                val nc = c + dc
                if (nr in 0..<m && nc in 0..<n && matrix[nr][nc] > matrix[r][c]) {
                    maxLen = maxOf(maxLen, 1 + dfs(nr, nc))
                }
            }
            dp[r][c] = maxLen
            return maxLen
        }
        var result = 0
        for (i in 0..<m) {
            for (j in 0..<n) {
                result = maxOf(result, dfs(i, j))
            }
        }
        return result
    }
}