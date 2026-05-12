impl Solution {
    fn check_reachability(graph: &Vec<Vec<i32>>, start: i32, reach_status: &mut Vec<Vec<bool>>) {
        for &i in &graph[start as usize] {
            reach_status[start as usize][i as usize] = true;
            Self::check_reachability(graph, i, reach_status);
            for u in 0..reach_status.len() {
                if reach_status[i as usize][u] == true {
                    reach_status[start as usize][u] = reach_status[i as usize][u];
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
        for i in 0..num_courses {
            Self::check_reachability(&graph, i, &mut reach_status);
        }
        let mut answer: Vec<bool> = Vec::new();
        for pair in &queries {
            answer.push(reach_status[pair[0] as usize][pair[1] as usize]);
        }

        answer
    }
}

pub struct Solution;
