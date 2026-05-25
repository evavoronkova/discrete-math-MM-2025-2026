package analysis

import java.util.Locale

data class ClusteringStats(
    val triangleCount: Long,
    val avgClusteringCoefficient: Double,
    val globalClusteringCoefficient: Double
) {
    override fun toString(): String {
        return """
Triangle count: $triangleCount
Average clustering coefficient: ${String.format(Locale.US, "%.6f", avgClusteringCoefficient)}
Global clustering coefficient: ${String.format(Locale.US, "%.6f", globalClusteringCoefficient)}
        """.trimIndent()
    }
}
