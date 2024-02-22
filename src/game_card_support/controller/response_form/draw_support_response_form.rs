use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawSupportResponseForm {
    is_success: bool,
}

impl DrawSupportResponseForm {
    pub fn new(is_success: bool) -> Self { DrawSupportResponseForm { is_success } }
}