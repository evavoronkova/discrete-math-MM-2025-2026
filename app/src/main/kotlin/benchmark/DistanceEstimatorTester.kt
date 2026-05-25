package benchmark

import core.model.Graph
import landmarks.DistanceEstimator
import kotlin.math.abs

class DistanceEstimatorTester(
    private val graph: Graph,
    private val estimator: DistanceEstimator
) {

    fun test(numPairs: Int): DistanceEstimatorQuality {
        val pairs = mutableListOf<Pair<Int, Int>>()
        val vertexCount = graph.vertexCount

        repeat(numPairs) {
            val u = (0 until vertexCount).random()
            val v = (0 until vertexCount).random()
            pairs.add(Pair(u, v))
        }

        return testWithPairs(pairs)
    }

    fun testWithPairs(pairs: List<Pair<Int, Int>>): DistanceEstimatorQuality {
        var totalError = 0.0
        var totalRelativeError = 0.0
        var maxError = 0
        var exactMatches = 0
        var validQueries = 0

        for ((u, v) in pairs) {
            if (u == v) continue

            val exactDist = estimator.exactDistance(u, v)
            
            val estimatedDist = estimator.estimateDistance(u, v)

            if (exactDist != -1 && estimatedDist != -1) {
                validQueries++
                
                val error = abs(estimatedDist - exactDist)
                val relativeError = error.toDouble() / exactDist.toDouble()

                totalError += error
                totalRelativeError += relativeError
                
                if (error > maxError) maxError = error
                if (error == 0) exactMatches++
            }
        }

        if (validQueries == 0) {
            return DistanceEstimatorQuality(0.0, 0, 0.0, 0, 0, 0.0)
        }

        return DistanceEstimatorQuality(
            avgError = totalError / validQueries,
            maxError = maxError,
            avgRelativeError = totalRelativeError / validQueries,
            exactMatches = exactMatches,
            totalQueries = validQueries,
            exactMatchRate = exactMatches.toDouble() / validQueries
        )
    }
}