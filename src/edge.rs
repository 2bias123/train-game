pub mod tedge{
    use crate::player::player::player;
    #[derive(Debug)]
    pub struct Edge<T> {
        connected_node: u32,
        color: T,
        nmbr_of_trains: u32,
        owner: player
        //TODO: Add a field for if its owned by a player
    }

    impl <T> Edge<T> {
        pub fn new(connected_node: u32, color: T, nmbr_of_trains: u32, owner: player) -> Edge<T> {
            Edge { connected_node, color, nmbr_of_trains, owner}
        }

        pub fn change_connected_node(&mut self, connected_node: u32) {
            self.connected_node = connected_node;
        }

        pub fn change_color(&mut self, color: T) {
            self.color = color;
        }

        pub fn change_nmbr_of_trains(&mut self, nmbr_of_trains: u32) {
            self.nmbr_of_trains = nmbr_of_trains;
        }
    }
}