package core.model

interface Graph{
    val vertexCount: Int

    val edgeCount: Int

    fun neighbors(vertex: Int): Sequence<Int>

    fun degree(vertex: Int): Int

    fun density(): Double

    fun vertices(): Sequence<Int>

    fun hasEdge(from: Int, to: Int): Boolean
}