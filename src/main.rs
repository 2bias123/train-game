#![allow(warnings)]
mod graph;
mod node;
mod edge;
mod player;
mod buildgraph;
mod color_enum;
mod shortest_path;

use buildgraph::buildgraph::build_graph;
use shortest_path::short_path::djikstras;
use node::tnode::Node;

fn main() {
    let mut g = build_graph();
    let sanfran = Node::new(1,"San Fransisco".to_string());

    println!("{:?}",g.get_neighbors(&sanfran).iter().for_each(|(key,value)|{
        
    }
))

    // let sanfran = Node::new(1,"San Fransisco".to_string());
    // let la = Node::new(2,"Los Angeles".to_string());

    // let mut afa = djikstras(g, sanfran, la);

    // println!("{:?}", afa);
}