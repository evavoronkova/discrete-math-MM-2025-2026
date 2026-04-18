use super::Solution;

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let mut graph = vec![vec![]; patience.len()];
        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        let mut dist = vec![i32::MAX; patience.len()];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        dist[0] = 0;
        while let Some(node) = queue.pop_front() {
            for &neighbor in graph[node].iter() {
                if dist[neighbor] == i32::MAX {
                    dist[neighbor] = dist[node] + 1;
                    queue.push_back(neighbor);
                }
            }
        }
        let mut max_time = 0;
        for i in 1..patience.len() {
            let last_send = (((dist[i] << 1) - 1) / patience[i]) * patience[i];
            max_time = max_time.max(last_send + dist[i] * 2);
        }
        max_time + 1
    }
}
