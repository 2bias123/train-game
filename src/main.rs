mod graph;
mod node;
use graph::tgraph::Graph;
use node::tnode::Node;


fn main() {
    let mut g = Graph::<u32, u32, Node>::new();

    g.add_node(node::tnode::Node::new(1, "Node 1".to_string()));
    g.add_node(node::tnode::Node::new(2, "Node 2".to_string()));

    g.add_edge(1, 2);

    
}

