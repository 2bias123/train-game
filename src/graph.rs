pub mod tgraph {
    use std::collections::HashMap;

    pub struct Graph<T> {
        nodes: HashMap<T, Vec<T>>,
    }
    
    impl<T: Eq + std::hash::Hash + Clone> Graph<T> {
        pub fn new() -> Graph<T> {
            Graph { nodes: HashMap::new() }
        }
    
        pub fn add_node(&mut self, node: &T) {
            self.nodes.entry(node.clone()).or_insert(vec![]);
        }
    
        pub fn add_edge(&mut self, from: &T, to: &T) {
            self.nodes.entry(from.clone()).or_insert(vec![]).push(to.clone());
            self.nodes.entry(to.clone()).or_insert(vec![]).push(from.clone());
        }
    
        pub fn get_neighbors(&self, node: &T) -> Option<&Vec<T>> {
            self.nodes.get(node)
        }

        pub fn get_nodes(&self) -> Vec<T> {
            let mut nodes: Vec<T> = Vec::new();
            for (node, _) in &self.nodes {
                nodes.push(node.clone());
            }
            nodes
        }

        
    
        fn remove_node(&mut self, node: &T) -> Option<Vec<T>> {
            if let Some(edges) = self.nodes.remove(node) {
                for (_, neighbors) in &mut self.nodes {
                    neighbors.retain(|n| n != node);
                }
                Some(edges)
            } else {
                None
            }
        }
    
        fn remove_edge(&mut self, from: &T, to: &T) {
            if let Some(neighbors) = self.nodes.get_mut(from) {
                neighbors.retain(|n| n != to);
            }
            if let Some(neighbors) = self.nodes.get_mut(to) {
                neighbors.retain(|n| n != from);
            }
        }
    }
}
