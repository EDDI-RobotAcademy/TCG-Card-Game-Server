#[derive(Debug)]
pub struct GenerateSearchMyDeckDataRequest {

    found_card_list: Vec<i32>,
}

impl GenerateSearchMyDeckDataRequest {
    pub fn new(found_card_list: Vec<i32>) -> Self {
        GenerateSearchMyDeckDataRequest {
            found_card_list
        }
    }


    pub fn get_found_card_list(&self) -> &Vec<i32> { &self.found_card_list }
}