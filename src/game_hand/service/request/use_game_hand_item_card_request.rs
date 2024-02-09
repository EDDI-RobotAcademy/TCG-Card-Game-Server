#[derive(Debug)]
pub struct UseGameHandItemCardRequest {
    account_unique_id: i32,
    item_card_id: i32,
}

impl UseGameHandItemCardRequest {
    pub fn new(account_unique_id: i32, item_card_id: i32) -> Self {
        UseGameHandItemCardRequest {
            account_unique_id,
            item_card_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_item_card_id(&self) -> i32 {
        self.item_card_id
    }
}