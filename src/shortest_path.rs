pub mod short_path {
    use crate::graph::tgraph::Graph;
    use std::cmp::Reverse;
    use std::collections::HashMap;
    use std::collections::BinaryHeap;

    /*
    To implement Dijkstra's algorithm for the Graph structure you provided, you can follow these steps:

Create a function or method that takes the graph, source node, and destination node as input parameters.

Initialize a distance hashmap to store the shortest distances from the source node to all other nodes. Set the distance of the source node to 0 and initialize the distances of all other nodes to infinity.

Create a priority queue (e.g., a binary heap or Fibonacci heap) to store nodes based on their tentative distances from the source. Initialize it with the source node and its distance.

Create a hashmap to store the previous node in the shortest path to each node.

While the priority queue is not empty, do the following steps:

Extract the node with the minimum distance from the priority queue.
If the extracted node is the destination node, exit the loop and proceed to the next step.
Iterate over the neighboring nodes of the extracted node.
Calculate the tentative distance from the source to the neighboring node by adding the edge weight between the current node and the neighboring node.
If the tentative distance is smaller than the current distance stored in the distance hashmap for the neighboring node:
Update the distance hashmap with the new tentative distance.
Update the previous node hashmap with the extracted node as the previous node for the neighboring node.
Insert the neighboring node and its distance into the priority queue.
If a path exists from the source to the destination node, retrieve the shortest path by backtracking from the destination node to the source node using the previous node hashmap. Start from the destination node, follow the previous nodes until you reach the source node, and record the nodes along the way.

Reverse the recorded path to obtain the correct order of nodes from the source to the destination.
     */

    #[derive(Debug)]
    pub struct Wrapper<T>(usize, T);
    
    impl<T> PartialEq for Wrapper<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    impl<T> Eq for Wrapper<T> {}
    
    impl<T> PartialOrd for Wrapper<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    
    impl<T> Ord for Wrapper<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    pub fn djikstras<T: Eq + std::hash::Hash + Clone + std::fmt::Debug>(mut g: Graph<T>,src: T, dest: T) {
        let mut distancemap: HashMap<T, usize> = HashMap::new();
        //Initialize all distances to infinity
        g.get_nodes_iterator().for_each(|node| {
            if node == &src {
                distancemap.insert(node.clone(), 0);
            } else {
                distancemap.insert(node.clone(), std::usize::MAX);
            }
        });

        let mut priority_queue: BinaryHeap<Reverse<Wrapper<T>>> = BinaryHeap::new();
        priority_queue.push(Reverse(Wrapper(0, src.clone())));

        let mut previous_node_in_the_shortest_path: HashMap<T, T> = HashMap::new();

        while !priority_queue.is_empty() {
            let mut current_node =
            match priority_queue.pop() {
                Some(top_node) => top_node.0.1,
                None => panic!("Something went wrong")
            };
            if !(current_node == dest) {
                g.get_neighbors(&current_node).iter().for_each(|(key,value)|{
                    let tentative_distance = if distancemap[key] == std::usize::MAX{
                        *value
                    } else{
                        distancemap[key] + value
                    };

                    if tentative_distance < distancemap[key]{
                        match distancemap.get_mut(key) {
                            Some(val) => *val = tentative_distance,
                            None => panic!("Key not found"),
                        }
                        match previous_node_in_the_shortest_path.get_mut(&current_node) {
                            Some(previous_shortest) => *previous_shortest = key,
                            None => todo!(),
                        }
                    }
                    

                });
            }
        }
    }
}