#[derive(Debug)]
pub struct AddUnitToGameFieldRequest {
    account_unique_id: i32,
    unit_card_id: i32,
}

impl AddUnitToGameFieldRequest {
    pub fn new(account_unique_id: i32, unit_card_id: i32) -> Self {
        AddUnitToGameFieldRequest {
            account_unique_id,
            unit_card_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
}
