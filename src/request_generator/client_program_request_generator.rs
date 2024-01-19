use serde_json::Value as JsonValue;
use crate::client_program::service::request::client_program_exit_request::ClientProgramExitRequest;

pub fn create_client_program_exit_request(data: &JsonValue) -> Option<ClientProgramExitRequest> {
    Some(ClientProgramExitRequest::new())
}

