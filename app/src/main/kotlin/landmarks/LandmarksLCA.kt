package landmarks

import core.model.Graph
import core.algorithms.BFS

class LandmarksLCA(
    private val graph: Graph,
    private val numLandmarks: Int,
    private val landmarkSelection: LandmarkSelection
) : DistanceEstimator {
    
    private val landmarks: MutableList<Int> = mutableListOf()
    private val sptParents: MutableList<IntArray> = mutableListOf()
    private val sptDepths: MutableList<IntArray> = mutableListOf()

    override fun preprocess() {
        val selected = selectLandmarks()
        landmarks.addAll(selected)

        for (landmark in landmarks) {
            val (depths, parents) = buildShortestPathTree(landmark)
            sptDepths.add(depths)
            sptParents.add(parents)
        }
    }
}