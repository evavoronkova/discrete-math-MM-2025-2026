use super::Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn manhattan_distance(p1: &[i32], p2: &[i32]) -> i32 {
        (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
    }
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut heap = BinaryHeap::new();
        let mut visited = vec![false; n];
        let mut total_cost = 0;
        let mut used_points = 0;

        heap.push(Reverse((0, 0usize)));

        while let Some(Reverse((cost, point_idx))) = heap.pop() {
            if visited[point_idx] {
                continue;
            }

            visited[point_idx] = true;
            total_cost += cost;
            used_points += 1;

            if used_points == n {
                break;
            }

            for next_idx in 0..n {
                if !visited[next_idx] {
                    let dist = Solution::manhattan_distance(&points[point_idx], &points[next_idx]);
                    heap.push(Reverse((dist, next_idx)));
                }
            }
        }

        total_cost
    }
}
