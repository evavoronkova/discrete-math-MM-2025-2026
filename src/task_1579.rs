impl Solution {
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) -> bool {
        let x_root = Self::find(parent, x);
        let y_root = Self::find(parent, y);
        if x_root == y_root {
            return false;
        }
        if size[x_root] < size[y_root] {
            parent[x_root] = y_root;
            size[y_root] += size[x_root];
        } else {
            parent[y_root] = x_root;
            size[x_root] += size[y_root];
        }
        true
    }

    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent_alice: Vec<usize> = (0..n).collect();
        let mut size_alice = vec![1; n];
        let mut parent_bob: Vec<usize> = (0..n).collect();
        let mut size_bob = vec![1; n];
        let mut counter = 0;

        for edge in &edges {
            if edge[0] == 3 {
                let u = edge[1] as usize - 1;
                let v = edge[2] as usize - 1;
                let united_alice = Self::union(&mut parent_alice, &mut size_alice, u, v);
                let united_bob = Self::union(&mut parent_bob, &mut size_bob, u, v);
                if united_alice || united_bob {
                    counter += 1;
                }
            }
        }

        for edge in &edges {
            if edge[0] == 1 {
                let u = edge[1] as usize - 1;
                let v = edge[2] as usize - 1;
                if Self::union(&mut parent_alice, &mut size_alice, u, v) {
                    counter += 1;
                }
            }
        }

        for edge in &edges {
            if edge[0] == 2 {
                let u = edge[1] as usize - 1;
                let v = edge[2] as usize - 1;
                if Self::union(&mut parent_bob, &mut size_bob, u, v) {
                    counter += 1;
                }
            }
        }

        let root_alice = Self::find(&mut parent_alice, 0);
        let root_bob = Self::find(&mut parent_bob, 0);
        for i in 0..n {
            if Self::find(&mut parent_alice, i) != root_alice
                || Self::find(&mut parent_bob, i) != root_bob
            {
                return -1;
            }
        }

        edges.len() as i32 - counter
    }
}

pub struct Solution;
