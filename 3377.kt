class Solution {
    fun minOperations(n: Int, m: Int): Int {
    val MAX = 10_000

    // Решето Эратосфена
    val prime = BooleanArray(MAX) { true }
    prime[0] = false
    prime[1] = false
    for (i in 2 until MAX) {
        if (prime[i]) {
            var j = i.toLong() * i
            while (j < MAX) { prime[j.toInt()] = false; j += i }
        }
    }

    // Если n или m простые — сразу -1
    if (prime[n] || prime[m]) {
        return -1
    }

    val digitCount = n.toString().length  // кол-во цифр (одинаково для n и m)

    val distance = IntArray(MAX) { Int.MAX_VALUE }
    distance[n] = n

    // priorityQueue: [стоимость, значение]
    val priorityQueue = java.util.PriorityQueue<IntArray>(
        Comparator { first, second -> first[0].compareTo(second[0]) }
    )
    priorityQueue.offer(intArrayOf(n, n))

    val digits = IntArray(digitCount)

    while (priorityQueue.isNotEmpty()) {
        val currentQueueElement = priorityQueue.poll()
        val cost = currentQueueElement[0]
        val currentNumber = currentQueueElement[1]

        if (cost > distance[currentNumber]) {
            continue
        }
        if (currentNumber == m) {
            return cost
        }

        // Разбиваем currentNumber на цифры
        var temporary = currentNumber
        for (i in digitCount - 1 downTo 0) {
            digits[i] = temporary % 10 
            temporary /= 10
        }

        for (i in 0 until digitCount) {
            val digit = digits[i]
            // Первая цифра многозначного числа не может стать 0
            val minDigit = if (i == 0 && digitCount > 1) 1 else 0

            for (delta in intArrayOf(-1, 1)) {
                val newDigit = digit + delta
                if (newDigit < minDigit || newDigit > 9) {
                    continue
                }

                digits[i] = newDigit
                var neighborNumber = 0
                for (x in digits) {
                    neighborNumber = neighborNumber * 10 + x
                }
                digits[i] = digit  // откатываем

                if (prime[neighborNumber]) {
                    continue // нельзя проходить через простые
                }

                val newCost = cost + neighborNumber
                if (newCost < distance[neighborNumber]) {
                    distance[neighborNumber] = newCost
                    priorityQueue.offer(intArrayOf(newCost, neighborNumber))
                }
            }
        }
    }

    if (distance[m] == Int.MAX_VALUE) {
        return -1
    } else {
        return distance[m]
    }
}
}
