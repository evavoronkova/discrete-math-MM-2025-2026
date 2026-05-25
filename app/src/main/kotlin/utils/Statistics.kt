package utils

object Statistics {

    fun mean(values: List<Double>): Double =
        if (values.isEmpty()) 0.0 else values.sum() / values.size

    fun median(values: List<Double>): Double {
        if (values.isEmpty()) return 0.0
        val sorted = values.sorted()
        val mid = sorted.size / 2
        return if (sorted.size % 2 == 0) {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }

    fun percentile(values: List<Double>, p: Double): Double {
        if (values.isEmpty()) return 0.0
        val sorted = values.sorted()
        val index = (p / 100.0 * (sorted.size - 1)).toInt()
            .coerceIn(0, sorted.size - 1)
        return sorted[index]
    }

    fun standardDeviation(values: List<Double>): Double {
        val n = values.size
        if (n == 0) return 0.0
        val avg = mean(values)
        val variance = values.sumOf { (it - avg) * (it - avg) } / n
        return kotlin.math.sqrt(variance)
    }
}
