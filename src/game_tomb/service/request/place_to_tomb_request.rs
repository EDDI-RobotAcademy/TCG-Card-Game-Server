#[derive(Debug)]
pub struct PlaceToTombRequest {
    account_unique_id: i32,
    used_card_id: i32
}

impl PlaceToTombRequest {
    pub fn new(account_unique_id: i32, used_card_id: i32) -> Self {
        PlaceToTombRequest {
            account_unique_id,
            used_card_id
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_used_card_id(&self) -> i32 {
        self.used_card_id
    }
}