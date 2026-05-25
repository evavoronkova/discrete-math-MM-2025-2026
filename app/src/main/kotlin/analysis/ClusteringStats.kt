package analysis

data class ClusteringStats(
    val triangleCount: Long,
    val avgClusteringCoefficient: Double,
    val globalClusteringCoefficient: Double
) {
    override fun toString(): String {
        return """
Triangle count: $triangleCount
Average clustering coefficient: ${"%.6f".format(avgClusteringCoefficient)}
Global clustering coefficient: ${"%.6f".format(globalClusteringCoefficient)}
        """.trimIndent()
    }
}
