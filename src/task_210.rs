impl Solution {
    fn dfs(graph: &Vec<Vec<i32>>, list: &mut Vec<i32>, visited: &mut Vec<i32>, start: usize) {
        if visited[start] != -1 {
            return;
        }
        visited[start] = 0;
        for &i in &graph[start] {
            if visited[i as usize] == -1 {
                Self::dfs(graph, list, visited, i as usize);
                if visited[i as usize] == -1 {
                    return;
                }
            } else if visited[i as usize] == 0 {
                return;
            }
        }
        visited[start] = 1;
        list.push(start as i32);
    }
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); num_courses as usize];
        for pair in &prerequisites {
            graph[pair[1] as usize].push(pair[0]);
        }
        let mut visited = vec![-1; num_courses as usize];
        let mut answer: Vec<i32> = Vec::new();
        for i in 0..num_courses {
            Self::dfs(&graph, &mut answer, &mut visited, i as usize);
        }

        if answer.len() != num_courses as usize {
            return vec![];
        }
        answer.reverse();
        answer
    }
}
pub struct Solution;
