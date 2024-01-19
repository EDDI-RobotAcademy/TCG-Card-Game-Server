use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientProgramExitResponse {
    does_client_exit_success: bool,
}

impl ClientProgramExitResponse {
    pub fn new(does_client_exit_success: bool) -> Self {
        ClientProgramExitResponse { does_client_exit_success }
    }

    pub fn does_client_exit_success(&self) -> bool {
        self.does_client_exit_success
    }
}