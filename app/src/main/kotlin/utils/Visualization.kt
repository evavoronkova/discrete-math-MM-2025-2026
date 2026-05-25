package utils

import analysis.DegreeStats
import benchmark.DistanceEstimatorQuality
import experiments.DeletionExperimentResult
import java.io.File

object Visualization {

    fun exportDegreeDistribution(stats: DegreeStats, filename: String) {
        val pd = stats.probabilityDistribution()
        val sb = StringBuilder()
        sb.appendLine("degree,count,probability")
        for ((deg, cnt) in stats.degreeDistribution.entries.sortedBy { it.key }) {
            sb.appendLine("$deg,$cnt,${pd[deg]}")
        }
        File(filename).writeText(sb.toString())
    }

    fun exportLogLogDegree(stats: DegreeStats, filename: String) {
        val pd = stats.probabilityDistribution()
        val sb = StringBuilder()
        sb.appendLine("log_degree,log_probability")
        for ((deg, prob) in pd.entries.sortedBy { it.key }) {
            if (deg > 0 && prob > 0) {
                sb.appendLine("${kotlin.math.ln(deg.toDouble())},${kotlin.math.ln(prob)}")
            }
        }
        File(filename).writeText(sb.toString())
    }

    fun exportDeletionResults(results: List<DeletionExperimentResult>, filename: String) {
        val sb = StringBuilder()
        sb.appendLine("percent_deleted,num_components,largest_component_size,largest_component_fraction")
        for (r in results) {
            sb.appendLine("${r.percentDeleted},${r.numComponents},${r.largestComponentSize},${r.largestComponentFraction}")
        }
        File(filename).writeText(sb.toString())
    }

    fun exportDistanceQuality(quality: DistanceEstimatorQuality, filename: String) {
        val sb = StringBuilder()
        sb.appendLine("avg_error,max_error,avg_relative_error,exact_matches,total_queries,exact_match_rate")
        sb.appendLine("${quality.avgError},${quality.maxError},${quality.avgRelativeError},${quality.exactMatches},${quality.totalQueries},${quality.exactMatchRate}")
        File(filename).writeText(sb.toString())
    }
}
