pub mod player {

    #[derive(Debug)]
    pub struct player{
        name: String,
        score: u32,
        number_of_trains: u32,
        cards: Vec<String>,
    }
}