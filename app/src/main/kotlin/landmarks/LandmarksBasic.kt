package landmarks

import core.model.Graph
import core.algorithms.BFS
import utils.Parallel

class LandmarksBasic(
    private val graph: Graph,
    private val numLandmarks: Int,
    private val landmarkSelection: LandmarkSelection
) : DistanceEstimator {

    private val landmarks: MutableList<Int> = mutableListOf()
    private val landmarkDistances: MutableList<IntArray> = mutableListOf()

    override fun preprocess() {
        if (landmarkSelection == LandmarkSelection.COVERAGE) {
            preprocessCoverage()
        } else {
            val selected = selectLandmarks()
            landmarks.addAll(selected)
            landmarkDistances.addAll(Parallel.map(landmarks) { BFS.run(graph, it).distances })
        }
    }

    override fun estimateDistance(from: Int, to: Int): Int {
        var minDistance = Int.MAX_VALUE

        for (distancesFromLandmark in landmarkDistances) {
            val distToFrom = distancesFromLandmark[from]
            val distToTo = distancesFromLandmark[to]

            if (distToFrom != -1 && distToTo != -1) {
                val currentEstimate = distToFrom + distToTo
                if (currentEstimate < minDistance) {
                    minDistance = currentEstimate
                }
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

    private fun preprocessCoverage() {
        val n = graph.vertexCount
        val minDist = IntArray(n) { Int.MAX_VALUE }

        val first = (0 until n).maxByOrNull { graph.degree(it) } ?: 0
        landmarks.add(first)
        val firstBfs = BFS.run(graph, first)
        landmarkDistances.add(firstBfs.distances)
        for (v in 0 until n) {
            if (firstBfs.distances[v] != -1) minDist[v] = firstBfs.distances[v]
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
            val bfs = BFS.run(graph, best)
            landmarkDistances.add(bfs.distances)
            for (v in 0 until n) {
                if (bfs.distances[v] != -1 && bfs.distances[v] < minDist[v]) {
                    minDist[v] = bfs.distances[v]
                }
            }
        }
    }
}