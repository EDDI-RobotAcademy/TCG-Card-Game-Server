#[derive(Debug)]
pub struct FindActiveSkillUsageUnitIdByIndexRequest {
    account_unique_id: i32,
    active_skill_usage_unit_index: i32
}

impl FindActiveSkillUsageUnitIdByIndexRequest {
    pub fn new(account_unique_id: i32, active_skill_usage_unit_index: i32) -> Self {
        FindActiveSkillUsageUnitIdByIndexRequest {
            account_unique_id,
            active_skill_usage_unit_index
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_active_skill_usage_unit_index(&self) -> i32 {
        self.active_skill_usage_unit_index
    }
}
