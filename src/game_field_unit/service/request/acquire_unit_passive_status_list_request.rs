#[derive(Debug)]
pub struct AcquireUnitPassiveStatusListRequest {
    account_unique_id: i32,
    unit_index: i32
}

impl AcquireUnitPassiveStatusListRequest {
    pub fn new(account_unique_id: i32, unit_index: i32) -> Self {
        AcquireUnitPassiveStatusListRequest {
            account_unique_id,
            unit_index
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_unit_index(&self) -> i32 {
        self.unit_index
    }
}