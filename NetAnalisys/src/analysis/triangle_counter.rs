use crate::graph::Graph;

pub fn find_triangles(graph: &Graph) -> u32 {
    let mut triangles = 0;

    for (node, neighbours) in graph.adjacency_entries_internal() {
        for &neighbour in neighbours {
            if node < neighbour {
                let neighbour_neighbours = graph.neighbors_internal(neighbour);
                let (small, big) = if neighbours.len() < neighbour_neighbours.len() {
                    (neighbours, neighbour_neighbours)
                } else {
                    (neighbour_neighbours, neighbours)
                };

                for &w in small {
                    if big.contains(&w) {
                        triangles += 1;
                    }
                }
            }
        }
    }

    (triangles / 3) as u32
}
