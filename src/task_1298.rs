use std::collections::VecDeque;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let n = status.len();
        let mut status = status;
        let mut candies = candies;
        let mut queue_boxes: VecDeque<i32> = VecDeque::new();
        let mut skipped = vec![false; n];
        let mut found_keys = vec![false; n];
        let mut counter = 0;

        for b in initial_boxes {
            let b = b as usize;
            if status[b] == 1 || found_keys[b] {
                status[b] = 1;
                queue_boxes.push_back(b as i32);
            } else {
                skipped[b] = true;
            }
        }

        while let Some(b) = queue_boxes.pop_front() {
            let b = b as usize;
            counter += candies[b];
            candies[b] = 0;

            for &key in &keys[b] {
                let key = key as usize;
                found_keys[key] = true;
                if skipped[key] {
                    skipped[key] = false;
                    status[key] = 1;
                    queue_boxes.push_back(key as i32);
                }
            }

            for &i in &contained_boxes[b] {
                let i = i as usize;
                if status[i] == 1 || found_keys[i] {
                    status[i] = 1;
                    queue_boxes.push_back(i as i32);
                } else {
                    skipped[i] = true;
                }
            }
        }

        counter
    }
}

pub struct Solution;
