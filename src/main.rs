#![allow(warnings)]
mod graph;
mod node;
mod edge;
mod player;
mod buildgraph;
mod color_enum;


use buildgraph::buildgraph::build_graph;

fn main() {
    let mut g = build_graph();

    g.print_graph()

}