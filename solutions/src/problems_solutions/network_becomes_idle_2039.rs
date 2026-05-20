use super::Solution;

impl Solution {
    // BFS от мастера (узел 0) — находим расстояния.
    // Для каждого узла считаем последний раз, когда он отправил сообщение
    // (чтобы не отправить после того, как уже получил ответ).
    // Ответ = max(время последнего ответа) + 1.
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        // Строим граф
        let mut graph = vec![vec![]; patience.len()];
        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        // BFS от 0 — расстояния
        let mut dist = vec![i32::MAX; patience.len()];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        dist[0] = 0;
        while let Some(node) = queue.pop_front() {
            for &neighbor in graph[node].iter() {
                if dist[neighbor] == i32::MAX {
                    dist[neighbor] = dist[node] + 1;
                    queue.push_back(neighbor);
                }
            }
        }

        // Считаем время последнего ответа для каждого узла
        let mut max_time = 0;
        for i in 1..patience.len() {
            // Последняя отправка, которая ещё успеет до получения ответа
            let last_send = (((dist[i] << 1) - 1) / patience[i]) * patience[i];
            max_time = max_time.max(last_send + dist[i] * 2);
        }

        // Бездействие — на следующий тик после последнего ответа
        max_time + 1
    }
}
