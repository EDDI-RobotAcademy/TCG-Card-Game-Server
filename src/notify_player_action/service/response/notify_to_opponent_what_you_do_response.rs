use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentWhatYouDoResponse {
    is_success: bool,
}

impl NotifyToOpponentWhatYouDoResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentWhatYouDoResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
