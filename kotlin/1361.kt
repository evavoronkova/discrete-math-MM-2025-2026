class Solution {
    fun validateBinaryTreeNodes(n: Int, leftChild: IntArray, rightChild: IntArray): Boolean {
        val indegree = IntArray(n)
        for (i in 0 until n) {
            if (leftChild[i] != -1) indegree[leftChild[i]]++
            if (rightChild[i] != -1) indegree[rightChild[i]]++
        }
        var root = -1
        for (i in 0 until n) {
            when (indegree[i]) {
                0 -> {
                    if (root != -1) return false
                    root = i
                }
                1 -> Unit
                else -> return false
            }
        }
        if (root == -1) return false
        val visited = BooleanArray(n)
        val queue = IntArray(n)
        var head = 0
        var tail = 0
        queue[tail++] = root
        visited[root] = true
        var count = 0

        while (head < tail) {
            val curr = queue[head++]
            count++

            val left = leftChild[curr]
            val right = rightChild[curr]

            if (left != -1) {
                if (visited[left]) return false
                visited[left] = true
                queue[tail++] = left
            }
            if (right != -1) {
                if (visited[right]) return false
                visited[right] = true
                queue[tail++] = right
            }
        }
        return count == n
    }
}