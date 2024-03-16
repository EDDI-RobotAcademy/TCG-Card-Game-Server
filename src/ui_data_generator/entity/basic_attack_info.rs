use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackInfo {
    target_player_index: PlayerIndex,
    target_unit_index: i32,
    active_skill_index: i32,
    passive_skill_index: i32,
}

impl AttackInfo {
    pub fn new(target_player_index: PlayerIndex,
               target_unit_index: i32,
               active_skill_index: i32,
               passive_skill_index: i32,) -> Self {
        AttackInfo {
            target_player_index,
            target_unit_index,
            active_skill_index,
            passive_skill_index
        }
    }
}