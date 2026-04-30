package core.model

interface DirectedGraph {
    fun outNeighbors(vertex: Int): Set<Int>

    fun inNeighbors(vertex: Int): Set<Int>

    fun outDegree(vertex: Int): Int

    fun inDegree(vertex: Int): Int
}