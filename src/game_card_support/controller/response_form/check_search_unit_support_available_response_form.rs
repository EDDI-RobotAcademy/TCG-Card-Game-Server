use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSearchUnitSupportAvailableResponseForm {
    is_success: bool,
    false_message_enum: i32,
    accessible_deck_unit_card_list: HashMap<i32, i32>,
}

impl CheckSearchUnitSupportAvailableResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               accessible_deck_unit_card_list: HashMap<i32, i32>,) -> Self {
        CheckSearchUnitSupportAvailableResponseForm {
            is_success,
            false_message_enum,
            accessible_deck_unit_card_list
        }
    }

    pub fn default() -> CheckSearchUnitSupportAvailableResponseForm {

        CheckSearchUnitSupportAvailableResponseForm::new(
            false,
            -1,
            HashMap::new())
    }

    pub fn from_false_response_with_message(false_message: FalseMessage) -> CheckSearchUnitSupportAvailableResponseForm {
        CheckSearchUnitSupportAvailableResponseForm::new(
            false,
            false_message as i32,
            HashMap::new())
    }
}