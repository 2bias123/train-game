pub mod buildgraph {
    use crate::edge::tedge::Edge;
    use crate::node::tnode::Node;
    use crate::graph::tgraph::Graph;
    use crate::color_enum::color_enum::Color;


    pub fn build_graph() -> Graph<Node>{
        let node1 = Node::new(1,"San Fransisco".to_string());
        let node2 = Node::new(2,"Los Angeles".to_string());

        let mut g: Graph<Node> = Graph::new();

        g.add_node(node1.clone());
        g.add_node(node2.clone());

        g.add_bidirectional_edge(node1, node2, 3);

        g.add_node(Node::new(3,"Las Vegas".to_string()));
        g.add_node(Node::new(4,"Salt Lake City".to_string()));
        g.add_node(Node::new(5,"Phoenix".to_string()));
        g.add_node(Node::new(6,"El Paso".to_string()));
        g.add_node(Node::new(7,"Santa Fe".to_string()));

        g
    }
}