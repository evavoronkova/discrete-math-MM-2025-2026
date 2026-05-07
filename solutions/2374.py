def edgeScore(edges):
    n = len(edges)
    scores = [0] * n

    for i in range(n):
        scores[edges[i]] += i


    best_node = 0
    res = 0
    for j in range(len(scores)):
        if scores[j] > best_node:
            best_node = scores[j]
            res = j

    return res
