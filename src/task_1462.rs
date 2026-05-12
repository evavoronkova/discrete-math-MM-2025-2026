impl Solution {
    fn check_reachability(graph: &Vec<Vec<i32>>, start: i32, goal: i32) -> bool {
        for &i in &graph[start as usize] {
            if i == goal {
                return true;
            }
            if Self::check_reachability(graph, i, goal) == true {
                return true;
            }
        }
        false
    }
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); num_courses as usize];
        for link in &prerequisites {
            graph[link[0] as usize].push(link[1]);
        }
        let mut answer: Vec<bool> = Vec::new();
        for pair in &queries {
            answer.push(Self::check_reachability(&graph, pair[0], pair[1]));
        }

        answer
    }
}

pub struct Solution;
