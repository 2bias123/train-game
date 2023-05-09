pub mod buildgraph {
    use crate::node::tnode::Node;
    use crate::edge::tedge::Edge;
    use crate::graph::tgraph::Graph;

    #[derive(Debug)]
    pub enum Color {
        Red,
        Blue,
        Green,
        Yellow,
        Black,
        White,
        Orange,
        Pink,
        Purple,
        Gray
    }

    pub fn build_graph() -> Graph<Node,Edge<Color>>{
        let mut g: Graph<Node,Edge<Color>> = Graph::new();

        g.add_node(1, Node::new("San Fransisco".to_string()));
        g.add_node(2, Node::new("Los Angeles".to_string()));
        g.add_node(3, Node::new("Las Vegas".to_string()));
        g.add_node(4, Node::new("Salt Lake City".to_string()));
        g.add_node(5, Node::new("Phoenix".to_string()));
        g.add_node(6,Node::new("El Paso".to_string()));
        g.add_node(7,Node::new("Santa Fe".to_string()));

        g
    }
}