class Solution {
    fun findAllRecipes(recipes: Array<String>, ingredients: List<List<String>>, supplies: Array<String>): List<String> {
        val result = mutableListOf<String>()
        val inDegree = mutableMapOf<String, Int>()
        val graph = mutableMapOf<String, MutableList<String>>()
        for (i in recipes.indices) {
            inDegree[recipes[i]] = ingredients[i].size
            for (ing in ingredients[i]) {
                graph.getOrPut(ing) { mutableListOf() }.add(recipes[i])
            }
        }
        val queue = ArrayDeque<String>()
        queue.addAll(supplies)
        while (queue.isNotEmpty()) {
            val item = queue.removeFirst()
            val dependentRecipes = graph[item] ?: continue
            for (recipe in dependentRecipes) {
                inDegree[recipe] = inDegree[recipe]!! - 1
                if (inDegree[recipe] == 0) {
                    result.add(recipe)
                    queue.add(recipe)
                }
            }
        }
        return result
    }
}