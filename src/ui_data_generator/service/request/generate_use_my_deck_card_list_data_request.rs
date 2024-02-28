#[derive(Debug)]
pub struct GenerateUseMyDeckCardListDataRequest {
    deck_card_id_list: Vec<i32>,
}

impl GenerateUseMyDeckCardListDataRequest {
    pub fn new(deck_card_id_list: Vec<i32>,) -> Self {
        GenerateUseMyDeckCardListDataRequest {
            deck_card_id_list,
        }
    }

    pub fn get_deck_card_id_list(&self) -> &Vec<i32> { &self.deck_card_id_list }
}