package analysis

data class DiameterStats(
    val doubleSweepDiameter: Int,
    val doubleSweepSource: Int,
    val doubleSweepTarget: Int,
    val randomPairsDiameter: Int,
    val randomPairs90Percentile: Int,
    val snowballDiameter: Int,
    val snowball90Percentile: Int
) {
    override fun toString(): String {
        return """
Double sweep diameter: $doubleSweepDiameter (source=$doubleSweepSource, target=$doubleSweepTarget)
Random pairs 90th percentile: $randomPairs90Percentile, diameter: $randomPairsDiameter
Snowball 90th percentile: $snowball90Percentile, diameter: $snowballDiameter
        """.trimIndent()
    }
}
