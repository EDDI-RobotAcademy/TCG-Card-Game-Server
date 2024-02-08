use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDeathItemResponseForm {
    is_success: bool,
}

impl TargetDeathItemResponseForm {
    pub fn new(is_success: bool) -> Self { TargetDeathItemResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}