use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerHandUseInfo {
    player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
}

impl PlayerHandUseInfo {
    pub fn new(player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>) -> Self {
        PlayerHandUseInfo {
            player_hand_use_map
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