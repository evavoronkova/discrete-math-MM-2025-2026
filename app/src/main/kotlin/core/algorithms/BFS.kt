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
    }
}