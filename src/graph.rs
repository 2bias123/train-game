pub mod tgraph {
    use std::hash::Hash;
    use std::collections::HashMap;

    pub struct Graph<Id, E = (),V = ()> {
        nodes: HashMap<Id, V>,
        adjecency_list: HashMap<Id, Vec<(Id, E)>>,
    }

    impl<Id, E, V> Graph<Id, E, V>
    where 
        Id: Eq + Hash + Copy,
        V: Hash,
    {
        pub fn new() -> Graph<Id, E, V> {
            Graph {
                nodes: HashMap::new(),
                adjecency_list: HashMap::new(),
            }
        }

        pub fn add_node(&mut self, id: Id, value: V) {
            self.nodes.insert(id, value);
        }

        pub fn add_edge(&mut self, from: Id, to: Id, weight: E) {
            let to_from = self.adjecency_list.entry(from).or_default();
            to_from.push((to, weight));
        }
    }

}
