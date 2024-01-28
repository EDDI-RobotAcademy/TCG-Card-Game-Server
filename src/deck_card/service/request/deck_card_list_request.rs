#[derive(Debug)]
pub struct DeckCardListRequest {
    deck_id: i32
}

impl DeckCardListRequest {
    pub fn new(deck_id: i32) -> Self {
        DeckCardListRequest {
            deck_id
        }
    }
    pub fn deck_id(&self) -> i32 { self.deck_id }
}