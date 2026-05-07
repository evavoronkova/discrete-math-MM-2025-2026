package core.storage

import core.model.Graph
import core.model.DirectedGraph
import java.time.ZoneOffset

class CSRDirectedGraph(
    private val prevVertNumbers: IntArray,
    private val outOffs: IntArray,
    private val outNeighs: IntArray,
    private val inOffs: IntArray,
    private val inNeighs: IntArray
): Graph, DirectedGraph{
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

    

    fun isDeleted(vertex: Int): Boolean{

    }
}