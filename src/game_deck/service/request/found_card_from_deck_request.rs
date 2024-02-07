#[derive(Debug)]
pub struct FoundCardFromDeckRequest {
    account_unique_id: i32,
    need_to_find_card_id: i32,
    energy_count: i32
}

impl FoundCardFromDeckRequest {
    pub fn new(account_unique_id: i32, need_to_find_card_id: i32, energy_count: i32) -> Self {
        FoundCardFromDeckRequest {
            account_unique_id,
            need_to_find_card_id,
            energy_count
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_need_to_find_card_id(&self) -> i32 {
        self.need_to_find_card_id
    }

    pub fn get_energy_count(&self) -> i32 {
        self.energy_count
    }
}
