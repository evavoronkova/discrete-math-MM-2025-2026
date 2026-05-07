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
        val numberOfVertices = vertices.size
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

    fun fromFileToCSRUndirectedGraph(filename: String): CSRUndirectedGraph{
        val vertexSet = mutableSetOf<Int>()
        File(filename).useLines { lines ->
            lines.forEach { line ->
                if(line.isBlank()) throw IllegalStateException("Line $line is empty")
                val parts = line.trim().split(' ')
                if(parts.size != 2) throw IllegalStateException("Invalid line: $line")
                val u = parts[0].toIntOrNull() ?: throw IllegalStateException("Invalid line: $line")
                val v = parts[1].toIntOrNull() ?: throw IllegalStateException("Invalid line: $line")
                vertexSet.add(u); vertexSet.add(v)
            }
        }
        val vertexCount = vertexSet.size
        val previousVertexArray = vertexSet.toIntArray()
        quickSort(previousVertexArray, 0, vertexCount - 1)
        val vertexToIndexMap = mutableMapOf<Int, Int>()
        for(i in 0 until vertexCount){
            vertexToIndexMap[previousVertexArray[i]] = i
        }

        var edgeCount = 0
        val vertexDegreeArray = IntArray(vertexCount)
        File(filename).useLines { lines ->
            lines.forEach { line ->
                val (u, v) = line.trim().split(' ').map{ it.toInt() }
                edgeCount++
                vertexDegreeArray[vertexToIndexMap[u]!!]++
                vertexDegreeArray[vertexToIndexMap[v]!!]++
            }
        }
        val offs = IntArray(vertexCount + 1)
        for(i in 1..vertexCount){
            offs[i] = offs[i - 1] + vertexDegreeArray[i - 1]
        }

        val neighs = IntArray(edgeCount * 2)
        val currentPosition = IntArray(vertexCount)
        File(filename).useLines { lines ->
            lines.forEach { line ->
                val (u, v) = line.trim().split(' ').map { it.toInt() }
                val indexOfFirstVert = vertexToIndexMap[u]!!
                val indexOfSecondVert = vertexToIndexMap[v]!!
                neighs[offs[indexOfFirstVert] + currentPosition[indexOfFirstVert]] = indexOfSecondVert
                currentPosition[indexOfFirstVert]++
                neighs[offs[indexOfSecondVert] + currentPosition[indexOfSecondVert]] = indexOfFirstVert
                currentPosition[indexOfSecondVert]++
            }
        }
        return CSRUndirectedGraph(previousVertexArray, offs, neighs)
    }
}

