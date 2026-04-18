use super::Solution;
use std::cmp::Reverse;
impl Solution {
    fn dijkstra(graph: &Vec<Vec<(i32, i32)>>, start: i32) -> Vec<i64> {
        let mut dist = vec![i64::MAX; graph.len()];
        dist[start as usize] = 0;
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(Reverse((0, start)));
        while let Some(Reverse((cost, node))) = heap.pop() {
            if cost > dist[node as usize] {
                continue;
            }
            for &(neighbor, weight) in &graph[node as usize] {
                let new_cost = cost + weight as i64;
                if new_cost < dist[neighbor as usize] {
                    dist[neighbor as usize] = new_cost;
                    heap.push(Reverse((new_cost, neighbor)));
                }
            }
        }
        dist
    }
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let mut graph = vec![vec![]; n as usize];
        let mut rev_graph = vec![vec![]; n as usize];
        for edge in edges.iter() {
            graph[edge[0] as usize].push((edge[1], edge[2]));
            rev_graph[edge[1] as usize].push((edge[0], edge[2]));
        }
        let dist_src1 = Self::dijkstra(&graph, src1);
        let dist_src2 = Self::dijkstra(&graph, src2);
        let dist_dest = Self::dijkstra(&rev_graph, dest);
        let mut ans = i64::MAX;
        for rand_node in 0..n as usize {
            if dist_src1[rand_node] == i64::MAX
                || dist_src2[rand_node] == i64::MAX
                || dist_dest[rand_node] == i64::MAX
            {
                continue;
            } else {
                let total_cost = dist_src1[rand_node] + dist_src2[rand_node] + dist_dest[rand_node];
                ans = ans.min(total_cost);
            }
        }
        ans
    }
}
