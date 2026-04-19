class Solution {
    fun canFinish(numCourses: Int, prerequisites: Array<IntArray>): Boolean {
        val graph = Array(numCourses) { mutableListOf<Int>() }
        for (i in prerequisites) {
            graph[i[0]].add(i[1])
        }

        val state = IntArray(numCourses) { 0 }

        fun hasCycle(v: Int): Boolean {
            if (state[v] == 1) {
                return true
            }
            if (state[v] == 2) {
                return false
            }
            state[v] = 1
            for (j in graph[v]) {
                if (hasCycle(j)) return true
            }
            state[v] = 2
            return false
        }

        for (i in 0..<numCourses) {
            if (state[i] == 0) {
                if (hasCycle(i)) return false
            }
        }
        return true
    }
}