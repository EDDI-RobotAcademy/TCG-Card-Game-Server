use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerHandCardUseInfo {
    player_hand_card_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
}

impl PlayerHandCardUseInfo {
    pub fn new(player_hand_card_use_map: HashMap<PlayerIndex, UsedHandCardInfo>) -> Self {
        PlayerHandCardUseInfo {
            player_hand_card_use_map
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_data() {
        todo!()
    }
}