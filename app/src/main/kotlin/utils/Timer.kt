package utils

object Timer {
    private var startTime: Long = 0L

    fun start() {
        startTime = System.nanoTime()
    }

    fun stop(): Long {
        val elapsed = System.nanoTime() - startTime
        startTime = 0L
        return elapsed
    }

    fun elapsed(): Long = System.nanoTime() - startTime

    inline fun measure(block: () -> Unit): Long {
        start()
        block()
        return stop()
    }
}
