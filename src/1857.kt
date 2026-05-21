class Solution {
    fun largestPathValue(colors: String, edges: Array<IntArray>): Int {
        val n = colors.length
        val graph = Array(n){ mutableListOf<Int>() }
        val dp = Array(n){ IntArray(26) }
        for(i in 0 until n){
            dp[i][colors[i].code - 'a'.code] = 1
        }
        val inDegreeArray = IntArray(n)
        for(edge in edges){
            val v = edge[0]
            val u = edge[1]
            graph[v].add(u)
            inDegreeArray[u]++
        }
        var visited = 0
        val queue = ArrayDeque<Int>()
        for(i in 0 until n){
            if(inDegreeArray[i] == 0) queue.addLast(i)
        }

        while(queue.isNotEmpty()){
            val v = queue.removeFirst()
            visited++
            for(u in graph[v]){
                for(color in 0 until 26){
                    val oneOrNull = if(colors[u].code - 'a'.code == color) 1 else 0
                    dp[u][color] = if(dp[u][color] < dp[v][color] + oneOrNull) dp[v][color] + oneOrNull else dp[u][color]
                }
                inDegreeArray[u]--
                if(inDegreeArray[u] == 0) queue.add(u)
            }
        }
        if(visited != n) return -1
        var result = dp[0][0]
        for(i in 0 until n){
            if(dp[i][colors[i].code - 'a'.code] > result) result = dp[i][colors[i].code - 'a'.code]
        }
        return result
    }
}
