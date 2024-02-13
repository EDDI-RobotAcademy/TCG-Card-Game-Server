use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnEndResponseForm {
    is_success: bool,
}

impl TurnEndResponseForm {
    pub fn new(is_success: bool) -> Self {
        TurnEndResponseForm { is_success }
    }
}
