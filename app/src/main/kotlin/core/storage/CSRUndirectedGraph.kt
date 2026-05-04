package core.storage

import core.model.Graph

class CSRUndirectedGraph(
    private val offs: IntArray,
    private val neigs: IntArray
): Graph{
    val offsets = offs

    val neighbors = neigs

    override val vertexCount = offsets.size - 1

    override val edgeCount = neighbors.size / 2

    override fun neighbors(vertex: Int): Set<Int> {
        val result: MutableSet<Int> = mutableSetOf()
        val countOfNeighbors = offsets[vertex + 1] - offsets[vertex]
        for (i in 0..countOfNeighbors){
            result.add(neighbors[vertex + i])
        }
        return result
    }

    override fun degree(vertex: Int): Int = neighbors[vertex + 1] - neighbors[vertex]


}