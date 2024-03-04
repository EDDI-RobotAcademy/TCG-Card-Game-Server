use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RockPaperScissorsResponseForm {
    is_success: bool
}

impl RockPaperScissorsResponseForm {
    pub fn new(is_success: bool) -> Self { RockPaperScissorsResponseForm { is_success } }
}