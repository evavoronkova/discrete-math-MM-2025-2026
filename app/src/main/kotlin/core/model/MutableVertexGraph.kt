package core.model

interface MutableVertexGraph : Graph {
    fun markDeleted(vertices: List<Int>)

    fun clearDeleted()

    fun isDeleted(vertex: Int): Boolean

    fun activeVertexCount(): Int
}