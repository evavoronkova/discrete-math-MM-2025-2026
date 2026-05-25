package experiments

import core.model.Graph
import core.model.MutableVertexGraph

data class DeletionExperimentResult(
    val percentDeleted: Int,
    val numComponents: Int,
    val largestComponentSize: Int,
    val largestComponentFraction: Double
)

class DeletionExperiments(private val graph: Graph, private val mutableGraph: MutableVertexGraph? = null) {

    fun randomDeletion(percentages: List<Int>): List<DeletionExperimentResult> {
        val n = graph.vertexCount
        val results = mutableListOf<DeletionExperimentResult>()

        for (pct in percentages) {
            val removeCount = (n * pct / 100.0).toInt().coerceAtMost(n)
            val removed = (0 until n).shuffled().take(removeCount).toSet()

            if (mutableGraph != null) {
                mutableGraph.markDeleted(removed.toList())
                val (numComp, largestSize, largestFrac) = computeComponentInfoFromMutable()
                mutableGraph.clearDeleted()
                results.add(DeletionExperimentResult(pct, numComp, largestSize, largestFrac))
            } else {
                val (numComp, largestSize, largestFrac) = computeComponentInfoWithSet(removed)
                results.add(DeletionExperimentResult(pct, numComp, largestSize, largestFrac))
            }
        }

        return results
    }

    fun highDegreeDeletion(percentages: List<Int>): List<DeletionExperimentResult> {
        val n = graph.vertexCount
        val sortedByDegree = (0 until n).sortedByDescending { graph.degree(it) }
        val results = mutableListOf<DeletionExperimentResult>()

        for (pct in percentages) {
            val removeCount = (n * pct / 100.0).toInt().coerceAtMost(n)
            val removed = sortedByDegree.take(removeCount).toSet()

            if (mutableGraph != null) {
                mutableGraph.markDeleted(removed.toList())
                val (numComp, largestSize, largestFrac) = computeComponentInfoFromMutable()
                mutableGraph.clearDeleted()
                results.add(DeletionExperimentResult(pct, numComp, largestSize, largestFrac))
            } else {
                val (numComp, largestSize, largestFrac) = computeComponentInfoWithSet(removed)
                results.add(DeletionExperimentResult(pct, numComp, largestSize, largestFrac))
            }
        }

        return results
    }

    fun printResults(results: List<DeletionExperimentResult>, title: String) {
        println("--- $title ---")
        for (r in results) {
            println("  ${r.percentDeleted}%: components=${r.numComponents}, largest=${r.largestComponentSize} (${"%.4f".format(r.largestComponentFraction)})")
        }
    }

    private fun computeComponentInfoFromMutable(): Triple<Int, Int, Double> {
        val mg = mutableGraph ?: return Triple(0, 0, 0.0)
        val n = graph.vertexCount
        val visited = BooleanArray(n)
        var numComponents = 0
        var largestSize = 0
        var totalActive = 0

        for (v in 0 until n) {
            if (mg.isDeleted(v)) continue
            totalActive++
            if (visited[v]) continue
            numComponents++
            var size = 0
            val queue = IntArray(n)
            var head = 0
            var tail = 0
            queue[tail++] = v
            visited[v] = true
            while (head < tail) {
                val cur = queue[head++]
                size++
                for (nb in graph.neighbors(cur)) {
                    if (!mg.isDeleted(nb) && !visited[nb]) {
                        visited[nb] = true
                        queue[tail++] = nb
                    }
                }
            }
            if (size > largestSize) largestSize = size
        }

        val fraction = if (totalActive == 0) 0.0 else largestSize.toDouble() / totalActive
        return Triple(numComponents, largestSize, fraction)
    }

    private fun computeComponentInfoWithSet(removed: Set<Int>): Triple<Int, Int, Double> {
        val n = graph.vertexCount
        val visited = BooleanArray(n)
        var numComponents = 0
        var largestSize = 0
        var totalActive = 0

        for (v in 0 until n) {
            if (v in removed) continue
            totalActive++
            if (visited[v]) continue
            numComponents++
            var size = 0
            val queue = IntArray(n)
            var head = 0
            var tail = 0
            queue[tail++] = v
            visited[v] = true
            while (head < tail) {
                val cur = queue[head++]
                size++
                for (nb in graph.neighbors(cur)) {
                    if (nb !in removed && !visited[nb]) {
                        visited[nb] = true
                        queue[tail++] = nb
                    }
                }
            }
            if (size > largestSize) largestSize = size
        }

        val fraction = if (totalActive == 0) 0.0 else largestSize.toDouble() / totalActive
        return Triple(numComponents, largestSize, fraction)
    }
}
