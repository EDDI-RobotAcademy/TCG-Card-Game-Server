use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::client_program::service::client_program_service::ClientProgramService;
use crate::client_program::service::request::client_program_exit_request::ClientProgramExitRequest;
use crate::client_program::service::response::client_program_exit_response::ClientProgramExitResponse;

pub struct ClientProgramServiceImpl;

impl ClientProgramServiceImpl {
    pub fn new() -> Self {
        ClientProgramServiceImpl { }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ClientProgramServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ClientProgramServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ClientProgramServiceImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ClientProgramService for ClientProgramServiceImpl {
    async fn client_exit_program(&self, client_program_exit_request: ClientProgramExitRequest) -> ClientProgramExitResponse {
        println!("ClientProgramServiceImpl: client_exit_program()");

        ClientProgramExitResponse::new(true)
    }
}
