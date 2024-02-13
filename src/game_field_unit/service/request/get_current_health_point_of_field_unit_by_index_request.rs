#[derive(Debug)]
pub struct GetCurrentHealthPointOfFieldUnitByIndexRequest {
    account_unique_id: i32,
    field_unit_index: i32,
}

impl GetCurrentHealthPointOfFieldUnitByIndexRequest {
    pub fn new(account_unique_id: i32, field_unit_index: i32) -> Self {
        GetCurrentHealthPointOfFieldUnitByIndexRequest {
            account_unique_id,
            field_unit_index,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
    pub fn get_field_unit_index(&self) -> i32 {
        self.field_unit_index
    }
}