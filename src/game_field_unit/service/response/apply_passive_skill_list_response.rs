#[derive(Debug, Clone)]
pub struct ApplyPassiveSkillListResponse {
    is_success: bool
}

impl ApplyPassiveSkillListResponse {
    pub fn new(is_success: bool) -> Self {
        ApplyPassiveSkillListResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
