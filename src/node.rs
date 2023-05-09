pub mod tnode{

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub struct Node {
        name: String,
    }
    
    impl Node {
        pub fn new(name: String) -> Node {
            Node { name }
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }

    }
}