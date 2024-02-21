#[derive(Debug)]
pub struct CanUseCardRequest {
    account_unique_id: i32,
    card_id: i32,
}

impl CanUseCardRequest {
    pub fn new(account_unique_id: i32, card_id: i32) -> Self {
        CanUseCardRequest {
            account_unique_id,
            card_id
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_card_id(&self) -> i32 {
        self.card_id
    }
}