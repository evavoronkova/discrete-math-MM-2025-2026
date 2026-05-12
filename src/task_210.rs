impl Solution {
    fn dfs(graph: &Vec<Vec<u32>>, list: &mut Vec<i32>, visited: &mut Vec<u8>, start: usize) {
        if visited[start] != 0 {
            return;
        }
        visited[start] = 1;
        for &i in &graph[start] {
            let v = i as usize;
            if visited[v] == 0 {
                Self::dfs(graph, list, visited, v);
                if visited[v] == 1 {
                    return;
                }
            } else if visited[v] == 1 {
                return;
            }
        }
        visited[start] = 2;
        list.push(start as i32);
    }
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: Vec<Vec<u32>> = vec![Vec::new(); num_courses as usize];
        for pair in &prerequisites {
            graph[pair[1] as usize].push(pair[0] as u32);
        }
        let mut visited: Vec<u8> = vec![0; num_courses as usize];
        let mut answer: Vec<i32> = Vec::new();
        for i in 0..num_courses as usize {
            Self::dfs(&graph, &mut answer, &mut visited, i);
        }
        if answer.len() != num_courses as usize {
            return vec![];
        }
        answer.into_iter().rev().collect()
    }
}
pub struct Solution;
