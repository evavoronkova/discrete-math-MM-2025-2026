class Solution {
    fun findOrder(numCourses: Int, prerequisites: Array<IntArray>): IntArray {
        val adj = Array(numCourses) { mutableListOf<Int>() }
        val inDegree = IntArray(numCourses)

        for (prereq in prerequisites) {
            val course = prereq[0]
            val prereqCourse = prereq[1]
            adj[prereqCourse].add(course)
            inDegree[course]++
        }
        val queue = ArrayDeque<Int>()
        for (i in 0 until numCourses) {
            if (inDegree[i] == 0) queue.add(i)
        }
        val result = IntArray(numCourses)
        var idx = 0

        while (queue.isNotEmpty()) {
            val u = queue.removeFirst()
            result[idx++] = u

            for (v in adj[u]) {
                inDegree[v]--
                if (inDegree[v] == 0) {
                    queue.add(v)
                }
            }
        }
        return if (idx == numCourses) result else intArrayOf()
    }
}