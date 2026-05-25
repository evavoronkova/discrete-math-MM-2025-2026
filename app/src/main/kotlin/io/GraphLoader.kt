package io

import core.algorithms.quickSort
import core.storage.CSRDirectedGraph
import core.storage.CSRUndirectedGraph
import java.io.File
import java.io.IOException

class GraphLoader {
    fun preprocessFile(fileFrom: String, isDirectedGraph: Boolean): String{
        val originalFile = File(fileFrom)
        if(originalFile.length() == 0L) error("File $fileFrom is empty")
        val nameOfNewFile = "processed_${originalFile.name}"
        val preprocessedFile = File(originalFile.parentFile, nameOfNewFile)
        preprocessedFile.createNewFile()
        preprocessedFile.deleteOnExit()
        val command = if(!isDirectedGraph) {
            arrayOf(
                "bash",
                "-c",
                "tr -d '\\r' < \"$fileFrom\" | awk '{if(\$1!=\$2) print (\$1<\$2 ? \$1\" \"\$2 : \$2\" \"\$1)}' | sort -u > \"$preprocessedFile\""
            )
        }else{
            arrayOf(
                "bash",
                "-c",
                "tr -d '\\r' < \"$fileFrom\" | awk '{if(\$1!=\$2) print(\$1\" \"\$2)}' | sort -u > \"$preprocessedFile\""
            )
        }

        try {
            val process = ProcessBuilder(*command).start()
            val exitCode = process.waitFor()
            if (exitCode != 0) error("Preprocessing failed with exit code $exitCode")
            if (preprocessedFile.length() == 0L) error("Preprocessing produced empty file")
        } catch (e: IOException){
            error("Failed to start the process ${e.message}. Make sure bash is installed (Linux/macOS/WSL)")
        }
        return preprocessedFile.toString()
    }

    fun loadDirectedGraph(originalFile: String): CSRDirectedGraph{
        val processedFile = File(preprocessFile(originalFile, true))

        val vertexSet = mutableSetOf<Int>()
        processedFile.useLines { lines ->
            lines.forEach { line ->
                if(line.isBlank()) throw IllegalStateException("Line $line is empty")
                val parts = line.trim().split(' ')
                if(parts.size != 2) throw IllegalStateException("Invalid line: $line")
                val u = parts[0].toIntOrNull() ?: throw IllegalStateException("Invalid line: $line")
                val v = parts[1].toIntOrNull() ?: throw IllegalStateException("Invalid line: $line")
                vertexSet.add(u); vertexSet.add(v)
            }
        }
        val vertexCount = vertexSet.size
        val prevVertNumbers = vertexSet.toIntArray()
        quickSort(prevVertNumbers, 0, vertexCount - 1)
        val vertexToIndexMap = mutableMapOf<Int, Int>()
        for(i in 0 until vertexCount){
            vertexToIndexMap[prevVertNumbers[i]] = i
        }

        var edgeCount = 0
        val inDegree = IntArray(vertexCount)
        val outDegree = IntArray(vertexCount)
        processedFile.useLines { lines ->
            lines.forEach { line ->
                val (u, v) = line.trim().split(' ').map{ it.toInt() }
                outDegree[vertexToIndexMap[u]!!]++
                inDegree[vertexToIndexMap[v]!!]++
                edgeCount++
            }
        }
        val inOffs = IntArray(vertexCount + 1)
        val outOffs = IntArray(vertexCount + 1)
        for(i in 1 .. vertexCount){
            inOffs[i] = inOffs[i - 1] + inDegree[i - 1]
            outOffs[i] = outOffs[i - 1] + outDegree[i - 1]
        }

        val inNeighs = IntArray(edgeCount)
        val outNeighs = IntArray(edgeCount)
        val inCurrentPosition = IntArray(vertexCount)
        val outCurrentPosition = IntArray(vertexCount)
        processedFile.useLines { lines ->
            lines.forEach { line ->
                val (u, v) = line.trim().split(' ').map { it.toInt() }
                val indexOfFirstVert = vertexToIndexMap[u]!!
                val indexOfSecondVert = vertexToIndexMap[v]!!
                outNeighs[outOffs[indexOfFirstVert] + outCurrentPosition[indexOfFirstVert]] = indexOfSecondVert
                inNeighs[inOffs[indexOfSecondVert] + inCurrentPosition[indexOfSecondVert]] = indexOfFirstVert
                inCurrentPosition[indexOfSecondVert]++
                outCurrentPosition[indexOfFirstVert]++
            }
        }
        return CSRDirectedGraph(prevVertNumbers, outOffs, outNeighs, inOffs, inNeighs)
    }

    fun loadUndirectedGraph(originalFile: String): CSRUndirectedGraph{
        val processedFile = File(preprocessFile(originalFile, false))

        val vertexSet = mutableSetOf<Int>()
        processedFile.useLines { lines ->
            lines.forEach { line ->
                if(line.isBlank()) throw IllegalStateException("Line $line is empty")
                val parts = line.trim().split(' ')
                if(parts.size != 2) throw IllegalStateException("Invalid line: $line")
                val u = parts[0].toIntOrNull() ?: throw IllegalStateException("Invalid line: $line")
                val v = parts[1].toIntOrNull() ?: throw IllegalStateException("Invalid line: $line")
                vertexSet.add(u); vertexSet.add(v)
            }
        }
        val vertexCount = vertexSet.size
        val previousVertexArray = vertexSet.toIntArray()
        quickSort(previousVertexArray, 0, vertexCount - 1)
        val vertexToIndexMap = mutableMapOf<Int, Int>()
        for(i in 0 until vertexCount){
            vertexToIndexMap[previousVertexArray[i]] = i
        }

        var edgeCount = 0
        val vertexDegreeArray = IntArray(vertexCount)
        processedFile.useLines { lines ->
            lines.forEach { line ->
                val (u, v) = line.trim().split(' ').map{ it.toInt() }
                edgeCount++
                vertexDegreeArray[vertexToIndexMap[u]!!]++
                vertexDegreeArray[vertexToIndexMap[v]!!]++
            }
        }
        val offs = IntArray(vertexCount + 1)
        for(i in 1..vertexCount){
            offs[i] = offs[i - 1] + vertexDegreeArray[i - 1]
        }

        val neighs = IntArray(edgeCount * 2)
        val currentPosition = IntArray(vertexCount)
        processedFile.useLines { lines ->
            lines.forEach { line ->
                val (u, v) = line.trim().split(' ').map { it.toInt() }
                val indexOfFirstVert = vertexToIndexMap[u]!!
                val indexOfSecondVert = vertexToIndexMap[v]!!
                neighs[offs[indexOfFirstVert] + currentPosition[indexOfFirstVert]] = indexOfSecondVert
                currentPosition[indexOfFirstVert]++
                neighs[offs[indexOfSecondVert] + currentPosition[indexOfSecondVert]] = indexOfFirstVert
                currentPosition[indexOfSecondVert]++
            }
        }
        return CSRUndirectedGraph(previousVertexArray, offs, neighs)
    }
}
