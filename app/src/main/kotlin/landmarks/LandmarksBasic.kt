package landmarks

import core.model.Graph
import core.algorithms.BFS

class LandmarksBasic(
    private val graph: Graph,
    private val numLandmarks: Int,
    private val landmarkSelection: LandmarkSelection
) : DistanceEstimator {

    private val landmarks: MutableList<Int> = mutableListOf()
    private val landmarkDistances: MutableList<IntArray> = mutableListOf()

    override fun preprocess() {
        val selected = selectLandmarks()
        landmarks.addAll(selected)

        for (landmark in landmarks) {
            val bfsResult = BFS.run(graph, landmark)
            landmarkDistances.add(bfsResult.distances)
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
        val allVertices = (0 until graph.vertexCount).toList()
        return allVertices.sortedByDescending { graph.degree(it) }.take(numLandmarks)
    }

    private fun selectCoverageLandmarks(): List<Int> {
        return selectRandomLandmarks() 
    }
}