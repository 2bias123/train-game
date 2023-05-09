pub mod tgraph {
    use std::collections::HashMap;

    //The key in the nodes HashMap is the id of the node that is to be used for refrencing, and the T is the value
    //The key in the adjacency_list HashMap is the id of the node that is to be used for refrencing, and the Vec<u32> is a vector of the ids the key is connected to
    pub struct Graph<T,E> {
        nodes: HashMap<u32, T>,
        adjacency_list: HashMap<u32, Vec<E>>
    }
    
    impl<T: Eq + std::hash::Hash + Clone, E> Graph<T,E> {
        pub fn new() -> Graph<T,E> {
            Graph { nodes: HashMap::new(), adjacency_list: HashMap::new() }
        }

        pub fn add_node(&mut self, id: u32, value: T) {
            self.nodes.insert(id, value);
        }

        pub fn add_edge(&mut self, id: u32, edge: E) {
            if self.adjacency_list.contains_key(&id) {
                self.adjacency_list.get_mut(&id).unwrap().push(edge);
            } else {
                self.adjacency_list.insert(id, vec![edge]);
            }
        }

        pub fn get_node(&self, id: u32) -> Option<&T> {
            self.nodes.get(&id)
        }

        pub fn get_edge(&self, id: u32) -> Option<&Vec<E>> {
            self.adjacency_list.get(&id)
        }

        pub fn get_edges(&self) -> &HashMap<u32, Vec<E>> {
            &self.adjacency_list
        }
    }    
}