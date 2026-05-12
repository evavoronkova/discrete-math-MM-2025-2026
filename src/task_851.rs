use std::collections::HashMap;

impl Solution {
    fn bfs_for_min(graph: &HashMap<i32, Vec<i32>>, start: &i32, quiet: &Vec<i32>) -> Option<i32> {
        let mut best_idx: Option<i32> = None;
        if !(graph.contains_key(start)) {
            return None;
        }
        for neighbour in &graph[start] {
            let val = Self::bfs_for_min(graph, neighbour, quiet);
            if let Some(res_idx) = val {
                best_idx = match best_idx {
                    None => Some(res_idx),
                    Some(current) => {
                        if quiet[res_idx as usize] < quiet[current as usize] {
                            Some(res_idx)
                        } else {
                            Some(current)
                        }
                    }
                };
            }
            best_idx = match best_idx {
                None => Some(*neighbour),
                Some(current) => {
                    if quiet[*neighbour as usize] < quiet[current as usize] {
                        Some(*neighbour)
                    } else {
                        Some(current)
                    }
                }
            };
        }
        best_idx
    }

    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for link in &richer {
            let a = link[0];
            let b = link[1];
            let mut vector = match graph.contains_key(&b) {
                true => graph[&b].clone(),
                false => Vec::new(),
            };

            vector.push(a);
            graph.insert(b, vector);
        }

        let mut answer: Vec<i32> = Vec::new();
        let n = quiet.len() as i32;
        for index in 0..n {
            let y = match Self::bfs_for_min(&graph, &index, &quiet) {
                Some(idx) => {
                    if quiet[idx as usize] < quiet[index as usize] {
                        idx
                    } else {
                        index
                    }
                }
                None => index,
            };
            answer.push(y);
        }

        answer
    }
}

pub struct Solution;
