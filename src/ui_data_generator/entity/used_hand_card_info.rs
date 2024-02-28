use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsedHandCardInfo {
    card_id: i32,
    card_kind: i32
}

impl UsedHandCardInfo {
    pub fn new(card_id: i32,
               card_kind: i32) -> Self {
        UsedHandCardInfo {
            card_id,
            card_kind
        }
    }
}