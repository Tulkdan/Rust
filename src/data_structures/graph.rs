#[derive(Debug, PartialEq)]
pub enum Errors {
    OutOfBounds
}

#[derive(Clone, PartialEq)]
pub struct Node {
    pub value: i32,
    pub pointers: Vec<i32>
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node { value, pointers: vec![] }
    }
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub length: usize,
}

impl Graph {
    pub fn new(nodes_qtd: usize) -> Graph {
        Graph {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_an_empty_vector() {
        let graph = Graph::new(2);
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.length, 2);
    }

    #[test]
    fn should_add_node_to_graph() {
        let mut graph = Graph::new(2);
        let first_node = Node::new(0);

        graph.add_node(first_node).unwrap();

        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].value, 0);
        assert_eq!(graph.nodes[0].pointers.len(), 0);
    }

    #[test]
    fn should_throw_error_when_exceeding_nodes() {
        let mut graph = Graph::new(1);
        let first_node = Node::new(0);
        let second_node = Node::new(4);

        graph.add_node(first_node).unwrap();
        match graph.add_node(second_node) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, Errors::OutOfBounds)
        };
    }
}
