package core

interface Graph{
    val vertexCount: Int

    val edgeCount: Int

    fun neighbors(vertex: Int): Set<Int>

    fun degree(vertex: Int): Int

    fun density(vertex: Int): Double

    fun vertices(): Collection<Int>

    fun hasEdge(from: Int, to: Int): Boolean
}