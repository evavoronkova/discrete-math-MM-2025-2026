package core.storage

import core.model.Graph
import core.algoritms.quickSort
import java.io.File

class CSRUndirectedGraph(
    private val prevVertNumbers: IntArray,
    private val offs: IntArray,
    private val neighs: IntArray
): Graph{
    val previousVertexNumbers = prevVertNumbers

    val offsets = offs

    val neighbors = neighs

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

    override fun density(): Double = 2 * edgeCount.toDouble() /
                (vertexCount.toDouble() * (vertexCount.toDouble() - 1.0))

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

    fun markDeleted(vertices: Collection<Int>){
        for (vertex in vertices){
            if (vertex !in 0 until vertexCount){
                throw IndexOutOfBoundsException()
            }
            deleted[vertex] = true
        }
    }

    fun clearDeleted() = deleted.fill(false)

    fun isDeleted(vertex: Int): Boolean{
        if (vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        return deleted[vertex]
    }

    fun activeVertexCount(): Int = deleted.count(){ !it }
}
