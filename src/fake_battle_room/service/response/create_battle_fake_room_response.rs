use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFakeBattleRoomResponse {
    is_success: bool,
}

impl CreateFakeBattleRoomResponse {
    pub fn new(is_success: bool) -> Self {
        CreateFakeBattleRoomResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
