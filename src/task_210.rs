use std::collections::VecDeque;

impl Solution {
    fn check_reachability(
        graph: &Vec<Vec<i32>>,
        start: usize,
        reach_status: &mut Vec<Vec<bool>>,
        visited: &mut Vec<bool>,
    ) {
        if visited[start] {
            return;
        }
        visited[start] = true;
        reach_status[start][start] = true;
        for &i in &graph[start] {
            reach_status[start][i as usize] = true;
            Self::check_reachability(graph, i as usize, reach_status, visited);
            for u in 0..reach_status.len() {
                if reach_status[i as usize][u] == true {
                    reach_status[start][u] = reach_status[i as usize][u];
                }
            }
        }
    }
    fn bfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, start: i32, list: &mut Vec<i32>) {
        let mut queue: VecDeque<i32> = VecDeque::new();
        list.push(start);
        visited[start as usize] = true;
        queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            for &to in &graph[v as usize] {
                if !visited[to as usize] {
                    visited[to as usize] = true;
                    queue.push_back(to);
                    list.push(to);
                }
            }
        }
    }
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); num_courses as usize];
        for pair in &prerequisites {
            graph[pair[1] as usize].push(pair[0]);
        }
        let mut visited = vec![false; num_courses as usize];
        let mut is_connected = vec![vec![false; num_courses as usize]; num_courses as usize];
        for i in 0..num_courses {
            Self::check_reachability(&graph, i as usize, &mut is_connected, &mut visited);
        }
        let mut answer: Vec<i32> = Vec::new();
        for i in &is_connected {
            for u in i {
                print!("{} ", u);
            }
            println!();
        }
        for i in 0..num_courses as usize {
            if is_connected[i] == vec![true; num_courses as usize] {
                visited = vec![false; num_courses as usize];
                Self::bfs(&mut graph, &mut visited, i as i32, &mut answer);
            }
        }
        answer
    }
}
pub struct Solution;
