use super::Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();

        let mut dist = vec![i32::MAX; n * m * 2];
        let mut pq = BinaryHeap::new();

        dist[0] = 0;
        pq.push((Reverse(0), 0u16, 0u16, 0u8));

        while let Some((Reverse(t), x, y, p)) = pq.pop() {
            let x = x as usize;
            let y = y as usize;
            let p = p as usize;

            let base = (x * m + y) << 1;
            let cur_idx = base | p;

            if x == n - 1 && y == m - 1 {
                return t;
            }

            if t > dist[cur_idx] {
                continue;
            }

            let cost = (p + 1) as i32;
            let np = 1 - p;

            if x + 1 < n {
                let nx = x + 1;
                let ny = y;

                let mt = unsafe { *move_time.get_unchecked(nx).get_unchecked(ny) };
                let start = if t > mt { t } else { mt };
                let new_time = start + cost;

                let i = ((nx * m + ny) << 1) | np;

                if new_time < dist[i] {
                    dist[i] = new_time;
                    pq.push((Reverse(new_time), nx as u16, ny as u16, np as u8));
                }
            }

            if x > 0 {
                let nx = x - 1;
                let ny = y;

                let mt = unsafe { *move_time.get_unchecked(nx).get_unchecked(ny) };
                let start = if t > mt { t } else { mt };
                let new_time = start + cost;

                let i = ((nx * m + ny) << 1) | np;

                if new_time < dist[i] {
                    dist[i] = new_time;
                    pq.push((Reverse(new_time), nx as u16, ny as u16, np as u8));
                }
            }

            if y + 1 < m {
                let nx = x;
                let ny = y + 1;

                let mt = unsafe { *move_time.get_unchecked(nx).get_unchecked(ny) };
                let start = if t > mt { t } else { mt };
                let new_time = start + cost;

                let i = ((nx * m + ny) << 1) | np;

                if new_time < dist[i] {
                    dist[i] = new_time;
                    pq.push((Reverse(new_time), nx as u16, ny as u16, np as u8));
                }
            }

            if y > 0 {
                let nx = x;
                let ny = y - 1;

                let mt = unsafe { *move_time.get_unchecked(nx).get_unchecked(ny) };
                let start = if t > mt { t } else { mt };
                let new_time = start + cost;

                let i = ((nx * m + ny) << 1) | np;

                if new_time < dist[i] {
                    dist[i] = new_time;
                    pq.push((Reverse(new_time), nx as u16, ny as u16, np as u8));
                }
            }
        }

        -1
    }
}
