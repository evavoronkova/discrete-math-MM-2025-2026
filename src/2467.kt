class Solution {
    fun mostProfitablePath(edges: Array<IntArray>, bob: Int, amount: IntArray): Int {
        val n = edges.size + 1
        val graph = Array(n) { mutableListOf<Int>() }
        for(edge in edges){
            graph[edge[0]].add(edge[1])
            graph[edge[1]].add(edge[0])
        }
        val bobPath = bfs(n, graph, bob, 0)
        val bobTime = IntArray(n) { -1 }
        var currentTime = 0
        for(v in bobPath){
            bobTime[v] = currentTime
            currentTime++
        }

        var bestProfit = Int.MIN_VALUE

        val stack = ArrayDeque<IntArray>()
        val visited = BooleanArray(n)
        visited[0] = true
        stack.addFirst(intArrayOf(0, 0))
        val isLeaf = BooleanArray(n)
        val profits = IntArray(n)
        profits[0] = amount[0]

        while(stack.isNotEmpty()){
            val arr = stack.removeLast()
            val v = arr[0]
            val timeV = arr[1]
            if(graph[v].size == 1 && v != 0){
                isLeaf[v] = true
            }
            for(ney in graph[v].reversed()){
                if(visited[ney]) continue
                val timeNey = timeV + 1
                stack.addLast(intArrayOf(ney, timeNey))
                visited[ney] = true
                val gain = when {
                    bobTime[ney] == -1 -> amount[ney]
                    timeNey == bobTime[ney] -> amount[ney] / 2
                    timeNey < bobTime[ney] -> amount[ney]
                    else -> 0
                }
                profits[ney] = profits[v] + gain
            }
        }

        for(i in 0 until n){
            if(isLeaf[i]){
                bestProfit = if(bestProfit < profits[i]) profits[i] else bestProfit
            }
        }

        return bestProfit
    }

    private fun bfs(n: Int, graph: Array<MutableList<Int>>, from: Int, to: Int): ArrayDeque<Int>{
        val result = ArrayDeque<Int>()
        val queue = ArrayDeque<Int>()
        queue.addLast(from)
        val parents = IntArray(n) { -1 }
        while(queue.isNotEmpty()){
            val vert = queue.removeFirst()
            for(vert1 in graph[vert]){
                if(vert1 == parents[vert]) continue
                parents[vert1] = vert
                if(vert1 == to){
                    var current = to
                    while(current != from){
                        result.addFirst(current)
                        current = parents[current]
                    }
                    result.addFirst(from)
                    return result
                }
                queue.addLast(vert1)
            }
        }
        return result
    }
}
