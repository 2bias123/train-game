mod graph;
mod node;
use graph::tgraph::Graph;
use node::tnode::Node;


fn main() {
    let mut g: Graph<Node> = Graph::new();

    let mut a = Node::new(1, "Node 1".to_string());
    g.add_node(&a);

    let mut b = Node::new(2, "Node 2".to_string());
    g.add_node(&b);

    g.add_edge(&a, &b);

    println!("{:?}", g.get_nodes());
    println!("{:?}", g.get_neighbors(&a).unwrap());

    a.set_name("Node 1 changed".to_string());

    println!("{:?}", g.get_nodes());
}

