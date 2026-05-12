use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

impl Solution {
    fn dijkstra(
        adj_list: &Vec<Vec<Edge>>,
        start: usize,
        goal: usize,
        disappear: &Vec<i32>,
    ) -> Option<usize> {
        let mut dists: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();

        dists[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                return Some(cost);
            }
            if cost > dists[position] {
                continue;
            }
            for edge in &adj_list[position] {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                if next.cost < dists[next.position] && next.cost < disappear[next.position] as usize
                {
                    heap.push(next);
                    dists[next.position] = next.cost;
                }
            }
        }
        None
    }
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let mut graph: Vec<Vec<Edge>> = vec![vec![]; n as usize];
        for edge in &edges {
            graph[edge[0] as usize].push(Edge {
                node: edge[1] as usize,
                cost: edge[2] as usize,
            });

            graph[edge[1] as usize].push(Edge {
                node: edge[0] as usize,
                cost: edge[2] as usize,
            });
        }

        let mut answer: Vec<i32> = Vec::new();
        for index in 0..n as usize {
            answer.push(match Self::dijkstra(&graph, 0, index, &disappear) {
                Some(res) => res as i32,
                None => -1,
            });
        }

        answer
    }
}

pub struct Solution;
