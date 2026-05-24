package core.storage

import core.model.MutableVertexGraph

class CSRUndirectedGraph(
    private val prevVertNumbers: IntArray,
    private val offs: IntArray,
    private val neighs: IntArray
): MutableVertexGraph{
    val previousVertexNumbers = prevVertNumbers.copyOf()

    val offsets = offs.copyOf()

    val neighbors = neighs.copyOf()

    override val vertexCount = offsets.size - 1

    override val edgeCount = neighbors.size / 2

    val deleted: BooleanArray = BooleanArray(vertexCount){ false }

    override fun neighbors(vertex: Int): Set<Int> {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(vertex)){
            throw IllegalStateException("Vertex is deleted")
        }
        return neighbors.sliceArray(offsets[vertex] until offsets[vertex + 1]).toSet()
    }

    override fun degree(vertex: Int): Int {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(vertex)){
            throw IllegalStateException("Vertex is deleted")
        }
        return offsets[vertex + 1] - offsets[vertex]
    }

    override fun density(): Double = if(vertexCount <= 1) 0.0
            else 2 * edgeCount.toDouble() / (vertexCount.toDouble() * (vertexCount.toDouble() - 1))

    override fun hasEdge(from: Int, to: Int): Boolean {
        if(from !in 0 until vertexCount || to !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(from) || isDeleted(to)){
            throw IllegalStateException("Vertex is deleted")
        }

        val vertexWithFewerNeighbors = if(offsets[from + 1] - offsets[from]
            < offsets[to + 1] - offsets[to]) from else to
        val vertexWithMoreNeighbors = if(vertexWithFewerNeighbors == from) to else from
        return (offsets[vertexWithFewerNeighbors] until offsets[vertexWithFewerNeighbors + 1]).any{ neighbors[it] == vertexWithMoreNeighbors }
    }

    override fun vertices(): IntArray = previousVertexNumbers.copyOf()

    override fun markDeleted(vertices: Collection<Int>){
        for (vertex in vertices){
            if (vertex !in 0 until vertexCount){
                throw IndexOutOfBoundsException()
            }
            deleted[vertex] = true
        }
    }

    override fun clearDeleted() = deleted.fill(false)

    override fun isDeleted(vertex: Int): Boolean{
        if (vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        return deleted[vertex]
    }

    override fun activeVertexCount(): Int = deleted.count{ !it }
}
