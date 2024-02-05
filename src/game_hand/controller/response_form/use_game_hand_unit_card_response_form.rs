use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandUnitCardResponseForm {
    is_success: bool,
}

impl UseGameHandUnitCardResponseForm {
    pub fn new(is_success: bool) -> Self { UseGameHandUnitCardResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}