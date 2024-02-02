#[derive(Debug)]
pub struct AccountDeckCardListRequest {
    deck_id: i32
}

impl AccountDeckCardListRequest {
    pub fn new(deck_id: i32) -> Self {
        AccountDeckCardListRequest {
            deck_id
        }
    }
    pub fn deck_id(&self) -> i32 { self.deck_id }
}