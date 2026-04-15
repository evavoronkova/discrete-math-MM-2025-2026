use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use super::Solution;
impl Solution {
    pub fn find_cheapest_price(
        n: i32,
        flights: Vec<Vec<i32>>,
        src: i32,
        dst: i32,
        k: i32,
    ) -> i32 {
        let n = n as usize;
        let max_stops = k + 1;

        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for flight in &flights {
            let (from, to, price) = (
                flight[0] as usize,
                flight[1] as usize,
                flight[2],
            );
            graph[from].push((to, price));
        }

        let mut cost: Vec<HashMap<i32, i32>> = vec![HashMap::new(); n];
        cost[src as usize].insert(0, 0);

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), src as usize, 0));

        while let Some((Reverse(total_cost), node, stops)) = heap.pop() {
            if node == dst as usize {
                return total_cost;
            }

            if stops > max_stops {
                continue;
            }

            if let Some(&best) = cost[node].get(&stops) {
                if total_cost > best {
                    continue;
                }
            }

            if stops == max_stops {
                continue;
            }

            for &(neighbor, price) in &graph[node] {
                let new_stops = stops + 1;
                let new_cost = total_cost + price;

                let mut dominated = false;
                if let Some(map) = cost.get(neighbor) {
                    for (&s, &c) in map {
                        if s <= new_stops && c <= new_cost {
                            dominated = true;
                            break;
                        }
                    }
                }

                if dominated {
                    continue;
                }

                let entry = cost[neighbor].entry(new_stops).or_insert(i32::MAX);
                if new_cost < *entry {
                    *entry = new_cost;

                    cost[neighbor].retain(|s, c| {
                        let s_val = *s;
                        let c_val = *c;
                    
                        !(s_val >= new_stops && c_val >= new_cost &&
                          !(s_val == new_stops && c_val == new_cost))
                    });

                    heap.push((Reverse(new_cost), neighbor, new_stops));
                }
            }
        }

        -1
    }
}