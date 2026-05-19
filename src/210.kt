class Solution {
    fun findOrder(numCourses: Int, prerequisites: Array<IntArray>): IntArray {
        val adjList = Array(numCourses) { mutableListOf<Int>() }
        val inDegree = IntArray(numCourses)

        for ((course, required) in prerequisites) {
            adjList[required].add(course)
            inDegree[course]++
        }

        val queue = ArrayDeque<Int>()
        for (i in 0..<numCourses) {
            if (inDegree[i] == 0) {
                queue.add(i)
            }
        }

        val result = IntArray(numCourses)
        var count = 0

        while (queue.isNotEmpty()) {
            val curr = queue.removeFirst()
            result[count++] = curr

            for (neighbor in adjList[curr]) {
                inDegree[neighbor]--
                if (inDegree[neighbor] == 0) {
                    queue.add(neighbor)
                }
            }
        }

        return if (count == numCourses) result else intArrayOf()
    }
}