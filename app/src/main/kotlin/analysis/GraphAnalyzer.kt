package analysis

import core.algorithms.BFS
import core.algorithms.Component
import core.algorithms.ConnectedComponents
import core.model.Graph

class GraphAnalyzer(private val graph: Graph) {

    fun computeBasicStats(isDirected: Boolean = false): BasicStats {
        val n = graph.vertexCount
        val edgeCount = graph.edgeCount
        val density = graph.density()

        val wcc = ConnectedComponents.weaklyConnected(graph)
        val weakComponents = wcc.numComponents
        val weakFrac = wcc.largestFraction(n)

        var strongComponents: Int? = null
        var strongFrac: Double? = null
        if (isDirected && graph is core.model.DirectedGraph) {
            val scc = ConnectedComponents.stronglyConnected(graph)
            strongComponents = scc.numComponents
            strongFrac = scc.largestFraction(n)
        }

        return BasicStats(n, edgeCount, density, weakComponents, weakFrac, strongComponents, strongFrac)
    }

    fun estimateDiameter(largestComponent: Component, numRandomPairs: Int, snowballSize: Int): DiameterStats {
        val (diam, src, tgt) = doubleSweep(largestComponent)
        val (rdDiam, rdP90) = randomPairsEstimate(largestComponent, numRandomPairs)
        val (snDiam, snP90) = snowballEstimate(largestComponent, snowballSize)
        return DiameterStats(diam, src, tgt, rdDiam, rdP90, snDiam, snP90)
    }

    fun computeClusteringStats(): ClusteringStats {
        return computeClusteringStatsForVertSet(null)
    }

    fun computeClusteringStatsInComponent(component: Component): ClusteringStats {
        return computeClusteringStatsForVertSet(component.vertices.toSet())
    }

    fun computeDegreeStats(): DegreeStats {
        val n = graph.vertexCount
        val degreeCounts = mutableMapOf<Int, Int>()
        var minDeg = Int.MAX_VALUE
        var maxDeg = 0
        var sumDeg = 0L

        for (v in 0 until n) {
            val d = graph.degree(v)
            if (d < minDeg) minDeg = d
            if (d > maxDeg) maxDeg = d
            sumDeg += d
            degreeCounts[d] = (degreeCounts[d] ?: 0) + 1
        }

        return DegreeStats(minDeg, maxDeg, sumDeg.toDouble() / n, degreeCounts)
    }

    private fun computeClusteringStatsForVertSet(vertSet: Set<Int>?): ClusteringStats {
        val n = graph.vertexCount
        val deg = IntArray(n) { graph.degree(it) }

        val degInComp = if (vertSet == null) deg else IntArray(n) { v ->
            if (v in vertSet) graph.neighbors(v).count { it in vertSet } else 0
        }

        val order = (0 until n).sortedWith(compareBy({ degInComp[it] }, { it }))
        val rank = IntArray(n)
        for ((idx, v) in order.withIndex()) rank[v] = idx

        val mark = BooleanArray(n)
        val triangleCount = IntArray(n)
        var connectedTriples = 0L

        for (v in order) {
            if (vertSet != null && v !in vertSet) continue
            val k = degInComp[v]
            if (k < 2) continue
            connectedTriples += k.toLong() * (k - 1) / 2

            for (u in graph.neighbors(v)) {
                if (vertSet != null && u !in vertSet) continue
                if (rank[u] > rank[v]) mark[u] = true
            }

            for (u in graph.neighbors(v)) {
                if (!mark[u]) continue
                val uNeighbors = graph.neighbors(u)
                for (w in uNeighbors) {
                    if (rank[w] > rank[u] && mark[w]) {
                        triangleCount[v]++
                        triangleCount[u]++
                        triangleCount[w]++
                    }
                }
            }

            for (u in graph.neighbors(v)) mark[u] = false
        }

        val totalTriangles = triangleCount.sum() / 3L

        var totalLocalC = 0.0
        for (v in 0 until n) {
            if (vertSet != null && v !in vertSet) continue
            val k = degInComp[v]
            if (k >= 2) {
                totalLocalC += (2.0 * triangleCount[v]) / (k.toDouble() * (k - 1))
            }
        }

        val divisor = vertSet?.size ?: n
        val avgC = totalLocalC / divisor
        val globalC = if (connectedTriples > 0) (3.0 * totalTriangles) / connectedTriples.toDouble() else 0.0

        return ClusteringStats(totalTriangles, avgC, globalC)
    }

    private fun doubleSweep(component: Component): Triple<Int, Int, Int> {
        val verts = component.vertices
        if (verts.size <= 1) return Triple(0, if (verts.isEmpty()) -1 else verts[0], if (verts.isEmpty()) -1 else verts[0])

        val start = verts.random()
        val firstBfs = BFS.run(graph, start)
        var src = start
        var maxDist = -1
        for (v in verts) {
            val d = firstBfs.distances[v]
            if (d > maxDist) {
                maxDist = d
                src = v
            }
        }

        val secondBfs = BFS.run(graph, src)
        var tgt = src
        maxDist = -1
        for (v in verts) {
            val d = secondBfs.distances[v]
            if (d > maxDist) {
                maxDist = d
                tgt = v
            }
        }

        return Triple(maxDist, src, tgt)
    }

    private fun randomPairsEstimate(component: Component, numPairs: Int): Pair<Int, Int> {
        val verts = component.vertices
        if (verts.size < 2) return Pair(0, 0)

        val sources = verts.shuffled().take(numPairs.coerceAtMost(verts.size))
        val distances = mutableListOf<Int>()

        for (i in sources.indices) {
            val u = sources[i]
            val bfs = BFS.run(graph, u)
            for (j in i + 1 until sources.size) {
                val v = sources[j]
                val d = bfs.distances[v]
                if (d != -1) distances.add(d)
            }
        }

        if (distances.isEmpty()) return Pair(0, 0)
        distances.sort()
        val diam = distances.max()
        val p90Idx = (0.9 * (distances.size - 1)).toInt()
        val p90 = distances[p90Idx.coerceIn(0, distances.size - 1)]
        return Pair(diam, p90)
    }

    private fun snowballEstimate(component: Component, targetSize: Int): Pair<Int, Int> {
        val sample = buildSnowballSample(component, targetSize)
        if (sample.size < 2) return Pair(0, 0)

        val distances = mutableListOf<Int>()
        val bfsSourceCount = minOf(targetSize, sample.size)

        for (i in sample.indices.take(bfsSourceCount)) {
            val u = sample[i]
            val bfs = BFS.run(graph, u)
            for (j in i + 1 until sample.size) {
                val v = sample[j]
                val d = bfs.distances[v]
                if (d != -1) distances.add(d)
            }
        }

        if (distances.isEmpty()) return Pair(0, 0)
        distances.sort()
        val diam = distances.max()
        val p90Idx = (0.9 * (distances.size - 1)).toInt()
        val p90 = distances[p90Idx.coerceIn(0, distances.size - 1)]
        return Pair(diam, p90)
    }

    private fun buildSnowballSample(component: Component, targetSize: Int): List<Int> {
        val verts = component.vertices
        if (verts.size <= targetSize) return verts

        val vertSet = verts.toSet()
        val visited = BooleanArray(graph.vertexCount)

        val seed = verts.random()
        val seedNeighbors = graph.neighbors(seed).filter { it in vertSet }
        val seeds = mutableListOf(seed)
        seeds.addAll(seedNeighbors.shuffled().take(2))

        val sample = mutableListOf<Int>()
        val queue = IntArray(graph.vertexCount)
        var head = 0
        var tail = 0

        for (s in seeds) {
            if (!visited[s]) {
                visited[s] = true
                queue[tail++] = s
                sample.add(s)
            }
        }

        while (head < tail && sample.size < targetSize) {
            val cur = queue[head++]
            for (nb in graph.neighbors(cur)) {
                if (nb !in vertSet) continue
                if (!visited[nb]) {
                    visited[nb] = true
                    queue[tail++] = nb
                    sample.add(nb)
                    if (sample.size >= targetSize) break
                }
            }
        }

        return sample
    }
}
