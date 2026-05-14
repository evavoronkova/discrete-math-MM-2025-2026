use std::collections::VecDeque;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut queue_boxes: VecDeque<i32> = initial_boxes.iter().map(|x| *x).collect();
        let mut status = status;
        let mut candies = candies;
        let keys = keys;
        let contained_boxes = contained_boxes;
        let mut counter = 0;
        let mut skipped: Vec<i32> = Vec::new();

        while let Some(b) = queue_boxes.pop_front() {
            println!("{}", b);
            if status[b as usize] == 0 {
                skipped.push(b);
                continue;
            }
            println!("ok");
            for i in &skipped {
                queue_boxes.push_back(*i);
            }
            skipped = Vec::new();
            counter += candies[b as usize];
            candies[b as usize] = 0;
            for key in &keys[b as usize] {
                print!("{} ", key);
                status[*key as usize] = 1;
            }
            println!();
            for i in &contained_boxes[b as usize] {
                print!("{} ", i);
                queue_boxes.push_back(*i);
            }
            println!();
        }

        counter
    }
}

pub struct Solution;
