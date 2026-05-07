package core.model

interface Graph{
    val vertexCount: Int

    val edgeCount: Int

    fun neighbors(vertex: Int): Set<Int>

    fun degree(vertex: Int): Int

    fun density(): Double

    fun vertices(): IntArray

    fun hasEdge(from: Int, to: Int): Boolean
}