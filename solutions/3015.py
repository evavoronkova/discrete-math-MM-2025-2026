def countOfPairs(n: int, x: int, y: int) -> list[int]:
    ans = [0] * n
    for i in range(1, n + 1):
        for j in range(i + 1, n + 1):
            dist = min(
                j - i,
                abs(i - x) + 1 + abs(j - y),
                abs(i - y) + 1 + abs(j - x)
            )
            ans[dist - 1] += 2
    return ans
