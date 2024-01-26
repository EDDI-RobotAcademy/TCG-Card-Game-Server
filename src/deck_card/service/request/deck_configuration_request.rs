#[derive(Debug)]
pub struct DeckConfigurationRequest {
    deck_id: i32,
    card_id_list: Vec<i32>
}

impl DeckConfigurationRequest {
    pub fn new(deck_id: i32, card_id_list: Vec<i32>) -> Self {
        DeckConfigurationRequest {
            deck_id,
            card_id_list
        }
    }
    pub fn deck_id(&self) -> i32 { self.deck_id }
    pub fn card_id_list_of_deck(&self) -> &Vec<i32> { &self.card_id_list }
}