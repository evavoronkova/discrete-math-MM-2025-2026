import analysis.GraphAnalyzer
import benchmark.DistanceEstimatorQuality
import benchmark.DistanceEstimatorTester
import core.algorithms.ConnectedComponents
import core.model.Graph
import core.model.MutableVertexGraph
import experiments.DeletionExperiments
import io.GraphExporter
import io.GraphLoader
import landmarks.DistanceEstimator
import landmarks.LandmarkSelection
import landmarks.LandmarksBasic
import landmarks.LandmarksLCA
import utils.Statistics
import utils.Timer
import utils.Visualization

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        println("Usage: gradle run --args=\"<graph-file> [options]\"")
        println("Options:")
        println("  --directed               Load as directed graph")
        println("  --num-pairs=N            Number of vertices for distance estimation (default: 500)")
        println("  --snowball-size=N        Snowball sample target size (default: 500)")
        println("  --landmarks=N            Number of landmarks for Part 2 (default: 10)")
        println("  --landmark-strategy=S    Strategy: RANDOM, HIGH_DEGREE, COVERAGE (default: RANDOM)")
        println("  --part2                  Run Part 2 distance estimation")
        println("  --compare-landmarks      Compare all landmark strategies")
        return
    }

    val filePath = args[0]
    val isDirected = args.any { it == "--directed" }
    val numPairs = argInt(args, "--num-pairs=", 500)
    val snowballSize = argInt(args, "--snowball-size=", 500)
    val numLandmarks = argInt(args, "--landmarks=", 10)
    val runPart2 = args.any { it == "--part2" }
    val compareLandmarks = args.any { it == "--compare-landmarks" }
    val landmarkStrategy = argEnum(args, "--landmark-strategy=", LandmarkSelection.RANDOM)

    val loader = GraphLoader()
    val graph = if (isDirected) {
        loader.loadDirectedGraph(filePath)
    } else {
        loader.loadUndirectedGraph(filePath)
    }

    println("Loaded graph: ${graph.vertexCount} vertices, ${graph.edgeCount} edges")

    runPart1Analysis(graph, isDirected, numPairs, snowballSize)

    runPartBExperiments(graph)

    if (runPart2 || compareLandmarks) {
        runPart2DistanceEstimation(graph, numLandmarks, landmarkStrategy, compareLandmarks)
    }

    println("\nDone.")
}

private fun argInt(args: Array<String>, prefix: String, default: Int): Int {
    val arg = args.firstOrNull { it.startsWith(prefix) } ?: return default
    return arg.removePrefix(prefix).toIntOrNull() ?: default
}

private fun argEnum(args: Array<String>, prefix: String, default: LandmarkSelection): LandmarkSelection {
    val arg = args.firstOrNull { it.startsWith(prefix) } ?: return default
    return try {
        LandmarkSelection.valueOf(arg.removePrefix(prefix).uppercase())
    } catch (e: IllegalArgumentException) {
        default
    }
}

private fun runPart1Analysis(graph: Graph, isDirected: Boolean, numPairs: Int, snowballSize: Int) {
    println("\n" + "=".repeat(60))
    println("PART 1: NETWORK STRUCTURE ANALYSIS")
    println("=".repeat(60))

    val analyzer = GraphAnalyzer(graph)

    println("\n--- A.1 Basic characteristics ---")
    val basic = analyzer.computeBasicStats(isDirected)
    println(basic)

    println("\n--- A.2 Distance estimation (numPairs=$numPairs, snowball=$snowballSize) ---")
    val wcc = ConnectedComponents.weaklyConnected(graph)
    val largestComp = wcc.largestComponent
    println("Largest WCC size: ${largestComp.size}")
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

    Visualization.exportDegreeDistribution(degrees, "degree_dist.csv")
    Visualization.exportLogLogDegree(degrees, "degree_loglog.csv")
}

private fun runPartBExperiments(graph: Graph) {
    println("\n" + "=".repeat(60))
    println("PART B: ROBUSTNESS ANALYSIS")
    println("=".repeat(60))

    val mutableGraph = graph as? MutableVertexGraph
    val experiments = DeletionExperiments(graph, mutableGraph)
    val percentages = listOf(0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50)

    println("\n--- B.1 Random node deletion ---")
    val randomResults = experiments.randomDeletion(percentages)
    experiments.printResults(randomResults, "Random deletion")
    Visualization.exportDeletionResults(randomResults, "deletion_random.csv")

    println("\n--- B.2 High-degree node deletion ---")
    val highDegResults = experiments.highDegreeDeletion(percentages)
    experiments.printResults(highDegResults, "High-degree deletion")
    Visualization.exportDeletionResults(highDegResults, "deletion_high_degree.csv")
}

private fun runPart2DistanceEstimation(
    graph: Graph,
    numLandmarks: Int,
    strategy: LandmarkSelection,
    compareAll: Boolean
) {
    println("\n" + "=".repeat(60))
    println("PART 2: DISTANCE ESTIMATION (LANDMARKS)")
    println("=".repeat(60))

    if (compareAll) {
        compareDistanceAlgorithms(graph, numLandmarks)
    } else {
        println("\n--- Landmarks-Basic ($strategy, landmarks=$numLandmarks) ---")
        runEstimator(graph, LandmarksBasic(graph, numLandmarks, strategy), "Basic")

        println("\n--- Landmarks-LCA ($strategy, landmarks=$numLandmarks) ---")
        runEstimator(graph, LandmarksLCA(graph, numLandmarks, strategy), "LCA")
    }
}

private fun runEstimator(graph: Graph, estimator: DistanceEstimator, label: String) {
    val preprocessTime = Timer.measure { estimator.preprocess() }
    println("  Preprocess time: ${"%.2f".format(preprocessTime / 1_000_000.0)} ms")

    val tester = DistanceEstimatorTester(graph, estimator)
    val numTestPairs = 200
    val queryTime = Timer.measure {
        val quality = tester.test(numTestPairs)
        println("  Quality: $quality")
    }
    println("  Query time ($numTestPairs pairs): ${"%.2f".format(queryTime / 1_000_000.0)} ms")
}

private fun compareDistanceAlgorithms(graph: Graph, numLandmarks: Int) {
    val strategies = LandmarkSelection.values()
    val algorithms = listOf<(LandmarkSelection) -> DistanceEstimator>(
        { s -> LandmarksBasic(graph, numLandmarks, s) },
        { s -> LandmarksLCA(graph, numLandmarks, s) }
    )
    val algoNames = listOf("Landmarks-Basic", "Landmarks-LCA")

    println("\n--- Landmark Strategy Comparison ---")
    println("%-18s | %-12s | %-10s | %-10s | %-8s | %-8s".format(
        "Algorithm", "Strategy", "AvgError", "MaxError", "Exact%", "PrepTime(ms)"
    ))
    println("-".repeat(80))

    for ((algoIdx, algoFactory) in algorithms.withIndex()) {
        for (strategy in strategies) {
            val estimator = algoFactory(strategy)
            val prepTime = Timer.measure { estimator.preprocess() } / 1_000_000.0

            val tester = DistanceEstimatorTester(graph, estimator)
            val quality = tester.test(200)

            println("%-18s | %-12s | %-10.3f | %-10d | %-7.1f%% | %-8.1f".format(
                algoNames[algoIdx],
                strategy.name,
                quality.avgError,
                quality.maxError,
                quality.exactMatchRate * 100,
                prepTime
            ))
        }
    }
}
