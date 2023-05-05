pub mod tgraph {
    use std::collections::HashMap;

    //The key in the nodes HashMap is the id of the node that is to be used for refrencing, and the T is the value
    //The key in the adjacency_list HashMap is the id of the node that is to be used for refrencing, and the Vec<u32> is a vector of the ids the key is connected to
    pub struct Graph<T> {
        nodes: HashMap<u32, T>,
        adjacency_list: HashMap<u32, Vec<u32>>
    }
    
    impl<T: Eq + std::hash::Hash + Clone> Graph<T> {
        pub fn new() -> Graph<T> {
            Graph { nodes: HashMap::new(), adjacency_list: HashMap::new() }
        }

        pub fn add_node(&mut self, id: u32, value: T) {
            self.nodes.insert(id, value);
            self.adjacency_list.insert(id, Vec::new());
        }

        pub fn get_node(&self, id: u32) -> Option<&T> {
            self.nodes.get(&id)
        }

        pub fn add_edge(&mut self, from: u32, to: u32) {
            self.adjacency_list.get_mut(&from).unwrap().push(to);
        }

        pub fn get_adjacency_list(&self) -> &HashMap<u32, Vec<u32>> {
            &self.adjacency_list
        }
    }    


}
