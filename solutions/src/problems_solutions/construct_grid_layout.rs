use super::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        if n == 1 {
            return vec![vec![0]];
        }

        let mut graph = vec![Vec::<usize>::new(); n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let corner = (0..n).min_by_key(|&i| graph[i].len()).unwrap();

        if graph[corner].len() == 0 {
            return vec![vec![corner as i32]];
        }

        let dist = bfs(&graph, corner);

        let mut line = vec![corner];
        let mut prev = corner;
        let mut cur = graph[corner][0];
        line.push(cur);

        loop {
            let cur_dist = dist[cur];
            let mut next_node = None;

            for &v in &graph[cur] {
                if v == prev {
                    continue;
                }
                if dist[v] != cur_dist + 1 {
                    continue;
                }

                let cnt_prev = graph[v].iter().filter(|&&u| dist[u] == cur_dist).count();

                if cnt_prev == 1 {
                    next_node = Some(v);
                    break;
                }
            }

            match next_node {
                Some(v) => {
                    prev = cur;
                    cur = v;
                    line.push(cur);
                }
                None => break,
            }
        }

        let mut used = vec![false; n];
        for &v in &line {
            used[v] = true;
        }

        let mut layout: Vec<Vec<usize>> = vec![line];

        while used.iter().any(|&x| !x) {
            let last = layout.last().unwrap();
            let mut next_line = Vec::with_capacity(last.len());

            for &u in last {
                let mut picked = None;
                for &v in &graph[u] {
                    if !used[v] {
                        picked = Some(v);
                        break;
                    }
                }

                if let Some(v) = picked {
                    next_line.push(v);
                }
            }

            if next_line.is_empty() {
                break;
            }

            for &v in &next_line {
                used[v] = true;
            }

            layout.push(next_line);
        }

        layout
            .into_iter()
            .map(|row| row.into_iter().map(|x| x as i32).collect())
            .collect()
    }
}

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];
    let mut q = VecDeque::new();

    dist[start] = 0;
    q.push_back(start);

    while let Some(u) = q.pop_front() {
        for &v in &graph[u] {
            if dist[v] == usize::MAX {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }

    dist
}
