from typing import List
from itertools import combinations

class Solution:
    def minNumberOfSemesters(self, n: int, relations: List[List[int]], k: int) -> int:
        prereq = [0] * n

        for prev, next_course in relations:
            prev -= 1
            next_course -= 1
            prereq[next_course] |= (1 << prev)

        total_masks = 1 << n
        dp = [10 ** 9] * total_masks
        dp[0] = 0

        for mask in range(total_masks):
            if dp[mask] == 10 ** 9:
                continue

            available = []

            for course in range(n):
                if mask & (1 << course):
                    continue

                if (prereq[course] & mask) == prereq[course]:
                    available.append(course)

            if len(available) <= k:
                new_mask = mask

                for course in available:
                    new_mask |= (1 << course)

                dp[new_mask] = min(dp[new_mask], dp[mask] + 1)

            else:
                for chosen_courses in combinations(available, k):
                    new_mask = mask

                    for course in chosen_courses:
                        new_mask |= (1 << course)

                    dp[new_mask] = min(dp[new_mask], dp[mask] + 1)

        return dp[total_masks - 1]