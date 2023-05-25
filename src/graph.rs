pub mod tgraph {
    use std::collections::HashMap;
    use std::collections::HashSet;


    pub struct Graph<T,E> {
        nodes: HashSet<T>,
        adjacency_list: HashMap<T, Vec<E>>
    }
    
    impl< T: Eq + std::hash::Hash + Clone, E: Eq + std::hash::Hash + Clone> Graph<T, E> {
        pub fn new() -> Graph< T, E> {
            Graph { 
                nodes: HashSet::new(), 
                adjacency_list: HashMap::new() }
        }

        pub fn add_node(&mut self, node: T){
            if !self.adjacency_list.contains_key(&node)
            {
                self.adjacency_list.insert(&node, Vec::new());
                self.nodes.insert(node);
            }
            else {
                println!("The graph already contains this node");
            }
        }

        pub fn add_edge(&mut self, node1: T, node2: T, edge: E){
            self.adjacency_list.entry(node1).or_insert_with(Vec::new).push(edge.clone());
            self.adjacency_list.entry(node2).or_insert_with(Vec::new).push(edge.clone());
        }

        pub fn get_node(&mut self, node: &T) -> &T{
            match self.nodes.get(node) {
                Some(n) => n,
                None => panic!("The node does not exist in the graph")
            }
        }
        
        pub fn get_neighbors(&mut self, node: &T) -> &Vec<E>{
            match self.adjacency_list.get(node) {
                Some(n) => n,
                None => panic!("The node does not exist in the graph")
            }
        }

        pub fn get_nodes(&mut self) -> &HashSet<T>{
            &self.nodes
        }

        pub fn get_adjecencylist(&mut self) -> &HashMap<T, Vec<E>>{
            &self.adjacency_list
        }
    }
}