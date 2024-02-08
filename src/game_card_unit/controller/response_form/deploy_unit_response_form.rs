use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployUnitResponseForm {
    is_success: bool,
}

impl DeployUnitResponseForm {
    pub fn new(is_success: bool) -> Self { DeployUnitResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}