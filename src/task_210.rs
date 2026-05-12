impl Solution {
    fn kahn(graph: &Vec<Vec<u32>>, indegrees: &mut Vec<u16>, n: usize) -> Vec<i32> {
        let mut queue = std::collections::VecDeque::new();
        for i in 0..n {
            if indegrees[i] == 0 {
                queue.push_back(i);
            }
        }
        let mut answer: Vec<i32> = Vec::with_capacity(n);
        while let Some(v) = queue.pop_front() {
            answer.push(v as i32);
            for &u in &graph[v] {
                let u = u as usize;
                indegrees[u] -= 1;
                if indegrees[u] == 0 {
                    queue.push_back(u);
                }
            }
        }
        if answer.len() == n { answer } else { vec![] }
    }

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut graph: Vec<Vec<u32>> = vec![Vec::new(); n];
        let mut indegrees: Vec<u16> = vec![0; n];

        for pair in &prerequisites {
            let u = pair[0] as u32;
            let v = pair[1] as u32;
            graph[v as usize].push(u);
            indegrees[u as usize] += 1;
        }

        Self::kahn(&graph, &mut indegrees, n)
    }
}
pub struct Solution;
