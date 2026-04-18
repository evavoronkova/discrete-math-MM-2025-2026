use super::Solution;

impl Solution {
    fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, current: usize, n: usize) {
        visited[current] = true;
        for next in 0..n {
            if is_connected[current][next] == 1 && !visited[next] {
                Solution::dfs(&is_connected, visited, next, n);
            }
        }
    }
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut count = 0;
        for i in 0..n {
            if !visited[i] {
                count += 1;
                Solution::dfs(&is_connected, &mut visited, i, n);
            }
        }
        count
    }
}
