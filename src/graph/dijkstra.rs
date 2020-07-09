pub struct Graph {
    vertices: i32,
    items: Vec<Vec<i32>>
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

    pub fn dijkstra(&self, src: i32) -> Vec<i32> {
        let mut dist = vec![i32::max_value(); self.vertices as usize];
        dist[src as usize] = 0;

        let mut spt_set = vec![false; self.vertices as usize];

        for _ in 0..self.vertices - 1 {
            let u = self.min_distance(&dist, &spt_set);

            spt_set[u as usize] = true;

            for v in 0..self.vertices {
                let vert = &self.items[u as usize];
                if !spt_set[v as usize]
                    && vert[v as usize] > 0
                    && dist[u as usize] != i32::max_value()
                    && (dist[u as usize] + vert[v as usize]) < dist[v as usize] {
                        dist[v as usize] = dist[u as usize] + vert[v as usize];
                }
            }
        }

        dist
    }

    fn min_distance(&self, dist: &Vec<i32>, spt_set: &Vec<bool>) -> i32 {
        let mut min_index = 0;
        let mut min = i32::max_value();

        for idx in 0..self.vertices {
            if !spt_set[idx as usize] && dist[idx as usize] <= min {
                min = dist[idx as usize];
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

        assert_eq!(graph.dijkstra(0), expect_distances);
    }
}
