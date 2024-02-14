#[derive(Debug)]
pub struct DrawCardsFromDeckRequest {
    account_unique_id: i32,
    draw_count: i32,
}

impl DrawCardsFromDeckRequest {
    pub fn new(account_unique_id: i32, draw_count: i32) -> Self {
        DrawCardsFromDeckRequest {
            account_unique_id,
            draw_count,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }

    pub fn get_draw_count(&self) -> i32 {
        self.draw_count
    }
}