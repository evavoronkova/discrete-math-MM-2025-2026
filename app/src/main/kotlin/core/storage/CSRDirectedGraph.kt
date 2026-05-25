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
) : DirectedGraph, MutableVertexGraph {
    val previousVertexNumbers = prevVertNumbers.copyOf()

    val outOffsets = outOffs.copyOf()

    val outNeighborArray = outNeighs.copyOf()

    val inOffsets = inOffs.copyOf()

    val inNeighborArray = inNeighs.copyOf()

    override val vertexCount: Int = inOffsets.size - 1

    override val edgeCount: Int = inNeighborArray.size

    val deleted = BooleanArray(vertexCount) { false }

    override fun outNeighbors(vertex: Int): Sequence<Int> {
        if (vertex !in 0 until vertexCount) {
            throw IndexOutOfBoundsException()
        }
        if (isDeleted(vertex)) {
            throw IllegalStateException("Vertex is deleted")
        }
        val start = outOffsets[vertex]
        val end = outOffsets[vertex + 1]
        return (start until end).asSequence().map { outNeighs[it] }
    }

    override fun inNeighbors(vertex: Int): Sequence<Int> {
        if (vertex !in 0 until vertexCount) {
            throw IndexOutOfBoundsException()
        }
        if (isDeleted(vertex)) {
            throw IllegalStateException("Vertex is deleted")
        }
        val start = inOffsets[vertex]
        val end = inOffsets[vertex + 1]
        return (start until end).asSequence().map { inNeighs[it] }
    }

    override fun neighbors(vertex: Int): Sequence<Int> {
        if (vertex !in 0 until vertexCount) {
            throw IndexOutOfBoundsException()
        }
        if (isDeleted(vertex)) {
            throw IllegalStateException("Vertex is deleted")
        }
        return sequence {
            val outStart = outOffsets[vertex]
            val outEnd = outOffsets[vertex + 1]
            for (i in outStart until outEnd) yield(outNeighs[i])
            val inStart = inOffsets[vertex]
            val inEnd = inOffsets[vertex + 1]
            for (i in inStart until inEnd) yield(inNeighs[i])
        }
    }

    override fun outDegree(vertex: Int): Int {
        if (vertex !in 0 until vertexCount) {
            throw IndexOutOfBoundsException()
        }
        if (isDeleted(vertex)) {
            throw IllegalStateException("Vertex is deleted")
        }
        return outOffsets[vertex + 1] - outOffsets[vertex]
    }

    override fun inDegree(vertex: Int): Int {
        if (vertex !in 0 until vertexCount) {
            throw IndexOutOfBoundsException()
        }
        if (isDeleted(vertex)) {
            throw IllegalStateException("Vertex is deleted")
        }
        return inOffsets[vertex + 1] - inOffsets[vertex]
    }

    override fun degree(vertex: Int): Int {
        if (vertex !in 0 until vertexCount) {
            throw IndexOutOfBoundsException()
        }
        if (isDeleted(vertex)) {
            throw IllegalStateException("Vertex is deleted")
        }
        return inDegree(vertex) + outDegree(vertex)
    }

    override fun hasEdge(from: Int, to: Int): Boolean {
        if (from !in 0 until vertexCount || to !in 0 until vertexCount) {
            throw IndexOutOfBoundsException()
        }
        if (isDeleted(to) || isDeleted(from)) {
            throw IllegalStateException("Vertex is deleted")
        }

        return (outOffsets[from] until outOffsets[from + 1]).any { outNeighs[it] == to }
    }

    override fun density(): Double = if (vertexCount <= 1) 0.0
    else edgeCount.toDouble() / (vertexCount.toDouble() * (vertexCount.toDouble() - 1))

    override fun vertices(): Sequence<Int> = previousVertexNumbers.asSequence()

    override fun markDeleted(vertices: Collection<Int>) {
        for (vertex in vertices) {
            if (vertex !in 0 until vertexCount) {
                throw IndexOutOfBoundsException()
            }
            deleted[vertex] = true
        }
    }

    override fun clearDeleted() = deleted.fill(false)

    override fun isDeleted(vertex: Int): Boolean {
        if (vertex !in 0 until vertexCount) {
            throw IndexOutOfBoundsException()
        }
        return deleted[vertex]
    }

    override fun activeVertexCount(): Int = deleted.count { !it }

    fun toUndirected(): CSRUndirectedGraph {
        val uniqueDegree = IntArray(vertexCount)
        for (i in 0 until vertexCount) {
            val combined = inNeighs.sliceArray(inOffsets[i] until inOffsets[i + 1]) +
                    outNeighs.sliceArray(outOffsets[i] until outOffsets[i + 1])
            uniqueDegree[i] = combined.toSet().size
        }

        val offsets = IntArray(vertexCount + 1)
        for (i in 1..vertexCount) {
            offsets[i] = offsets[i - 1] + uniqueDegree[i - 1]
        }

        val neighbors = IntArray(offsets[vertexCount])
        for (i in 0 until vertexCount) {
            val combined = (inNeighs.sliceArray(inOffsets[i] until inOffsets[i + 1]) +
                    outNeighs.sliceArray(outOffsets[i] until outOffsets[i + 1])).toSet()
            val sorted = combined.toIntArray()
            quickSort(sorted, 0, sorted.size - 1)
            sorted.copyInto(neighbors, destinationOffset = offsets[i])
        }
        return CSRUndirectedGraph(previousVertexNumbers, offsets, neighbors)
    }
}
