use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerDeckCardUseListInfo {
    player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>,
}

impl PlayerDeckCardUseListInfo {
    pub fn new(player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>) -> Self {
        PlayerDeckCardUseListInfo {
            player_deck_card_use_list_map
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