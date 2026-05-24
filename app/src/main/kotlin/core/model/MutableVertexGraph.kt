package core.model

interface MutableVertexGraph : Graph {
    fun markDeleted(vertices: Collection<Int>)

    fun clearDeleted()

    fun isDeleted(vertex: Int): Boolean

    fun activeVertexCount(): Int
}
