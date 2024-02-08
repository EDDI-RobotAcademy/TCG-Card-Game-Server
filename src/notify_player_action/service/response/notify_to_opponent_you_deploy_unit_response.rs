use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouDeployUnitResponse {
    is_success: bool,
}

impl NotifyToOpponentYouDeployUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouDeployUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
