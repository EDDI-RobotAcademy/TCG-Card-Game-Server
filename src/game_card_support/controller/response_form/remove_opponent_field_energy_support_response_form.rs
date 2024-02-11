use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveOpponentFieldEnergySupportResponseForm {
    is_success: bool,
}

impl RemoveOpponentFieldEnergySupportResponseForm {
    pub fn new(is_success: bool) -> Self { RemoveOpponentFieldEnergySupportResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}