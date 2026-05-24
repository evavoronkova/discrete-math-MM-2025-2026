package core.storage

import core.algorithms.quickSort
import core.model.DirectedGraph
import core.model.MutableVertexGraph

class CSRDirectedGraph(
    private val prevVertNumbers: IntArray,
    private val outOffs: IntArray,
    private val outNeighs: IntArray,
    private val inOffs: IntArray,
    private val inNeighs: IntArray
): DirectedGraph, MutableVertexGraph{
    val previousVertexNumbers = prevVertNumbers.copyOf()

    val outOffsets = outOffs.copyOf()

    val outNeighbors = outNeighs.copyOf()

    val inOffsets = inOffs.copyOf()

    val inNeighbors = inNeighs.copyOf()

    override val vertexCount: Int = inOffsets.size - 1

    override val edgeCount: Int = inNeighbors.size

    val deleted = BooleanArray(vertexCount){ false }

    override fun outNeighbors(vertex: Int): Set<Int>{
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(vertex)){
            throw IllegalStateException("Vertex is deleted")
        }
        return outNeighbors.sliceArray(outOffsets[vertex] until outOffsets[vertex + 1]).toSet()
    }

    override fun inNeighbors(vertex: Int): Set<Int> {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(vertex)){
            throw IllegalStateException("Vertex is deleted")
        }
        return inNeighbors.sliceArray(inOffsets[vertex] until inOffsets[vertex + 1]).toSet()
    }

    override fun neighbors(vertex: Int): Set<Int> {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(vertex)){
            throw IllegalStateException("Vertex is deleted")
        }
        return outNeighbors(vertex) + inNeighbors(vertex)
    }

    override fun outDegree(vertex: Int): Int {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(vertex)){
            throw IllegalStateException("Vertex is deleted")
        }
        return outOffsets[vertex + 1] - outOffsets[vertex]
    }

    override fun inDegree(vertex: Int): Int {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(vertex)){
            throw IllegalStateException("Vertex is deleted")
        }
        return inOffsets[vertex + 1] - inOffsets[vertex]
    }

    override fun degree(vertex: Int): Int {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(vertex)){
            throw IllegalStateException("Vertex is deleted")
        }
        return inDegree(vertex) + outDegree(vertex)
    }

    override fun hasEdge(from: Int, to: Int): Boolean {
        if(from !in 0 until vertexCount || to !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        if(isDeleted(to) || isDeleted(from)){
            throw IllegalStateException("Vertex is deleted")
        }

        return (outOffsets[from] until outOffsets[from + 1]).any{ outNeighbors[it] == to }
    }

    override fun density(): Double = if(vertexCount <= 1) 0.0
            else edgeCount.toDouble() / (vertexCount.toDouble() * (vertexCount.toDouble() - 1))

    override fun vertices(): IntArray = previousVertexNumbers.copyOf()

    override fun markDeleted(vertices: Collection<Int>){
        for(vertex in vertices){
            if(vertex !in 0 until vertexCount){
                throw IndexOutOfBoundsException()
            }
            deleted[vertex] = true
        }
    }

    override fun clearDeleted() = deleted.fill(false)

    override fun isDeleted(vertex: Int): Boolean{
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        return deleted[vertex]
    }

    override fun activeVertexCount(): Int = deleted.count{ !it }

    fun toUndirected(): CSRUndirectedGraph{
        val uniqueDegree = IntArray(vertexCount)
        for(i in 0 until vertexCount){
            val combined = inNeighbors.sliceArray(inOffsets[i] until inOffsets[i + 1]) +
                    outNeighbors.sliceArray(outOffsets[i] until outOffsets[i + 1])
            uniqueDegree[i] = combined.toSet().size
        }

        val offsets = IntArray(vertexCount + 1)
        for(i in 1 .. vertexCount){
            offsets[i] = offsets[i - 1] + uniqueDegree[i - 1]
        }

        val neighbors = IntArray(offsets[vertexCount])
        for(i in 0 until vertexCount){
            val combined = (inNeighbors.sliceArray(inOffsets[i] until inOffsets[i + 1]) +
                    outNeighbors.sliceArray(outOffsets[i] until outOffsets[i + 1])).toSet()
            val sorted = combined.toIntArray()
            quickSort(sorted, 0, sorted.size - 1)
            sorted.copyInto(neighbors, destinationOffset = offsets[i])
        }
        return CSRUndirectedGraph(previousVertexNumbers, offsets, neighbors)
    }
}
