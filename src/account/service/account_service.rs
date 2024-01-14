use async_trait::async_trait;
use crate::account::service::request::account_register_request::AccountRegisterRequest;

#[async_trait]
pub trait AccountService {
    async fn account_register(&self, account_register_request: AccountRegisterRequest);
}