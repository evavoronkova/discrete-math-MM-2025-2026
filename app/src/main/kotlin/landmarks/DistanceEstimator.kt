package landmarks

interface DistanceEstimator {
    fun preprocess()
    fun estimateDistance(from: Int, to: Int): Int
    fun exactDistance(from: Int, to: Int): Int
}
