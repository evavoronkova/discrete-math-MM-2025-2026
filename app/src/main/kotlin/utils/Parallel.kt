package utils

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.runBlocking

object Parallel {
    private val dispatcher = Dispatchers.Default.limitedParallelism(4)

    fun <T, R> map(items: List<T>, block: (T) -> R): List<R> = runBlocking {
        coroutineScope {
            items.map { async(dispatcher) { block(it) } }.awaitAll()
        }
    }
}
