#[derive(Debug)]
pub struct AcquireUnitAttackPointRequest {
    account_unique_id: i32,
    attacker_unit_index: i32
}

impl AcquireUnitAttackPointRequest {
    pub fn new(account_unique_id: i32, attacker_unit_index: i32) -> Self {
        AcquireUnitAttackPointRequest {
            account_unique_id,
            attacker_unit_index
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_attacker_unit_index(&self) -> i32 {
        self.attacker_unit_index
    }
}
