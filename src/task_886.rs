impl Solution {
    fn dfs(v: usize, color: i32, graph: &Vec<Vec<i32>>, colors: &mut Vec<i32>) -> bool {
        colors[v] = color;
        for &to in &graph[v] {
            if colors[to as usize] == -1 {
                if !Self::dfs(to as usize, 1 - color, graph, colors) {
                    return false
                }
            } else if colors[to as usize] == color {
                    return false
            }
        }
        return true
    }



    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];

        for i in 0..dislikes.len() {
            graph[dislikes[i][0] as usize - 1].push(dislikes[i][1] - 1);
            graph[dislikes[i][1] as usize - 1].push(dislikes[i][0] - 1);
        }

        let mut colors = vec![-1; n as usize];

        for i in 0..(n as usize) {
            if colors[i] == -1 {
                if !Self::dfs(i, 0, &graph, &mut colors) {
                    return false
                }
            }
        }

        return true
    }
}

pub struct Solution;