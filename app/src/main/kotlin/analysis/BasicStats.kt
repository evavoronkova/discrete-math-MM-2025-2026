package analysis

data class BasicStats(
    val vertexCount: Int,
    val edgeCount: Int,
    val density: Double,
    val weakComponents: Int,
    val weakLargestFraction: Double,
    val strongComponents: Int? = null,
    val strongLargestFraction: Double? = null
) {
    override fun toString(): String {
        val sb = StringBuilder()
        sb.appendLine("Vertices: $vertexCount")
        sb.appendLine("Edges: $edgeCount")
        sb.appendLine("Density: ${"%.8f".format(density)}")
        sb.appendLine("Weak components: $weakComponents")
        sb.appendLine("Largest weak component fraction: ${"%.4f".format(weakLargestFraction)}")
        if (strongComponents != null) {
            sb.appendLine("Strong components: $strongComponents")
            sb.appendLine("Largest strong component fraction: ${"%.4f".format(strongLargestFraction)}")
        }
        return sb.toString()
    }
}
