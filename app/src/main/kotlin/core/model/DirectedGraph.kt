package core.model

interface DirectedGraph: Graph {
    fun outNeighbors(vertex: Int): Sequence<Int>

    fun inNeighbors(vertex: Int): Sequence<Int>

    fun outDegree(vertex: Int): Int

    fun inDegree(vertex: Int): Int
}