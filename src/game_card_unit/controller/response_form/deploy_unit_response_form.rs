use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployUnitResponseForm {
    is_success: bool,
    number_of_passive_skill_to_handle: i32,
}

impl DeployUnitResponseForm {
    pub fn new(
        is_success: bool,
        number_of_passive_skill_to_handle: i32,
    ) -> Self {

        DeployUnitResponseForm {
            is_success,
            number_of_passive_skill_to_handle
        }
    }
}