use crate::graph::Graph;

#[derive(Debug, Clone)]
pub struct TriangleStats {
    pub total_triangles: u64,
    pub triangles_per_vertex: Vec<u32>,
    pub degrees: Vec<u32>,
    pub triplets_total: u64,
}

fn choose_2(n: u32) -> u64 {
    if n < 2 {
        0
    } else {
        (n as u64 * (n as u64 - 1)) / 2
    }
}

pub fn compute_triangle_stats(graph: &Graph, allowed_mask: Option<&[bool]>) -> TriangleStats {
    let n = graph.num_vertices();
    let mut degrees = vec![0_u32; n];

    for v in graph.vertices_internal() {
        if allowed_mask.is_some_and(|mask| !mask[v as usize]) {
            continue;
        }

        let degree = graph
            .neighbors_internal(v)
            .iter()
            .filter(|&&u| allowed_mask.is_none_or(|mask| mask[u as usize]))
            .count() as u32;
        degrees[v as usize] = degree;
    }

    // Ориентируем каждое неориентированное ребро только в одну сторону по
    // ordering (degree, id). Это резко уменьшает размеры списков пересечения.
    let mut forward = vec![Vec::new(); n];
    for u in graph.vertices_internal() {
        if allowed_mask.is_some_and(|mask| !mask[u as usize]) {
            continue;
        }

        for &v in graph.neighbors_internal(u) {
            if allowed_mask.is_some_and(|mask| !mask[v as usize]) || u >= v {
                continue;
            }

            let order_u = (degrees[u as usize], u);
            let order_v = (degrees[v as usize], v);
            if order_u < order_v {
                forward[u as usize].push(v);
            } else {
                forward[v as usize].push(u);
            }
        }
    }

    for neighbors in &mut forward {
        neighbors.sort_unstable();
        neighbors.dedup();
    }

    let mut total_triangles = 0_u64;
    let mut triangles_per_vertex = vec![0_u32; n];

    for u in graph.vertices_internal() {
        if allowed_mask.is_some_and(|mask| !mask[u as usize]) {
            continue;
        }

        for &v in &forward[u as usize] {
            let left = &forward[u as usize];
            let right = &forward[v as usize];
            let mut i = 0;
            let mut j = 0;

            // Два указателя по уже отсортированным forward-спискам дают
            // пересечение без вложенного contains по большим adjacency.
            while i < left.len() && j < right.len() {
                match left[i].cmp(&right[j]) {
                    std::cmp::Ordering::Less => i += 1,
                    std::cmp::Ordering::Greater => j += 1,
                    std::cmp::Ordering::Equal => {
                        let w = left[i];
                        total_triangles += 1;
                        triangles_per_vertex[u as usize] += 1;
                        triangles_per_vertex[v as usize] += 1;
                        triangles_per_vertex[w as usize] += 1;
                        i += 1;
                        j += 1;
                    }
                }
            }
        }
    }

    let triplets_total = graph
        .vertices_internal()
        .filter(|&v| allowed_mask.is_none_or(|mask| mask[v as usize]))
        .map(|v| choose_2(degrees[v as usize]))
        .sum();

    TriangleStats {
        total_triangles,
        triangles_per_vertex,
        degrees,
        triplets_total,
    }
}

pub fn find_triangles(graph: &Graph) -> u32 {
    compute_triangle_stats(graph, None).total_triangles as u32
}
