use std::collections::VecDeque;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        let health = health - grid[0][0];
        if health <= 0 {
            return false;
        }

        let mut healths: Vec<Vec<i32>> = vec![vec![-1; m]; n];
        let vars = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();

        healths[0][0] = health;
        queue.push_back((0, 0, health));

        while let Some((current_x, current_y, current_health)) = queue.pop_front() {
            if current_x == n as i32 - 1 && current_y == m as i32 - 1 {
                return true;
            }
            for (dx, dy) in vars.iter() {
                let new_x = current_x + dx;
                let new_y = current_y + dy;
                if new_x >= 0 && new_y >= 0 && new_x < n as i32 && new_y < m as i32 {
                    let new_health = current_health - grid[new_x as usize][new_y as usize];
                    if new_health > 0 && new_health > healths[new_x as usize][new_y as usize] {
                        healths[new_x as usize][new_y as usize] = new_health;
                        queue.push_back((new_x, new_y, new_health));
                    }
                }
            }
        }

        false
    }
}

pub struct Solution;
