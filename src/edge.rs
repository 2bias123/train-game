pub mod tedge{
    
    pub struct Edge<T> {
        from: T,
        to: T,
        weight: u32,
    }

    impl <T> Edge<T> {
        pub fn new(from: T, to: T, weight: u32) -> Edge<T> {
            Edge { from, to, weight }
        }

        pub fn get_from(&self) -> &T {
            &self.from
        }

        pub fn get_to(&self) -> &T {
            &self.to
        }

        pub fn get_weight(&self) -> u32 {
            self.weight
        }
    }
}