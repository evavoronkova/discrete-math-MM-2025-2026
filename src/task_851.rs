impl Solution {
    fn bfs_for_min(
        graph: &Vec<Vec<i32>>,
        start: i32,
        quiet: &Vec<i32>,
        memo: &mut Vec<Option<i32>>,
    ) -> i32 {
        let u = start as usize;
        if let Some(cached) = memo[u] {
            return cached;
        }

        let mut best_idx = start;
        for &neighbour in &graph[u] {
            let candidate = Self::bfs_for_min(graph, neighbour, quiet, memo);
            if quiet[candidate as usize] < quiet[best_idx as usize] {
                best_idx = candidate;
            }
        }

        memo[u] = Some(best_idx);
        best_idx
    }

    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n];
        for link in &richer {
            let a = link[0] as usize;
            let b = link[1] as usize;
            graph[b].push(a as i32);
        }

        let mut memo: Vec<Option<i32>> = vec![None; n];
        let mut answer: Vec<i32> = Vec::with_capacity(n);
        for index in 0..n {
            let y = Self::bfs_for_min(&graph, index as i32, &quiet, &mut memo);
            answer.push(y);
        }

        answer
    }
}

pub struct Solution;
