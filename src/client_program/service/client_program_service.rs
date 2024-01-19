use async_trait::async_trait;
use crate::client_program::service::request::client_program_exit_request::ClientProgramExitRequest;
use crate::client_program::service::response::client_program_exit_response::ClientProgramExitResponse;

#[async_trait]
pub trait ClientProgramService {
    async fn client_exit_program(&self, client_program_exit_request: ClientProgramExitRequest) -> ClientProgramExitResponse;
}