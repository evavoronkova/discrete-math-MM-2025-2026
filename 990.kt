class Solution {
    fun equationsPossible(equations: Array<String>): Boolean {
        // Система непересекающихся множеств на 26 элементах, изначально каждая буква сама себе родитель
        val parent = IntArray(26) { index -> index }
        // find со сжатием пути: находим корень, потом подвешиваем все промежуточные напрямую к корню
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

        // Объединяем два класса: корень одного делаем родителем корня другого
        fun union(first: Int, second: Int) {
            parent[find(first)] = find(second)
        }
        // Проход 1: обрабатываем '==', сначала все равенства, потом проверки
        for (equation in equations) {
            if (equation[1] == '=') {
                union(equation[0] - 'a', equation[3] - 'a')
            }
        }
        // Проход 2: проверяем '!=', буквы должны лежать в разных классах, иначе противоречие
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
