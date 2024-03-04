use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormDeployUnit {
    player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
}

impl NotifyFormDeployUnit {
    pub fn new(player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>) -> Self {

        NotifyFormDeployUnit {
            player_hand_use_map,
        }
    }
}