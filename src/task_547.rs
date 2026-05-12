use std::collections::HashSet;

impl Solution {
    fn dfs(graph: &Vec<Vec<i32>>, start: usize, visited: &mut Vec<bool>, comp: &mut HashSet<i32>) {
        visited[start] = true;
        comp.insert(start as i32);
        for &neighbor in &graph[start] {
            if !visited[neighbor as usize] {
                Self::dfs(graph, neighbor as usize, visited, comp);
            }
        }
    }

    fn find_components(graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>) -> Vec<HashSet<i32>> {
        let mut components = Vec::new();

        for index in 0..graph.len() {
            if !visited[index] {
                let mut comp = HashSet::new();
                Self::dfs(graph, index, visited, &mut comp);
                components.push(comp);
            }
        }

        components
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<i32>> = Vec::new();
        for i in 0..is_connected.len() {
            let mut vector: Vec<i32> = Vec::new();
            for u in 0..is_connected.len() {
                if is_connected[i][u] == 1 && u != i {
                    vector.push(u as i32);
                }
            }
            graph.push(vector);
        }

        let mut visited = vec![false; graph.len()];
        let comps = Self::find_components(&graph, &mut visited);

        comps.len() as i32
    }
}

pub struct Solution;
