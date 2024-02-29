#[derive(Debug)]
pub struct GenerateDrawOpponentDeckDataRequest {
    drawn_card_list: Vec<i32>,
}

impl GenerateDrawOpponentDeckDataRequest {
    pub fn new(drawn_card_list: Vec<i32>,) -> Self {
        GenerateDrawOpponentDeckDataRequest {
            drawn_card_list,
        }
    }

    pub fn get_drawn_card_list(&self) -> &Vec<i32> { &self.drawn_card_list }
}