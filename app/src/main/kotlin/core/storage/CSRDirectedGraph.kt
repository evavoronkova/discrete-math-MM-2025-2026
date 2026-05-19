package core.storage

import core.algoritms.quickSort
import core.model.DirectedGraph
import java.io.File
class CSRDirectedGraph(
    private val prevVertNumbers: IntArray,
    private val outOffs: IntArray,
    private val outNeighs: IntArray,
    private val inOffs: IntArray,
    private val inNeighs: IntArray
): DirectedGraph{
    val previousVertexNumbers = prevVertNumbers

    val outOffsets = outOffs

    val outNeighbors = outNeighs

    val inOffsets = inOffs

    val inNeighbors = inNeighs

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
        return inNeighbors.sliceArray(outOffsets[vertex] until outOffsets[vertex + 1]).toSet()
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

    override fun density(): Double = 2 * edgeCount.toDouble() /
            (vertexCount.toDouble() * (vertexCount.toDouble() - 1))

    override fun vertices(): IntArray = previousVertexNumbers.copyOf()

    fun markDeleted(vertices: Collection<Int>){
        for(vertex in vertices){
            if(vertex !in 0 until vertexCount){
                throw IndexOutOfBoundsException()
            }
            deleted[vertex] = true
        }
    }

    fun clearDeleted() = deleted.fill(false)

    fun isDeleted(vertex: Int): Boolean{
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        return deleted[vertex]
    }

    fun activeVertexCount(): Int = deleted.count{ !it }

    fun toUndirected(): CSRUndirectedGraph{
        val offsets = IntArray(vertexCount + 1){ i -> inOffsets[i] + outOffsets[i] }
        val neighbors = IntArray(edgeCount * 2)
        for(i in 1 .. vertexCount){
            val leftIndexOfIn = inOffsets[i - 1]
            val rightIndexOfIn = inOffsets[i]
            val leftIndexOfOut = outOffsets[i - 1]
            val rightIndexOfOut = outOffsets[i]
            val neighborsOfVertex = inNeighbors.sliceArray(leftIndexOfIn until rightIndexOfIn) +
                    outNeighbors.sliceArray(leftIndexOfOut until rightIndexOfOut)
            quickSort(neighborsOfVertex, 0, neighborsOfVertex.size - 1)
            neighborsOfVertex.copyInto(neighbors, destinationOffset = offsets[i - 1])
        }
        return CSRUndirectedGraph(previousVertexNumbers, offsets, neighbors)
    }
}
