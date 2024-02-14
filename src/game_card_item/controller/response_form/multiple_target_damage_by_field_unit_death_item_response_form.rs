use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleTargetDamageByFieldUnitDeathItemResponseForm {
    is_success: bool,
}

impl MultipleTargetDamageByFieldUnitDeathItemResponseForm {
    pub fn new(is_success: bool) -> Self { MultipleTargetDamageByFieldUnitDeathItemResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}