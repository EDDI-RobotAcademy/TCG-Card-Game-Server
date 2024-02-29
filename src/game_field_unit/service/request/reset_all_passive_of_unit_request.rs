#[derive(Debug)]
pub struct ResetAllPassiveOfUnitRequest {
    account_unique_id: i32,
    unit_card_index: i32,
    passive_default_list: Vec<bool>
}

impl ResetAllPassiveOfUnitRequest {
    pub fn new(account_unique_id: i32, unit_card_index: i32, passive_default_list: Vec<bool>) -> Self {
        ResetAllPassiveOfUnitRequest {
            account_unique_id,
            unit_card_index,
            passive_default_list
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
    pub fn get_unit_card_index(&self) -> i32 { self.unit_card_index }
    pub fn get_passive_default_list(self) -> Vec<bool> { self.passive_default_list }
}