use std::collections::VecDeque;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;
        let health = health - grid[0][0];

        let mut healths = vec![vec![0; m as usize]; n as usize];
        let mut queue: VecDeque<Vec<i32>> = VecDeque::new();
        healths[0][0] = health;
        queue.push_back(vec![0, 0, health]);
        while let Some(info) = queue.pop_front() {
            let current_x = info[0];
            let current_y = info[1];
            let current_health = info[2];
            if current_x == n - 1 && current_y == m - 1 {
                return true;
            }

            if current_x - 1 >= 0 {
                let new_health = current_health - grid[current_x as usize - 1][current_y as usize];
                if new_health > 0
                    && new_health > healths[current_x as usize - 1][current_y as usize]
                {
                    queue.push_back(vec![current_x - 1, current_y, new_health]);
                    healths[current_x as usize - 1][current_y as usize] = new_health;
                }
            }
            if current_y - 1 >= 0 {
                let new_health = current_health - grid[current_x as usize][current_y as usize - 1];
                if new_health > 0
                    && new_health > healths[current_x as usize][current_y as usize - 1]
                {
                    queue.push_back(vec![current_x, current_y - 1, new_health]);
                    healths[current_x as usize][current_y as usize - 1] = new_health;
                }
            }
            if current_x + 1 < n {
                let new_health = current_health - grid[current_x as usize + 1][current_y as usize];
                if new_health > 0
                    && new_health > healths[current_x as usize + 1][current_y as usize]
                {
                    queue.push_back(vec![current_x + 1, current_y, new_health]);
                    healths[current_x as usize + 1][current_y as usize] = new_health;
                }
            }
            if current_y + 1 < m {
                let new_health = current_health - grid[current_x as usize][current_y as usize + 1];
                if new_health > 0
                    && new_health > healths[current_x as usize][current_y as usize + 1]
                {
                    queue.push_back(vec![current_x, current_y + 1, new_health]);
                    healths[current_x as usize][current_y as usize + 1] = new_health;
                }
            }
        }

        false
    }
}

pub struct Solution;
