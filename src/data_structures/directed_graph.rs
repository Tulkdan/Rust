#[derive(Debug, PartialEq)]
pub enum Errors {
    OutOfBounds
}

#[derive(Clone, PartialEq)]
pub struct Node {
    value: i32,
    pointers: Vec<i32>
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node { value, pointers: vec![] }
    }
}

pub struct DirectedGraph {
    nodes: Vec<Node>,
    length: usize,
}

impl DirectedGraph {
    pub fn new(nodes_qtd: usize) -> DirectedGraph {
        DirectedGraph {
            nodes: vec![],
            length: nodes_qtd,
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), Errors> {
        if self.nodes.len() + 1 > self.length {
            return Err(Errors::OutOfBounds);
        }

        self.nodes.push(node);
        Ok(())
    }

    pub fn add_edge(&mut self, origin: usize, destination: usize) -> Result<(), Errors> {
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
    fn should_create_an_empty_vector() {
        let graph = DirectedGraph::new(2);
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.length, 2);
    }

    #[test]
    fn should_add_node_to_graph() {
        let mut graph = DirectedGraph::new(2);
        let first_node = Node::new(0);

        graph.add_node(first_node).unwrap();

        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].value, 0);
        assert_eq!(graph.nodes[0].pointers.len(), 0);
    }

    #[test]
    fn should_add_edge_to_node() {
        let mut graph = DirectedGraph::new(3);
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
        let mut graph = DirectedGraph::new(2);
        let first_node = Node::new(0);

        graph.add_node(first_node).unwrap();

        match graph.add_edge(0, 1) {
            Err(e) => assert_eq!(e, Errors::OutOfBounds),
            Ok(_) => assert!(false)
        };
    }

    #[test]
    fn should_throw_error_when_exceeding_nodes() {
        let mut graph = DirectedGraph::new(1);
        let first_node = Node::new(0);
        let second_node = Node::new(4);

        graph.add_node(first_node).unwrap();
        match graph.add_node(second_node) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, Errors::OutOfBounds)
        };
    }
}
