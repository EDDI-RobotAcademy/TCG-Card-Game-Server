use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;

#[derive(Debug, Clone)]
pub struct IsUsingActiveSkillPossibleResponse {
    is_possible: bool,
    false_message_enum: FalseMessage,
}

impl IsUsingActiveSkillPossibleResponse {
    pub fn new(is_possible: bool ,false_message_enum: FalseMessage) -> Self { IsUsingActiveSkillPossibleResponse { is_possible, false_message_enum } }
    pub fn is_possible(&self) -> bool { self.is_possible }
    pub fn false_message_enum(&self) -> FalseMessage {self.false_message_enum}

}