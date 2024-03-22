use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;

#[derive(Debug, Clone)]
pub struct IsUnitBasicAttackPossibleResponse {
    is_possible: bool,
    false_message_enum: FalseMessage,

}

impl IsUnitBasicAttackPossibleResponse {
    pub fn new(is_possible: bool ,false_message_enum: FalseMessage) -> Self { IsUnitBasicAttackPossibleResponse { is_possible, false_message_enum } }
    pub fn is_possible(&self) -> bool { self.is_possible }
    pub fn false_message_enum(&self) -> FalseMessage { self.false_message_enum }
}