package analysis

import java.util.Locale

data class DegreeStats(
    val minDegree: Int,
    val maxDegree: Int,
    val avgDegree: Double,
    val degreeDistribution: Map<Int, Int>
) {
    fun probabilityDistribution(): Map<Int, Double> {
        val total = degreeDistribution.values.sum().toDouble()
        return degreeDistribution.mapValues { it.value / total }
    }

    override fun toString(): String {
        val sb = StringBuilder()
        sb.appendLine("Min degree: $minDegree")
        sb.appendLine("Max degree: $maxDegree")
        sb.appendLine("Average degree: ${String.format(Locale.US, "%.4f", avgDegree)}")
        sb.appendLine("Degree distribution (normal scale):")
        sb.appendLine("  deg -> count -> prob")
        val pd = probabilityDistribution()
        for ((deg, cnt) in degreeDistribution.entries.sortedBy { it.key }) {
            sb.appendLine("  $deg -> $cnt -> ${String.format(Locale.US, "%.6f", pd[deg]!!)}")
        }
        sb.appendLine("Degree distribution (log-log scale):")
        sb.appendLine("  log(deg) -> log(prob)")
        for ((deg, prob) in pd.entries.sortedBy { it.key }) {
            if (deg > 0 && prob > 0) {
                sb.appendLine("  ${String.format(Locale.US, "%.6f", kotlin.math.ln(deg.toDouble()))} -> ${String.format(Locale.US, "%.6f", kotlin.math.ln(prob))}")
            }
        }
        return sb.toString()
    }
}
