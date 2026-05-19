package core.algoritms

import kotlin.random.Random

fun quickSort(arrayOfInt: IntArray, left: Int, right: Int){
    if(left >= right) return
    val randomIndex = Random.nextInt(from = left, until = right + 1)
    val pivot = arrayOfInt[randomIndex]

    swap(arrayOfInt, randomIndex, right)
    var i = left
    for(j in left until right){
        if(arrayOfInt[j] <= pivot){
            swap(arrayOfInt, i, j)
            i++
        }
    }
    swap(arrayOfInt, i, right)
    quickSort(arrayOfInt, left, i - 1)
    quickSort(arrayOfInt, i + 1, right)
}

fun swap(arr: IntArray, ind1: Int, ind2: Int){
    val temp = arr[ind1]
    arr[ind1] = arr[ind2]
    arr[ind2] = temp
}
