use super::Solution;
use std::collections::HashMap;

struct Dsu {
    parent: Vec<u16>,
    size: Vec<u16>,
    components: u16,
}

impl Dsu {
    fn new(n: u16) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n as usize],
            components: n,
        }
    }

    fn find(&mut self, x: u16) -> u16 {
        let idx = x as usize;
        if self.parent[idx] != x {
            let root = self.find(self.parent[idx]);
            self.parent[idx] = root;
        }
        self.parent[idx]
    }

    fn union(&mut self, x: u16, y: u16) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.size[root_x as usize] < self.size[root_y as usize] {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.parent[root_y as usize] = root_x;
        self.size[root_x as usize] += self.size[root_y as usize];
        self.components -= 1;
        true
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut rows: HashMap<u16, u16> = HashMap::new();
        let mut cols: HashMap<u16, u16> = HashMap::new();
        let mut next_id: u16 = 0;

        for stone in &stones {
            rows.entry(stone[0] as u16).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
        }

        for stone in &stones {
            cols.entry(stone[1] as u16).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
        }

        let mut dsu = Dsu::new(next_id);
        for stone in &stones {
            let row = stone[0] as u16;
            let col = stone[1] as u16;
            dsu.union(rows[&row], cols[&col]);
        }

        stones.len() as i32 - dsu.components as i32
    }
}
