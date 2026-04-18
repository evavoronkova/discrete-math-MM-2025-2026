use super::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let curr_x = 0;
        let curr_y = 0;
        let health = health - grid[curr_x as usize][curr_y as usize];
        if health <= 0 {
            return false;
        }

        let mut best_health: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
        queue.push_back((curr_x, curr_y, health));
        best_health[curr_x as usize][curr_y as usize] = health;
        while let Some((x, y, health)) = queue.pop_front() {
            if x as usize == grid.len() - 1 && y as usize == grid[0].len() - 1 {
                return true;
            }

            for (dx, dy) in directions.iter() {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0
                    && new_x < grid.len() as i32
                    && new_y >= 0
                    && new_y < grid[0].len() as i32
                {
                    let new_health = health - grid[new_x as usize][new_y as usize];
                    if new_health > 0 && new_health > best_health[new_x as usize][new_y as usize] {
                        best_health[new_x as usize][new_y as usize] = new_health;
                        queue.push_back((new_x, new_y, new_health));
                    }
                }
            }
        }
        false
    }
}
