#[derive(Debug)]
pub struct AccountDeckDeleteRequest {
    deck_unique_id: i32
}

impl AccountDeckDeleteRequest {
    pub fn new(deck_unique_id: i32) -> Self {
        AccountDeckDeleteRequest {
            deck_unique_id
        }
    }
    pub fn deck_unique_id(&self) -> i32 { self.deck_unique_id }
}