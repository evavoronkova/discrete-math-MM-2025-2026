class Solution {
    fun edgeScore(edges: IntArray): Int {
        val n = edges.size
        val score = LongArray(n)

        for (i in 0 until n) {
            score[edges[i]] += i.toLong()
        }

        var ans = 0

        for (i in 1 until n) {
            if (score[i] > score[ans]) {
                ans = i
            }
        }

        return ans
    }
}