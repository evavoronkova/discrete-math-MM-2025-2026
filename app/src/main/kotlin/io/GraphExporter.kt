package io

import core.model.Graph
import java.io.File

object GraphExporter {

    fun exportToEdgeList(graph: Graph, filename: String) {
        val sb = StringBuilder()
        for (v in 0 until graph.vertexCount) {
            for (nb in graph.neighbors(v)) {
                if (v < nb) {
                    sb.appendLine("$v $nb")
                }
            }
        }
        File(filename).writeText(sb.toString())
    }

    fun exportStatsToJson(stats: Any, filename: String) {
        val json = when (stats) {
            is Map<*, *> -> mapToJson(stats)
            else -> stats.toString()
        }
        File(filename).writeText(json)
    }

    private fun mapToJson(map: Map<*, *>): String {
        val entries = map.entries.joinToString(",\n  ") { (k, v) ->
            val key = "\"$k\""
            val value = when (v) {
                is Number -> v.toString()
                is String -> "\"$v\""
                is Map<*, *> -> mapToJson(v)
                else -> "\"$v\""
            }
            "$key: $value"
        }
        return "{\n  $entries\n}"
    }
}
