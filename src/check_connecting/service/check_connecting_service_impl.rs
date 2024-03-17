use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::check_connecting::repository::check_connecting_repository::CheckConnectingRepository;
use crate::check_connecting::repository::check_connecting_repository_impl::CheckConnectingRepositoryImpl;

use crate::check_connecting::service::check_connecting_service::CheckConnectingService;

use crate::check_connecting::service::request::check_connecting_status_request::CheckConnectingStatusRequest;
use crate::check_connecting::service::request::checked_response_request::CheckedResponseRequest;
use crate::check_connecting::service::response::check_connecting_status_response::CheckConnectingStatusResponse;
use crate::check_connecting::service::response::checked_response_response::CheckedResponseResponse;


#[derive(Clone)]
pub struct CheckConnectingServiceImpl {
    check_connecting_repository: Arc<AsyncMutex<CheckConnectingRepositoryImpl>>,
}

impl CheckConnectingServiceImpl {
    pub fn new(check_connecting_repository: Arc<AsyncMutex<CheckConnectingRepositoryImpl>>,
               ) -> Self {

        CheckConnectingServiceImpl {
            check_connecting_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<CheckConnectingServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CheckConnectingServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CheckConnectingServiceImpl::new(
                            CheckConnectingRepositoryImpl::get_instance(),
                            )));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CheckConnectingService for CheckConnectingServiceImpl {


    async fn check_connecting_status(&self, check_connecting_status_request: CheckConnectingStatusRequest) -> CheckConnectingStatusResponse {
        println!("CheckConnectingServiceImpl: check_connecting_status()");

        let mut check_connecting_repository_guard = self.check_connecting_repository.lock().await;

        let account_unique_id = check_connecting_repository_guard.
            find_account_unique_id_by_address(
                check_connecting_status_request.get_address()).await.unwrap();

        let client_socket = check_connecting_repository_guard.
            find_client_socket_by_account_unique_id(
                account_unique_id).await.unwrap();

        if Some(client_socket.clone()).is_some() {
            check_connecting_repository_guard.send_message_for_checking_connect(client_socket).await;
        }

        drop(check_connecting_repository_guard);
        return CheckConnectingStatusResponse::new(true)
    }

    async fn checked_response(&self, checked_response_request: CheckedResponseRequest) -> CheckedResponseResponse {
        println!("CheckConnectingServiceImpl: checked_response()");

        println!("Checked_response_for_connecting");
        CheckedResponseResponse::new(true)
    }
}