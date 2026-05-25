import analysis.GraphAnalyzer
import core.algorithms.ConnectedComponents
import core.model.Graph
import core.model.MutableVertexGraph
import core.storage.CSRUndirectedGraph
import experiments.DeletionExperiments
import io.GraphLoader

fun main(args: Array<String>) {
    if (args.size < 1) {
        println("Usage: gradle run --args=\"<graph-file> [--directed]\"")
        return
    }

    val filePath = args[0]
    val isDirected = args.size > 1 && args[1] == "--directed"

    val loader = GraphLoader()
    val graph = if (isDirected) {
        loader.loadDirectedGraph(filePath)
    } else {
        loader.loadUndirectedGraph(filePath)
    }

    println("Loaded graph: ${graph.vertexCount} vertices, ${graph.edgeCount} edges")

    runPart1Analysis(graph, isDirected)

    runPartBExperiments(graph)

    println("\nDone.")
}

private fun runPart1Analysis(graph: Graph, isDirected: Boolean) {
    println("\n" + "=" .repeat(60))
    println("PART 1: NETWORK STRUCTURE ANALYSIS")
    println("=" .repeat(60))

    val analyzer = GraphAnalyzer(graph)

    println("\n--- A.1 Basic characteristics ---")
    val basic = analyzer.computeBasicStats(isDirected)
    println(basic)

    println("\n--- A.2 Distance estimation ---")
    val wcc = ConnectedComponents.weaklyConnected(graph)
    val largestComp = wcc.largestComponent
    println("Largest WCC size: ${largestComp.size}")
    val numPairs = 500
    val snowballSize = 500
    val diamStats = analyzer.estimateDiameter(largestComp, numPairs, snowballSize)
    println(diamStats)

    println("\n--- A.3 Clustering statistics (whole graph) ---")
    val clustering = analyzer.computeClusteringStats()
    println(clustering)

    println("\n--- A.4 Clustering statistics (largest WCC) ---")
    val wccClustering = analyzer.computeClusteringStatsInComponent(largestComp)
    println(wccClustering)

    println("\n--- A.5 Degree statistics ---")
    val degrees = analyzer.computeDegreeStats()
    println(degrees)
}

private fun runPartBExperiments(graph: Graph) {
    println("\n" + "=" .repeat(60))
    println("PART B: ROBUSTNESS ANALYSIS")
    println("=" .repeat(60))

    val mutableGraph = when (graph) {
        is MutableVertexGraph -> graph
        is CSRUndirectedGraph -> graph
        else -> null
    }
    val mutForExp = if (mutableGraph is MutableVertexGraph) mutableGraph else null

    val experiments = DeletionExperiments(graph, mutForExp)
    val percentages = listOf(0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50)

    println("\n--- B.1 Random node deletion ---")
    val randomResults = experiments.randomDeletion(percentages)
    experiments.printResults(randomResults, "Random deletion")

    println("\n--- B.2 High-degree node deletion ---")
    val highDegResults = experiments.highDegreeDeletion(percentages)
    experiments.printResults(highDegResults, "High-degree deletion")
}
