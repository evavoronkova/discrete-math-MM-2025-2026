package analysis

import java.util.Locale

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
        sb.appendLine("Density: ${String.format(Locale.US, "%.8f", density)}")
        sb.appendLine("Weak components: $weakComponents")
        sb.appendLine("Largest weak component fraction: ${String.format(Locale.US, "%.4f", weakLargestFraction)}")
        if (strongComponents != null) {
            sb.appendLine("Strong components: $strongComponents")
            sb.appendLine("Largest strong component fraction: ${String.format(Locale.US, "%.4f", strongLargestFraction)}")
        }
        return sb.toString()
    }
}
