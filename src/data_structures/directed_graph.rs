use data_structures::graph::{Errors, Node, Graph};

pub trait DirectedGraph {
    fn add_edge(&mut self, origin: usize, destination: usize) -> Result<(), Errors>;
}

impl DirectedGraph for Graph {
    fn add_edge(&mut self, origin: usize, destination: usize) -> Result<(), Errors> {
        if destination >= self.nodes.len() || destination >= self.length || origin >= self.length {
            return Err(Errors::OutOfBounds);
        }

        let value = self.nodes[destination].value;
        self.nodes[origin].pointers.push(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_edge_to_node() {
        let mut graph = Graph::new(3);
        let first_node = Node::new(0);
        let second_node = Node::new(8);
        let third_node = Node::new(5);

        graph.add_node(first_node).unwrap();
        graph.add_node(second_node).unwrap();
        graph.add_node(third_node).unwrap();

        graph.add_edge(0, 1).unwrap();

        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.nodes[0].pointers.len(), 1);
        assert_eq!(graph.nodes[0].pointers[0], 8);
    }

    #[test]
    fn should_throw_error_when_adding_non_existed_edge() {
        let mut graph = Graph::new(2);
        let first_node = Node::new(0);

        graph.add_node(first_node).unwrap();

        match graph.add_edge(0, 1) {
            Err(e) => assert_eq!(e, Errors::OutOfBounds),
            Ok(_) => assert!(false)
        };
    }
}
