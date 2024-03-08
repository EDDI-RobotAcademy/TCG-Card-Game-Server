use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployUnitResponseForm {
    is_success: bool,
    index_list_of_passive_skill_to_handle: Vec<i32>,
}

impl DeployUnitResponseForm {
    pub fn new(
        is_success: bool,
        index_list_of_passive_skill_to_handle: Vec<i32>,
    ) -> Self {

        DeployUnitResponseForm {
            is_success,
            index_list_of_passive_skill_to_handle
        }
    }

    pub fn default() -> DeployUnitResponseForm {

        DeployUnitResponseForm::new(false, Vec::new())
    }
}