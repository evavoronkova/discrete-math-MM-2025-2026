package landmarks

import core.model.Graph
import core.algorithms.BFS
import kotlin.math.ln
import utils.Parallel

class LandmarksLCA(
    private val graph: Graph,
    private val numLandmarks: Int,
    private val landmarkSelection: LandmarkSelection
) : DistanceEstimator {

    private val landmarks: MutableList<Int> = mutableListOf()
    private val sptParents: MutableList<IntArray> = mutableListOf()
    private val sptDepths: MutableList<IntArray> = mutableListOf()

    private val logN: Int by lazy {
        val n = graph.vertexCount
        if (n <= 1) 0 else (ln(n.toDouble()) / ln(2.0)).toInt() + 1
    }

    private val upTables: MutableList<Array<IntArray>> = mutableListOf()

    override fun preprocess() {
        when (landmarkSelection) {
            LandmarkSelection.COVERAGE -> preprocessCoverage()
            else -> {
                val selected = selectLandmarks()
                landmarks.addAll(selected)
                val up = upTables // trigger lazy logN
                val results = Parallel.map(landmarks) { lm ->
                    val (depths, parents) = buildShortestPathTree(lm)
                    Triple(depths, parents, buildBinaryLifting(parents, depths))
                }
                for ((d, p, u) in results) {
                    sptDepths.add(d); sptParents.add(p); upTables.add(u)
                }
            }
        }
    }

    override fun estimateDistance(from: Int, to: Int): Int {
        var minDistance = Int.MAX_VALUE

        for (i in landmarks.indices) {
            val dist = computeDistanceViaLCA(from, to, i)
            if (dist != -1 && dist < minDistance) {
                minDistance = dist
            }
        }

        return if (minDistance == Int.MAX_VALUE) -1 else minDistance
    }

    override fun exactDistance(from: Int, to: Int): Int {
        val bfsResult = BFS.run(graph, from)
        return bfsResult.distances[to]
    }

    private fun selectLandmarks(): List<Int> {
        return when (landmarkSelection) {
            LandmarkSelection.RANDOM -> selectRandomLandmarks()
            LandmarkSelection.HIGH_DEGREE -> selectHighDegreeLandmarks()
            LandmarkSelection.COVERAGE -> selectCoverageLandmarks()
        }
    }

    private fun selectRandomLandmarks(): List<Int> {
        val allVertices = (0 until graph.vertexCount).toList()
        return allVertices.shuffled().take(numLandmarks)
    }

    private fun selectHighDegreeLandmarks(): List<Int> {
        return (0 until graph.vertexCount)
            .sortedByDescending { graph.degree(it) }
            .take(numLandmarks)
    }

    private fun selectCoverageLandmarks(): List<Int> {
        return selectRandomLandmarks()
    }

    private fun buildShortestPathTree(root: Int): Pair<IntArray, IntArray> {
        val result = BFS.run(graph, root)
        return Pair(result.distances, result.parents)
    }

    private fun buildBinaryLifting(parents: IntArray, depths: IntArray): Array<IntArray> {
        val n = parents.size
        val up = Array(logN) { IntArray(n) { -1 } }
        for (v in 0 until n) {
            up[0][v] = if (parents[v] == v || parents[v] == -1) -1 else parents[v]
        }
        for (k in 1 until logN) {
            for (v in 0 until n) {
                val mid = up[k - 1][v]
                up[k][v] = if (mid != -1) up[k - 1][mid] else -1
            }
        }
        return up
    }

    private fun findLCA(u: Int, v: Int, treeIndex: Int): Int {
        val depths = sptDepths[treeIndex]
        val up = upTables[treeIndex]

        if (depths[u] == -1 || depths[v] == -1) return -1

        var curU = u
        var curV = v

        if (depths[curU] < depths[curV]) {
            val diff = depths[curV] - depths[curU]
            var k = 0
            var d = diff
            while (d > 0) {
                if ((d and 1) == 1) curV = up[k][curV]
                d = d shr 1
                k++
            }
        } else if (depths[curV] < depths[curU]) {
            val diff = depths[curU] - depths[curV]
            var k = 0
            var d = diff
            while (d > 0) {
                if ((d and 1) == 1) curU = up[k][curU]
                d = d shr 1
                k++
            }
        }

        if (curU == curV) return curU

        for (k in logN - 1 downTo 0) {
            val nextU = up[k][curU]
            val nextV = up[k][curV]
            if (nextU != -1 && nextV != -1 && nextU != nextV) {
                curU = nextU
                curV = nextV
            }
        }

        return up[0][curU]
    }

    private fun computeDistanceViaLCA(u: Int, v: Int, treeIndex: Int): Int {
        val lca = findLCA(u, v, treeIndex)
        if (lca == -1) return -1

        val depths = sptDepths[treeIndex]
        return depths[u] + depths[v] - 2 * depths[lca]
    }

    private fun preprocessCoverage() {
        val n = graph.vertexCount
        val minDist = IntArray(n) { Int.MAX_VALUE }

        val first = (0 until n).maxByOrNull { graph.degree(it) } ?: 0
        landmarks.add(first)
        val (depths, parents) = buildShortestPathTree(first)
        sptDepths.add(depths)
        sptParents.add(parents)
        upTables.add(buildBinaryLifting(parents, depths))
        for (v in 0 until n) {
            if (depths[v] != -1) minDist[v] = depths[v]
        }

        while (landmarks.size < numLandmarks) {
            var best = -1
            var bestDist = -1
            for (v in 0 until n) {
                if (v in landmarks) continue
                if (minDist[v] > bestDist) {
                    bestDist = minDist[v]
                    best = v
                }
            }
            if (best == -1) break
            landmarks.add(best)
            val (d, p) = buildShortestPathTree(best)
            sptDepths.add(d)
            sptParents.add(p)
            upTables.add(buildBinaryLifting(p, d))
            for (v in 0 until n) {
                if (d[v] != -1 && d[v] < minDist[v]) {
                    minDist[v] = d[v]
                }
            }
        }
    }
}