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
        for (i in 0 until countOfNeighbors){
            result.add(neighbors[vertex + i])
        }
        return result
    }

    override fun degree(vertex: Int): Int = neighbors[vertex + 1] - neighbors[vertex]

    override fun density(vertex: Int): Double = 2 * edgeCount.toDouble() /
            (vertexCount.toDouble() * (vertexCount.toDouble() - 1.0))

    override fun hasEdge(from: Int, to: Int): Boolean {
        val vertexWithFewerNeighbors = if(offsets[from + 1] - offsets[from]
            < offsets[to + 1] - offsets[to]) from else to
        val vertexWithMoreNeighbors = if(vertexWithFewerNeighbors == from) to else from
        val numberOfNeighbors = offsets[vertexWithFewerNeighbors + 1] - offsets[vertexWithFewerNeighbors]
        for(i in 0 until  numberOfNeighbors){
            if(neighbors[offsets[vertexWithFewerNeighbors + i]] == vertexWithMoreNeighbors) return true
        }
        return false
    }

    override fun vertices(): Array<Int> = Array<Int>(vertexCount){ i -> i }
}