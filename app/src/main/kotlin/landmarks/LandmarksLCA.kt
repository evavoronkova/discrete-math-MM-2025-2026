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

    override fun estimateDistance(from: Int, to: Int): Int {
        var minDistance = Int.MAX_VALUE

        for (i in 0 until landmarks.size) {
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
        val allVertices = (0 until graph.vertexCount).toList()
        return when (landmarkSelection) {
            LandmarkSelection.RANDOM -> allVertices.shuffled().take(numLandmarks)
            LandmarkSelection.HIGH_DEGREE -> allVertices.sortedByDescending { graph.degree(it) }.take(numLandmarks)
            LandmarkSelection.COVERAGE -> allVertices.shuffled().take(numLandmarks)
        }
    }

    private fun buildShortestPathTree(root: Int): Pair<IntArray, IntArray> {
        val result = BFS.run(graph, root)
        return Pair(result.distances, result.parents)
    }

    private fun findLCA(u: Int, v: Int, treeIndex: Int): Int {
        val depths = sptDepths[treeIndex]
        val parents = sptParents[treeIndex]

        if (depths[u] == -1 || depths[v] == -1) return -1

        var currentU = u
        var currentV = v

        while (depths[currentU] > depths[currentV]) {
            currentU = parents[currentU]
        }
        while (depths[currentV] > depths[currentU]) {
            currentV = parents[currentV]
        }

        while (currentU != currentV) {
            currentU = parents[currentU]
            currentV = parents[currentV]
        }

        return currentU
    }

    private fun computeDistanceViaLCA(u: Int, v: Int, treeIndex: Int): Int {
        val lca = findLCA(u, v, treeIndex)
        if (lca == -1) return -1

        val depths = sptDepths[treeIndex]
        
        return depths[u] + depths[v] - 2 * depths[lca]
    }
}
