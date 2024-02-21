use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDistributeCardsResponseForm {
    is_success: bool
}

impl EventDistributeCardsResponseForm {
    pub fn new(is_success: bool) -> Self { EventDistributeCardsResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }

}