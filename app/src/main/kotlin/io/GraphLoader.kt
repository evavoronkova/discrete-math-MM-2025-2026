package io

import core.algorithms.quickSort
import core.storage.CSRDirectedGraph
import core.storage.CSRUndirectedGraph
import java.io.BufferedReader
import java.io.File
import java.io.IOException

class GraphLoader {

    fun preprocessFile(fileFrom: String, isDirectedGraph: Boolean): String {
        val originalFile = File(fileFrom)
        if (originalFile.length() == 0L) error("File $fileFrom is empty")
        val nameOfNewFile = "processed_${originalFile.name}"
        val preprocessedFile = File(originalFile.parentFile, nameOfNewFile)
        preprocessedFile.createNewFile()
        preprocessedFile.deleteOnExit()
        val command = if (!isDirectedGraph) {
            arrayOf(
                "bash", "-c",
                "tr -d '\\r' < \"$fileFrom\" | awk '{if(\$1!=\$2) print (\$1<\$2 ? \$1\" \"\$2 : \$2\" \"\$1)}' | sort -u > \"$preprocessedFile\""
            )
        } else {
            arrayOf(
                "bash", "-c",
                "tr -d '\\r' < \"$fileFrom\" | awk '{if(\$1!=\$2) print(\$1\" \"\$2)}' | sort -u > \"$preprocessedFile\""
            )
        }

        try {
            val process = ProcessBuilder(*command).start()
            val exitCode = process.waitFor()
            if (exitCode != 0) error("Preprocessing failed with exit code $exitCode")
            if (preprocessedFile.length() == 0L) error("Preprocessing produced empty file")
        } catch (e: IOException) {
            error("Failed to start the process ${e.message}. Make sure bash is installed (Linux/macOS/WSL)")
        }
        return preprocessedFile.toString()
    }

    fun loadDirectedGraph(originalFile: String): CSRDirectedGraph {
        val processedFile = File(preprocessFile(originalFile, true))
        return loadDirectedFromProcessed(processedFile)
    }

    fun loadUndirectedGraph(originalFile: String): CSRUndirectedGraph {
        val processedFile = File(preprocessFile(originalFile, false))
        return loadUndirectedFromProcessed(processedFile)
    }

    private fun loadDirectedFromProcessed(file: File): CSRDirectedGraph {
        val vertexSet = mutableSetOf<Int>()
        val outDegreeByOrig = IntArrayList()
        val inDegreeByOrig = IntArrayList()
        val edges = ArrayList<Long>()

        file.bufferedReader().use { reader ->
            var line = reader.readLine()
            while (line != null) {
                if (line.isBlank() || line[0] == '#') { line = reader.readLine(); continue }
                val spaceIdx = line.indexOf(' ')
                if (spaceIdx == -1) { line = reader.readLine(); continue }
                val u = line.substring(0, spaceIdx).toInt()
                val v = line.substring(spaceIdx + 1).toInt()
                vertexSet.add(u); vertexSet.add(v)
                outDegreeByOrig.add(u)
                inDegreeByOrig.add(v)
                edges.add(pack(u, v))
                line = reader.readLine()
            }
        }

        val vertexCount = vertexSet.size
        val prevVertNumbers = vertexSet.toIntArray()
        quickSort(prevVertNumbers, 0, vertexCount - 1)
        val vertexToIndex = Int2IntMap(prevVertNumbers)

        val outDegree = IntArray(vertexCount)
        val inDegree = IntArray(vertexCount)
        for (i in 0 until outDegreeByOrig.size) {
            outDegree[vertexToIndex[outDegreeByOrig[i]]]++
            inDegree[vertexToIndex[inDegreeByOrig[i]]]++
        }

        val outOffs = buildOffsets(outDegree, vertexCount)
        val inOffs = buildOffsets(inDegree, vertexCount)
        val edgeCount = outDegreeByOrig.size

        val outNeighs = IntArray(edgeCount)
        val inNeighs = IntArray(edgeCount)
        val outPos = IntArray(vertexCount)
        val inPos = IntArray(vertexCount)

        for (i in 0 until edges.size) {
            val (u, v) = unpack(edges[i])
            val ui = vertexToIndex[u]
            val vi = vertexToIndex[v]
            outNeighs[outOffs[ui] + outPos[ui]] = vi
            outPos[ui]++
            inNeighs[inOffs[vi] + inPos[vi]] = ui
            inPos[vi]++
        }

        return CSRDirectedGraph(prevVertNumbers, outOffs, outNeighs, inOffs, inNeighs)
    }

    private fun loadUndirectedFromProcessed(file: File): CSRUndirectedGraph {
        val vertexSet = mutableSetOf<Int>()
        val degreeByOrig = IntArrayList()
        val edges = ArrayList<Long>()

        file.bufferedReader().use { reader ->
            var line = reader.readLine()
            while (line != null) {
                if (line.isBlank() || line[0] == '#') { line = reader.readLine(); continue }
                val spaceIdx = line.indexOf(' ')
                if (spaceIdx == -1) { line = reader.readLine(); continue }
                val u = line.substring(0, spaceIdx).toInt()
                val v = line.substring(spaceIdx + 1).toInt()
                vertexSet.add(u); vertexSet.add(v)
                degreeByOrig.add(u)
                degreeByOrig.add(v)
                edges.add(pack(u, v))
                line = reader.readLine()
            }
        }

        val vertexCount = vertexSet.size
        val prevVertNumbers = vertexSet.toIntArray()
        quickSort(prevVertNumbers, 0, vertexCount - 1)
        val vertexToIndex = Int2IntMap(prevVertNumbers)

        val degree = IntArray(vertexCount)
        for (i in 0 until degreeByOrig.size) {
            degree[vertexToIndex[degreeByOrig[i]]]++
        }

        val offs = buildOffsets(degree, vertexCount)
        val edgeCount = degreeByOrig.size / 2

        val neighs = IntArray(degreeByOrig.size)
        val pos = IntArray(vertexCount)

        for (i in 0 until edges.size) {
            val (u, v) = unpack(edges[i])
            val ui = vertexToIndex[u]
            val vi = vertexToIndex[v]
            neighs[offs[ui] + pos[ui]] = vi
            pos[ui]++
            neighs[offs[vi] + pos[vi]] = ui
            pos[vi]++
        }

        return CSRUndirectedGraph(prevVertNumbers, offs, neighs)
    }

    private fun pack(u: Int, v: Int): Long = (u.toLong() shl 32) or (v.toLong() and 0xFFFFFFFFL)
    private fun unpack(packed: Long): Pair<Int, Int> = Pair((packed shr 32).toInt(), packed.toInt())

    private fun buildOffsets(degree: IntArray, vertexCount: Int): IntArray {
        val offs = IntArray(vertexCount + 1)
        for (i in 1..vertexCount) offs[i] = offs[i - 1] + degree[i - 1]
        return offs
    }
}

private class IntArrayList {
    private var data = IntArray(1024)
    var size = 0
        private set

    fun add(value: Int) {
        if (size >= data.size) data = data.copyOf(data.size * 2)
        data[size++] = value
    }

    operator fun get(index: Int): Int = data[index]
}

private class Int2IntMap(sortedKeys: IntArray) {
    private val keys = sortedKeys
    private val vals = IntArray(sortedKeys.size) { it }

    operator fun get(key: Int): Int {
        var lo = 0
        var hi = keys.size - 1
        while (lo <= hi) {
            val mid = (lo + hi) ushr 1
            val k = keys[mid]
            if (k < key) lo = mid + 1
            else if (k > key) hi = mid - 1
            else return vals[mid]
        }
        error("Vertex $key not found in mapping")
    }
}
