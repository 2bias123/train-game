pub mod tnode{

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub struct Node {
        id: u32,
        name: String,
    }
    
    impl Node {
        pub fn new(id: u32, name: String) -> Node {
            Node { id, name }
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }

        pub fn change_id(&mut self, id: u32) {
            self.id = id;
        }
    }
}