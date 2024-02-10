use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUnitSupportResponseForm {
    is_success: bool,
}

impl SearchUnitSupportResponseForm {
    pub fn new(is_success: bool) -> Self { SearchUnitSupportResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}