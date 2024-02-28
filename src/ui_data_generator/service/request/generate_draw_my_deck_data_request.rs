#[derive(Debug)]
pub struct GenerateDrawMyDeckDataRequest {
    drawn_card_list: Vec<i32>,
}

impl GenerateDrawMyDeckDataRequest {
    pub fn new(drawn_card_list: Vec<i32>,) -> Self {
        GenerateDrawMyDeckDataRequest {
            drawn_card_list,
        }
    }

    pub fn get_drawn_card_list(&self) -> &Vec<i32> { &self.drawn_card_list }
}