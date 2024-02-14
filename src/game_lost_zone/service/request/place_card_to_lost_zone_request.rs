#[derive(Debug)]
pub struct PlaceCardToLostZoneRequest {
    account_unique_id: i32,
    lost_card_id: i32
}

impl PlaceCardToLostZoneRequest {
    pub fn new(account_unique_id: i32, lost_card_id: i32) -> Self {
        PlaceCardToLostZoneRequest {
            account_unique_id,
            lost_card_id
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_lost_card_id(&self) -> i32 {
        self.lost_card_id
    }
}