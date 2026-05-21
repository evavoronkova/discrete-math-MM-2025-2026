class Solution {
    fun minimumDiameterAfterMerge(edges1: Array<IntArray>, edges2: Array<IntArray>): Int {
        val n1 = edges1.size + 1
        val n2 = edges2.size + 1

        val graph1 = Array(n1){ mutableListOf<Int>() }
        val graph2 = Array(n2){ mutableListOf<Int>() }

        for (edge in edges1){
            graph1[edge[0]].add(edge[1])
            graph1[edge[1]].add(edge[0])
        }

        for(edge in edges2){
            graph2[edge[0]].add(edge[1])
            graph2[edge[1]].add(edge[0])
        }

        val (__1, lastVert1) = bfs(graph1, 0, n1)
        val (diam1, _1) = bfs(graph1, lastVert1, n1)
        val rad1 = (diam1 + 1) / 2

        val (__2, lastVert2) = bfs(graph2, 0, n2)
        val(diam2, _2) = bfs(graph2, lastVert2, n2)
        val rad2 = (diam2 + 1) / 2

        return max(diam1, diam2, rad1 + rad2 + 1)
    }

    private fun bfs(graph: Array<MutableList<Int>>, vert: Int, n: Int): Pair<Int, Int>{
        if(n == 0) return Pair(0, 0)
        val visited = BooleanArray(n) { false }
        val queue = ArrayDeque<Int>()
        val length = IntArray(n) { 0 }
        queue.addLast(vert)
        var lastVert = vert
        while(queue.isNotEmpty()){
            val v = queue.removeFirst()
            visited[v] = true
            for(u in graph[v]){
                if(!visited[u]){
                    length[u] = length[v] + 1
                    queue.addLast(u)
                }
            }
            lastVert = v
        }
        return Pair(length[lastVert], lastVert)
    }

    private fun max(n1: Int, n2: Int, n3: Int): Int{
        if(n1 > n2 && n1 > n3) return n1
        else if(n2 > n1 && n2 > n3) return n2
        else return n3
    }
}
