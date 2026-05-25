package core.algorithms

import core.model.DirectedGraph
import core.model.Graph

class Component(val id: Int, val vertices: List<Int>) {
    val size: Int get() = vertices.size
}

class ComponentsResult(
    val components: List<Component>,
    val vertexToComponent: IntArray,
    val numComponents: Int,
    val largestComponent: Component,
    val largestSize: Int
) {
    fun largestFraction(totalVertices: Int): Double =
        if (totalVertices == 0) 0.0 else largestSize.toDouble() / totalVertices
}

object ConnectedComponents {

    fun weaklyConnected(graph: Graph): ComponentsResult {
        val n = graph.vertexCount
        val compId = IntArray(n) { -1 }
        val compVertices = mutableListOf<MutableList<Int>>()
        var compCount = 0

        for (v in 0 until n) {
            if (compId[v] != -1) continue
            val vertices = mutableListOf<Int>()
            val queue = IntArray(n)
            var head = 0
            var tail = 0
            queue[tail++] = v
            compId[v] = compCount
            while (head < tail) {
                val cur = queue[head++]
                vertices.add(cur)
                for (nb in graph.neighbors(cur)) {
                    if (compId[nb] == -1) {
                        compId[nb] = compCount
                        queue[tail++] = nb
                    }
                }
            }
            compVertices.add(vertices)
            compCount++
        }

        val components = compVertices.mapIndexed { i, verts -> Component(i, verts) }
        val largest = components.maxByOrNull { it.size } ?: Component(0, emptyList())
        return ComponentsResult(components, compId, compCount, largest, largest.size)
    }

    fun stronglyConnected(graph: DirectedGraph): ComponentsResult {
        val n = graph.vertexCount
        val visited = BooleanArray(n)
        val postOrder = mutableListOf<Int>()

        for (start in 0 until n) {
            if (visited[start]) continue
            val stack = mutableListOf(start)
            val iterStack = mutableListOf(graph.outNeighbors(start).iterator())
            visited[start] = true
            while (stack.isNotEmpty()) {
                val iter = iterStack.last()
                if (iter.hasNext()) {
                    val next = iter.next()
                    if (!visited[next]) {
                        visited[next] = true
                        stack.add(next)
                        iterStack.add(graph.outNeighbors(next).iterator())
                    }
                } else {
                    iterStack.removeAt(iterStack.size - 1)
                    postOrder.add(stack.removeAt(stack.size - 1))
                }
            }
        }

        val compId = IntArray(n) { -1 }
        val compVertices = mutableListOf<MutableList<Int>>()
        var compCount = 0

        for (v in postOrder.reversed()) {
            if (compId[v] != -1) continue
            val vertices = mutableListOf<Int>()
            val queue = IntArray(n)
            var head = 0
            var tail = 0
            queue[tail++] = v
            compId[v] = compCount
            while (head < tail) {
                val cur = queue[head++]
                vertices.add(cur)
                for (nb in graph.inNeighbors(cur)) {
                    if (compId[nb] == -1) {
                        compId[nb] = compCount
                        queue[tail++] = nb
                    }
                }
            }
            compVertices.add(vertices)
            compCount++
        }

        val components = compVertices.mapIndexed { i, verts -> Component(i, verts) }
        val largest = components.maxByOrNull { it.size } ?: Component(0, emptyList())
        return ComponentsResult(components, compId, compCount, largest, largest.size)
    }
}
