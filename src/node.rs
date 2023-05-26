pub mod tnode{

    #[derive(Debug, Hash, Clone)]
    pub struct Node {
        id: u32,
        name: String,
    }
    
    impl Node {
        pub fn new(id: u32,name: String) -> Node {
            Node { id, name }
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }


    }

    impl PartialEq for Node{
        fn eq(&self, other: &Node) -> bool {
            self.id == other.id &&
            self.name == other.name
        }
    }

    impl Eq for Node{}

}