#[derive(Debug)]
pub struct AttachEnergyToUnitRequest {
    account_unique_id: i32,
    unit_id: i32,
}

impl AttachEnergyToUnitRequest {
    pub fn new(account_unique_id: i32, unit_id: i32) -> Self {
        AttachEnergyToUnitRequest {
            account_unique_id,
            unit_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_unit_id(&self) -> i32 {
        self.unit_id
    }
}
