#[derive(Debug)]
pub struct ExecuteIndexPassiveOfUnitRequest {
    account_unique_id: i32,
    unit_card_index: i32,
    passive_skill_index: i32
}

impl ExecuteIndexPassiveOfUnitRequest {
    pub fn new(account_unique_id: i32, unit_card_index: i32, passive_skill_index: i32) -> Self {
        ExecuteIndexPassiveOfUnitRequest {
            account_unique_id,
            unit_card_index,
            passive_skill_index
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }
    pub fn get_passive_skill_index(&self) -> i32 { self.passive_skill_index }

}