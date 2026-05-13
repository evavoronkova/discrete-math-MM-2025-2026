use std::collections::HashMap;

impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let mut target_grid = target_grid;
        let mut hashmap: HashMap<i32, (u32, u32, u32, u32)> = HashMap::new();
        let n = target_grid.len();
        let m = target_grid[0].len();
        for i in 0..n {
            for j in 0..m {
                if hashmap.contains_key(&target_grid[i][j]) {
                    let mut new_i_min = hashmap[&target_grid[i][j]].0;
                    let mut new_i_max = hashmap[&target_grid[i][j]].1;
                    let mut new_j_min = hashmap[&target_grid[i][j]].2;
                    let mut new_j_max = hashmap[&target_grid[i][j]].3;
                    if new_i_min > i as u32 {
                        new_i_min = i as u32;
                    }
                    if new_i_max < i as u32 {
                        new_i_max = i as u32;
                    }
                    if new_j_min > j as u32 {
                        new_j_min = j as u32;
                    }
                    if new_j_max < j as u32 {
                        new_j_max = j as u32;
                    }
                    hashmap.insert(
                        target_grid[i][j],
                        (new_i_min, new_i_max, new_j_min, new_j_max),
                    );
                } else {
                    hashmap.insert(target_grid[i][j], (i as u32, i as u32, j as u32, j as u32));
                }
            }
        }
        for _ in 0..hashmap.keys().len() {
            for key in hashmap.keys() {
                let i_min = hashmap[key].0;
                let i_max = hashmap[key].1;
                let j_min = hashmap[key].2;
                let j_max = hashmap[key].3;
                let mut flag = 0;

                for i in i_min..=i_max {
                    if flag == 1 {
                        break;
                    }
                    for j in j_min..=j_max {
                        if target_grid[i as usize][j as usize] != *key
                            && target_grid[i as usize][j as usize] != 0
                        {
                            flag = 1;
                            break;
                        }
                    }
                }

                if flag == 0 {
                    for i in i_min..=i_max {
                        for j in j_min..=j_max {
                            target_grid[i as usize][j as usize] = 0;
                        }
                    }
                }
            }
            if target_grid == vec![vec![0; m]; n] {
                return true;
            }
        }
        false
    }
}

pub struct Solution;
