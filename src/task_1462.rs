impl Solution {
    fn check_reachability(
        graph: &Vec<Vec<i32>>,
        start: usize,
        reach_status: &mut Vec<Vec<bool>>,
        done: &mut Vec<bool>,
    ) {
        if done[start] {
            return;
        }
        done[start] = true;
        for &i in &graph[start] {
            reach_status[start][i as usize] = true;
            Self::check_reachability(graph, i as usize, reach_status, done);
            for u in 0..reach_status.len() {
                if reach_status[i as usize][u] == true {
                    reach_status[start][u] = reach_status[i as usize][u];
                }
            }
        }
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

        let mut reach_status: Vec<Vec<bool>> =
            vec![vec![false; num_courses as usize]; num_courses as usize];
        let mut done: Vec<bool> = vec![false; num_courses as usize];
        for i in 0..num_courses {
            Self::check_reachability(&graph, i as usize, &mut reach_status, &mut done);
        }
        let mut answer: Vec<bool> = Vec::new();
        for pair in &queries {
            answer.push(reach_status[pair[0] as usize][pair[1] as usize]);
        }

        answer
    }
}

pub struct Solution;
