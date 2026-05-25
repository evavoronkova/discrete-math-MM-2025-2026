package core.algorithms

import core.model.Graph

class BFS {
    companion object {
        fun run(graph: Graph, startVertex: Int): BFSResult {
            val vertexCount = graph.vertexCount

            val distances = IntArray(vertexCount) { -1 }
            val parents = IntArray(vertexCount) { -1 }

            val queue = IntArray(vertexCount)
            var head = 0
            var tail = 0

            queue[tail++] = startVertex
            distances[startVertex] = 0
            parents[startVertex] = startVertex

            while (head < tail) {
                val current = queue[head++]
                val currentDist = distances[current]

                val neighbors = graph.neighbors(current)

                for (neighbor in neighbors) {
                    if (distances[neighbor] == -1) {
                        distances[neighbor] = currentDist + 1
                        parents[neighbor] = current
                        queue[tail++] = neighbor
                    }
                }
            }

            return BFSResult(distances, parents)
        }

        fun multiSourceBFS(graph: Graph, sources: Collection<Int>): BFSResult {
            val vertexCount = graph.vertexCount

            val distances = IntArray(vertexCount) { -1 }
            val parents = IntArray(vertexCount) { -1 }

            val queue = IntArray(vertexCount)
            var head = 0
            var tail = 0

            for (s in sources) {
                queue[tail++] = s
                distances[s] = 0
                parents[s] = s
            }

            while (head < tail) {
                val current = queue[head++]
                val currentDist = distances[current]

                for (neighbor in graph.neighbors(current)) {
                    if (distances[neighbor] == -1) {
                        distances[neighbor] = currentDist + 1
                        parents[neighbor] = current
                        queue[tail++] = neighbor
                    }
                }
            }

            return BFSResult(distances, parents)
        }
    }
}
