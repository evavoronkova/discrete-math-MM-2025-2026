class Solution {
    fun equationsPossible(equations: Array<String>): Boolean {
        // parent для 26 букв, инициализируем как самих себя
        val parent = IntArray(26) { index -> index }
        fun find(node: Int): Int {
            var root = node
            while (parent[root] != root) {
                root = parent[root]
            }
            var current = node
            while (current != root) {
                val next = parent[current]
                parent[current] = root
                current = next
            }
            return root
        }

        fun union(first: Int, second: Int) {
            parent[find(first)] = find(second)
        }
        // Проход 1: обрабатываем только '=='
        for (equation in equations) {
            if (equation[1] == '=') {
                union(equation[0] - 'a', equation[3] - 'a')
            }
        }
        // Проход 2: проверяем '!='
        for (equation in equations) {
            if (equation[1] == '!') {
                if (find(equation[0] - 'a') == find(equation[3] - 'a')) {
                    return false
                }
            }
        }
        return true
    }
}
