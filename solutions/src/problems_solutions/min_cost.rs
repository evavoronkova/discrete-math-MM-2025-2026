use super::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut costs = vec![vec![i32::MAX; n]; m];
        let DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut double_queue = VecDeque::new();
        costs[0][0] = 0;
        double_queue.push_back((0, 0, 0));
        #[inline]
        fn is_valid(x: i32, y: i32, m: usize, n: usize) -> bool {
            x >= 0 && x < m as i32 && y >= 0 && y < n as i32
        }
        while let Some((curr_x, curr_y, cost)) = double_queue.pop_front() {
            for (index, (dx, dy)) in DIRECTIONS.iter().enumerate() {
                let new_x = curr_x + dx;
                let new_y = curr_y + dy;
                if is_valid(new_x, new_y, m, n) {
                    let is_zero = grid[curr_x as usize][curr_y as usize] == (index as i32 + 1);
                    let new_cost = cost + (!is_zero) as i32;

                    if new_cost < costs[new_x as usize][new_y as usize] {
                        costs[new_x as usize][new_y as usize] = new_cost;

                        let push = if is_zero {
                            VecDeque::push_front
                        } else {
                            VecDeque::push_back
                        };

                        push(&mut double_queue, (new_x, new_y, new_cost));
                    }
                }
            }
        }
        costs[m - 1][n - 1]
    }
}
