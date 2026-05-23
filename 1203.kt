class Solution {
    fun sortItems(n: Int, m: Int, group: IntArray, beforeItems: List<List<Int>>): IntArray {
        // Каждый элемент без группы (-1) получает свой уникальный ID группы, чтобы разбить их на отдельные "группы" из одного элемента.
        var nextGroupId = m
        val itemGroupId = IntArray(n) { index ->
            if (group[index] == -1) {
                nextGroupId++
            } else {
                group[index]
            }
        }
        val totalGroups = nextGroupId

        // Списки смежности и массивы входящих степеней для элементов и групп
        val itemAdjacency = Array(n) { mutableListOf<Int>() }
        val groupAdjacency = Array(totalGroups) { mutableListOf<Int>() }
        val itemInDegree = IntArray(n)
        val groupInDegree = IntArray(totalGroups)

        // Множество для дедупликации рёбер между группами (пару кодируем одним Long)
        val addedGroupEdges = HashSet<Long>()
        for (currentItem in 0 until n) {
            val currentGroup = itemGroupId[currentItem]
            for (predecessorItem in beforeItems[currentItem]) {
                val predecessorGroup = itemGroupId[predecessorItem]
                if (predecessorGroup == currentGroup) {
                    // Внутригрупповое ребро - добавляем в граф элементов
                    itemAdjacency[predecessorItem].add(currentItem)
                    itemInDegree[currentItem]++
                } else {
                    // Межгрупповое ребро - добавляем в граф групп (без дубликатов)
                    val edgeKey = predecessorGroup.toLong() * totalGroups + currentGroup
                    if (addedGroupEdges.add(edgeKey)) {
                        groupAdjacency[predecessorGroup].add(currentGroup)
                        groupInDegree[currentGroup]++
                    }
                }
            }
        }

        // Отображение: группа -> список её элементов
        val groupToItems = Array(totalGroups) { mutableListOf<Int>() }
        for (itemIndex in 0 until n) {
            groupToItems[itemGroupId[itemIndex]].add(itemIndex)
        }

        // Топологическая сортировка групп (алгоритм Кана), если цикл - пустой массив
        val sortedGroups = topologicalSort(totalGroups, groupAdjacency, groupInDegree)
            ?: return IntArray(0)

        // Собираем результат: для каждой группы топосортируем её элементы
        val result = IntArray(n)
        var resultPointer = 0
        for (currentGroup in sortedGroups) {
            val itemsInCurrentGroup = groupToItems[currentGroup]
            if (itemsInCurrentGroup.isEmpty()) {
                continue
            }
            val sortedItems = topologicalSortSubset(itemsInCurrentGroup, itemAdjacency, itemInDegree)
                ?: return IntArray(0)
            for (item in sortedItems) {
                result[resultPointer++] = item
            }
        }
        return result
    }

    // Топосортировка всех узлов от 0 до nodeCount-1 (алгоритм Кана: BFS по нулевым in-degree)
    fun topologicalSort(
        nodeCount: Int,
        adjacency: Array<MutableList<Int>>,
        inDegree: IntArray
    ): IntArray? {
        val queue = ArrayDeque<Int>(nodeCount)
        // стартуем со всех узлов с нулевой входящей степенью
        for (node in 0 until nodeCount) {
            if (inDegree[node] == 0) {
                queue.addLast(node)
            }
        }
        val sortedNodes = IntArray(nodeCount)
        var processedCount = 0
        while (queue.isNotEmpty()) {
            val currentNode = queue.removeFirst()
            sortedNodes[processedCount++] = currentNode
            // "удаляем" узел: уменьшаем in-degree у соседей
            for (neighborNode in adjacency[currentNode]) {
                if (--inDegree[neighborNode] == 0) {
                    queue.addLast(neighborNode)
                }
            }
        }
        // обработали меньше узлов - был цикл
        return if (processedCount == nodeCount) {
            sortedNodes
        } else {
            null
        }
    }

    // Топосортировка подмножества узлов, тот же inDegree но смотрим только на подмножество
    fun topologicalSortSubset(
        nodes: List<Int>,
        adjacency: Array<MutableList<Int>>,
        inDegree: IntArray
    ): List<Int>? {
        val queue = ArrayDeque<Int>(nodes.size)
        for (node in nodes) {
            if (inDegree[node] == 0) {
                queue.addLast(node)
            }
        }
        val sortedNodes = ArrayList<Int>(nodes.size)
        while (queue.isNotEmpty()) {
            val currentNode = queue.removeFirst()
            sortedNodes.add(currentNode)
            for (neighborNode in adjacency[currentNode]) {
                if (--inDegree[neighborNode] == 0) {
                    queue.addLast(neighborNode)
                }
            }
        }
        return if (sortedNodes.size == nodes.size) {
            sortedNodes
        } else {
            null
        }
    }
}
