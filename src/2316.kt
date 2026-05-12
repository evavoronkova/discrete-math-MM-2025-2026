class Solution {
    fun countPairs(n: Int, edges: Array<IntArray>): Long {
        val parents = IntArray(n){ i -> i }
        var vert1 = 0
        var vert2 = 0
        for(edge in edges){
            vert1 = edge[0]
            vert2 = edge[1]
            union(parents, vert1, vert2)
        }
        val compSize = LongArray(n)
        val setOfRoots = mutableSetOf<Int>()
        for(i in 0 until n){
            val root = find(i, parents)
            compSize[root]++
            setOfRoots.add(root)
        }
        var result: Long = n * (n - 1L) / 2L
        for(root in setOfRoots){
            result -= compSize[root] * (compSize[root] - 1L) / 2L
        }

        return result
    }

    private fun find(vert: Int, parents: IntArray): Int {
        if (parents[vert] != vert)
            parents[vert] = find(parents[vert], parents)
        return parents[vert]
    }

    private fun union(parents: IntArray, vert1: Int, vert2: Int){
        if(parents[vert1] == vert2 || parents[vert2] == vert1) return
        parents[find(vert1, parents)] = find(vert2, parents)
    }
}
