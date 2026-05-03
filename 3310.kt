class Solution {
    fun remainingMethods(n: Int, k: Int, invocations: Array<IntArray>): IntArray {
        val edgeCount = invocations.size
        val head = IntArray(n) { -1 }
        val to = IntArray(edgeCount)
        val next = IntArray(edgeCount)
        for (i in 0 until edgeCount) {
            val a = invocations[i][0]
            val b = invocations[i][1]
            to[i] = b
            next[i] = head[a]
            head[a] = i
        }
        
        val suspicious = BooleanArray(n)
        val stack = IntArray(n)
        var stackTop = 0
        suspicious[k] = true
        stack[stackTop] = k
        stackTop++
        while (stackTop > 0) {
            stackTop--
            val currentVertex = stack[stackTop]
            var edgeIndex = head[currentVertex]
            while (edgeIndex != -1) {
                val neighborVertex = to[edgeIndex]
                if (!suspicious[neighborVertex]) {
                    suspicious[neighborVertex] = true
                    stack[stackTop] = neighborVertex
                    stackTop++
                }
                edgeIndex = next[edgeIndex]
            }
        }

        for (i in 0 until edgeCount) {
            val a = invocations[i][0]
            val b = invocations[i][1]
            if (!suspicious[a] && suspicious[b]) {
                return IntArray(n) { it }
            }
        }
        val result = IntArray(n)
        var resultIndex = 0
        for (i in 0 until n) {
            if (!suspicious[i]) {
                result[resultIndex] = i
                resultIndex++
            }
        }
        return result.copyOf(resultIndex)
    }
}
