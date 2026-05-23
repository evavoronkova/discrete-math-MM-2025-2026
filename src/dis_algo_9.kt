class Solution {

    data class Edge(
        val to: Int,
        val w: Int
    )

    data class State(
        val d: Long,
        val v: Int
    )

    fun findAnswer(n: Int, edges: Array<IntArray>): BooleanArray {

        val g = Array(n) { mutableListOf<Edge>() }

        for (e in edges) {

            val a = e[0]
            val b = e[1]
            val w = e[2]

            g[a].add(Edge(b, w))
            g[b].add(Edge(a, w))
        }

        fun dijkstra(start: Int): LongArray {

            val dist = LongArray(n) { Long.MAX_VALUE }

            val pq = java.util.PriorityQueue<State> { a, b ->
                a.d.compareTo(b.d)
            }

            dist[start] = 0L
            pq.add(State(0L, start))

            while (pq.isNotEmpty()) {

                val cur = pq.poll()

                if (cur.d != dist[cur.v]) {
                    continue
                }

                for (e in g[cur.v]) {

                    val nd = cur.d + e.w

                    if (nd < dist[e.to]) {

                        dist[e.to] = nd

                        pq.add(State(nd, e.to))
                    }
                }
            }

            return dist
        }

        val d1 = dijkstra(0)
        val d2 = dijkstra(n - 1)

        val best = d1[n - 1]

        val ans = BooleanArray(edges.size)

        if (best == Long.MAX_VALUE) {
            return ans
        }

        for (i in edges.indices) {

            val u = edges[i][0]
            val v = edges[i][1]
            val w = edges[i][2].toLong()

            if (
                (d1[u] != Long.MAX_VALUE &&
                        d2[v] != Long.MAX_VALUE &&
                        d1[u] + w + d2[v] == best)
                ||
                (d1[v] != Long.MAX_VALUE &&
                        d2[u] != Long.MAX_VALUE &&
                        d1[v] + w + d2[u] == best)
            ) {
                ans[i] = true
            }
        }

        return ans
    }
}