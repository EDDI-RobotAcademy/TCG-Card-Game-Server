use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployUnitResponseForm {
    is_success: bool,
    false_message_enum: i32,
    index_list_of_passive_skill_to_handle: Vec<i32>,
}

impl DeployUnitResponseForm {
    pub fn new(
        is_success: bool,
        false_message_enum: i32,
        index_list_of_passive_skill_to_handle: Vec<i32>,
    ) -> Self {

        DeployUnitResponseForm {
            is_success,
            false_message_enum,
            index_list_of_passive_skill_to_handle
        }
    }

    pub fn default() -> DeployUnitResponseForm {

        DeployUnitResponseForm::new(false, -1, Vec::new())
    }
    pub fn from_response_with_message(false_message: FalseMessage) -> DeployUnitResponseForm {

        DeployUnitResponseForm::new(false, false_message as i32, Vec::new())
    }
}