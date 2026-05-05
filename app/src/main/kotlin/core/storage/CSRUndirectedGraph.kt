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
        val result: MutableSet<Int> = mutableSetOf()
        val countOfNeighbors = offsets[vertex + 1] - offsets[vertex]
        for (i in 0 until countOfNeighbors){
            result.add(neighbors[vertex + i])
        }
        return result
    }

    override fun degree(vertex: Int): Int {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        return neighbors[vertex + 1] - neighbors[vertex]
    }

    override fun density(vertex: Int): Double {
        if(vertex !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }
        return 2 * edgeCount.toDouble() /
                (vertexCount.toDouble() * (vertexCount.toDouble() - 1.0))
    }

    override fun hasEdge(from: Int, to: Int): Boolean {
        if(from !in 0 until vertexCount || to !in 0 until vertexCount){
            throw IndexOutOfBoundsException()
        }

        val vertexWithFewerNeighbors = if(offsets[from + 1] - offsets[from]
            < offsets[to + 1] - offsets[to]) from else to
        val vertexWithMoreNeighbors = if(vertexWithFewerNeighbors == from) to else from
        val numberOfNeighbors = offsets[vertexWithFewerNeighbors + 1] - offsets[vertexWithFewerNeighbors]
        for(i in 0 until  numberOfNeighbors){
            if(neighbors[offsets[vertexWithFewerNeighbors] + i] == vertexWithMoreNeighbors) return true
        }
        return false
    }

    override fun vertices(): Array<Int> = Array(vertexCount){ i -> i }

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

    fun activeVertexCount(): Int = deleted.count(){ it }

    fun fromEdges(edges: List<Pair<Int, Int>>): CSRUndirectedGraph{
        val previousVertexSet = hashSetOf<Int>()
        for(edge in edges){
            previousVertexSet.add(edge.first)
            previousVertexSet.add(edge.second)
        }
        val vertexCount = previousVertexSet.size
        val previousVertexArray = previousVertexSet.toIntArray()
        quickSort(previousVertexArray, 0, vertexCount - 1)
        val vertexToIndexMap = mutableMapOf<Int, Int>()
        for(i in 0 until vertexCount){
            vertexToIndexMap[previousVertexArray[i]] = i
        }
        val degreeOfVertexArray = IntArray(vertexCount)
        for(edge in edges){
            degreeOfVertexArray[vertexToIndexMap[edge.first]!!]++
            degreeOfVertexArray[vertexToIndexMap[edge.second]!!]++
        }
        val offs = IntArray(vertexCount + 1)
        offs[0] = 0
        for(i in 1 .. vertexCount){
            offs[i] = offs[i - 1] + degreeOfVertexArray[i - 1]
        }
        val edgeCount = edges.size
        val neighs = IntArray(2 * edgeCount)
        val currentPositionOfNeighs = IntArray(vertexCount)
        var indexOfFirstVert = 0
        var indexOfSecondVert = 0
        for(edge in edges){
            indexOfFirstVert = vertexToIndexMap[edge.first]!!
            indexOfSecondVert = vertexToIndexMap[edge.second]!!
            neighs[offs[indexOfFirstVert] + currentPositionOfNeighs[indexOfFirstVert]] = indexOfSecondVert
            currentPositionOfNeighs[indexOfFirstVert]++
            neighs[offs[indexOfSecondVert] + currentPositionOfNeighs[indexOfSecondVert]] = indexOfFirstVert
            currentPositionOfNeighs[indexOfSecondVert]++
        }
        return CSRUndirectedGraph(previousVertexArray, offs, neighs)
    }

    fun fromFileToListOfEdges(filename: String): List<Pair<Int, Int>> =
        File(filename).useLines { lines ->
            lines.mapNotNull { line ->
                val parts = line.split(' ')
                if (parts.size == 2) {
                    val u = parts[0].toIntOrNull()
                    val w = parts[1].toIntOrNull()
                    if (u != null && w != null) u to w
                    else null
                }else null
            }.toList()
        }


    fun fromFileToCSRUndirectedGraph(filename: String): CSRUndirectedGraph = fromEdges(fromFileToListOfEdges(filename))

}