package analysis

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
        sb.appendLine("Average degree: ${"%.4f".format(avgDegree)}")
        sb.appendLine("Degree distribution (normal scale):")
        sb.appendLine("  deg -> count -> prob")
        val pd = probabilityDistribution()
        for ((deg, cnt) in degreeDistribution.entries.sortedBy { it.key }) {
            sb.appendLine("  $deg -> $cnt -> ${"%.6f".format(pd[deg]!!)}")
        }
        sb.appendLine("Degree distribution (log-log scale):")
        sb.appendLine("  log(deg) -> log(prob)")
        for ((deg, prob) in pd.entries.sortedBy { it.key }) {
            if (deg > 0 && prob > 0) {
                sb.appendLine("  ${"%.6f".format(kotlin.math.ln(deg.toDouble()))} -> ${"%.6f".format(kotlin.math.ln(prob))}")
            }
        }
        return sb.toString()
    }
}
