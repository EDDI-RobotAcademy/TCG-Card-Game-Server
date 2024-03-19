use async_trait::async_trait;
use crate::check_connecting::service::request::check_connecting_status_request::CheckConnectingStatusRequest;
use crate::check_connecting::service::request::checked_response_request::CheckedResponseRequest;
use crate::check_connecting::service::response::check_connecting_status_response::CheckConnectingStatusResponse;
use crate::check_connecting::service::response::checked_response_response::CheckedResponseResponse;

#[async_trait]
pub trait CheckConnectingService: Send + Sync {
    async fn check_connecting_status(&self, check_connecting_status_request: CheckConnectingStatusRequest) -> CheckConnectingStatusResponse;
    async fn checked_response(&self, checked_response_request: CheckedResponseRequest) -> CheckedResponseResponse;
}