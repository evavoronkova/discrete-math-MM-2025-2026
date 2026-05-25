package benchmark

import java.util.Locale

data class DistanceEstimatorQuality(
    val avgError: Double,
    val maxError: Int,
    val avgRelativeError: Double,
    val exactMatches: Int,
    val totalQueries: Int,
    val exactMatchRate: Double
) {
    override fun toString(): String {
        return """
            --- Result tests ---
            Total requests: $totalQueries
            Exact matches: $exactMatches (${String.format(Locale.US, "%.2f", exactMatchRate * 100)}%)
            Average error (edges): ${String.format(Locale.US, "%.3f", avgError)}
            Maximum error (edges): $maxError
            Average relative error: ${String.format(Locale.US, "%.2f", avgRelativeError * 100)}%
            -------------------------------
        """.trimIndent()
    }
}
