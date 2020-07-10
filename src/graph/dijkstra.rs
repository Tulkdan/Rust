pub struct Graph {
    vertices: i32,
    items: Vec<Vec<i32>>
}

#[derive(PartialEq, Clone, Debug)]
pub enum Visited {
    Yes,
    No
}

#[derive(Clone, Debug)]
pub struct Dijkstra {
    distance: i32,
    visited: Visited,
}

impl Dijkstra {
    pub fn new() -> Dijkstra {
        Dijkstra {
            distance: i32::max_value(),
            visited: Visited::No,
        }
    }

    pub fn reset_distance(&mut self) {
        self.set_distance(0);
    }

    pub fn set_visited(&mut self, visited: Visited) {
        self.visited = visited;
    }

    pub fn set_distance(&mut self, distance: i32) {
        self.distance = distance;
    }
}

impl Graph {
    pub fn new(vertices: i32) -> Graph {
        let items = vec![
            Vec::with_capacity(vertices as usize);
            vertices as usize
        ];

        Graph {
            vertices,
            items,
        }
    }

    pub fn insert_at(&mut self, idx: usize, values: &[i32]) {
        self.items[idx].extend_from_slice(values);
    }

    pub fn dijkstra(&self, src: i32) -> Vec<Dijkstra> {
        let mut dist = vec![Dijkstra::new(); self.vertices as usize];
        dist[src as usize].reset_distance();

        for _ in 0..self.vertices - 1 {
            let u = self.min_distance(&dist);

            dist[u].set_visited(Visited::Yes);

            for v in 0..self.vertices {
                let item = &self.items[u as usize];

                if dist[v as usize].visited == Visited::No
                    && item[v as usize] > 0
                    && dist[u].distance != i32::max_value()
                    && (dist[u].distance + item[v as usize]) < dist[v as usize].distance {
                        let sum = dist[u].distance + item[v as usize];
                        dist[v as usize].set_distance(sum);
                }
            }
        }

        dist
    }

    fn min_distance(&self, dist: &Vec<Dijkstra>) -> usize {
        let mut min_index = 0;
        let mut min = i32::max_value();

        for (idx, item) in dist.iter().enumerate() {
            if item.visited == Visited::No && item.distance <= min {
                min = item.distance;
                min_index = idx;
            }
        }

        min_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_graph_with_n_vertices() {
        let graph = Graph::new(2);
        assert_eq!(graph.vertices, 2);
        assert_eq!(graph.items.len(), 2);
        for item in graph.items.iter() {
            assert_eq!(item.len(), 0);
        }
    }

    #[test]
    fn should_insert_values_for_graph() {
        let vertices_num = 9;
        let mut graph = Graph::new(vertices_num);
        let graph_values = [
            [0, 4, 0, 0, 0, 0, 0, 8, 0],
            [4, 0, 8, 0, 0, 0, 0, 11, 0],
            [0, 8, 0, 7, 0, 4, 0, 0, 2],
            [0, 0, 7, 0, 9, 14, 0, 0, 0],
            [0, 0, 0, 9, 0, 10, 0, 0, 0],
            [0, 0, 4, 14, 10, 0, 2, 0, 0],
            [0, 0, 0, 0, 0, 2, 0, 1, 6],
            [8, 11, 0, 0, 0, 0, 1, 0, 7],
            [0, 0, 2, 0, 0, 0, 6, 7, 0]
        ];

        for (idx, values) in graph_values.iter().enumerate() {
            graph.insert_at(idx, values);
        }

        for (idx_v, vertices) in graph_values.iter().enumerate() {
            assert_eq!(graph.items[idx_v].len(), vertices_num as usize);
            for (idx, value) in vertices.iter().enumerate() {
                assert_eq!(graph.items[idx_v][idx], *value);
            }
        }
    }

    #[test]
    fn dijkstra_algorithm() {
        let vertices_num = 9;
        let mut graph = Graph::new(vertices_num);
        let graph_values = [
            [0, 4, 0, 0, 0, 0, 0, 8, 0],
            [4, 0, 8, 0, 0, 0, 0, 11, 0],
            [0, 8, 0, 7, 0, 4, 0, 0, 2],
            [0, 0, 7, 0, 9, 14, 0, 0, 0],
            [0, 0, 0, 9, 0, 10, 0, 0, 0],
            [0, 0, 4, 14, 10, 0, 2, 0, 0],
            [0, 0, 0, 0, 0, 2, 0, 1, 6],
            [8, 11, 0, 0, 0, 0, 1, 0, 7],
            [0, 0, 2, 0, 0, 0, 6, 7, 0]
        ];

        for (idx, values) in graph_values.iter().enumerate() {
            graph.insert_at(idx, values);
        }

        let expect_distances = [0, 4, 12, 19, 21, 11, 9, 8, 14];
        let dijkstra = graph.dijkstra(0);

        for (idx, distance) in expect_distances.iter().enumerate() {
            assert_eq!(dijkstra[idx].distance, *distance);
        }
    }
}
