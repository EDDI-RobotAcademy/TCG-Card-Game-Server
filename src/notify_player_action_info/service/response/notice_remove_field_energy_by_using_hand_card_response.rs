use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeRemoveFieldEnergyByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeRemoveFieldEnergyByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeRemoveFieldEnergyByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}