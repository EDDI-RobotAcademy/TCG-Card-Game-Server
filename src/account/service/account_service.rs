use async_trait::async_trait;

use crate::account::service::request::account_session_logout_request::AccountSessionLogoutRequest;
use crate::account::service::request::account_session_login_request::AccountSessionLoginRequest;

use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::account::service::request::account_delete_request::AccountDeleteRequest;
use crate::account::service::request::account_modify_request::AccountModifyRequest;
use crate::account::service::request::account_login_request::AccountLoginRequest;

use crate::account::service::response::account_register_response::AccountRegisterResponse;
use crate::account::service::response::account_logout_response::AccountLogoutResponse;
use crate::account::service::response::account_delete_response::AccountDeleteResponse;
use crate::account::service::response::account_modify_response::AccountModifyResponse;
use crate::account::service::response::account_login_response::AccountLoginResponse;

#[async_trait]
pub trait AccountService {
    async fn account_register(&self, account_register_request: AccountRegisterRequest) -> AccountRegisterResponse;
    async fn account_login(&self, account_login_request: AccountLoginRequest) -> AccountLoginResponse;
    async fn account_logout(&self, account_login_request: AccountLoginRequest) -> AccountLogoutResponse;
    async fn account_session_login(&self, account_session_login_request: AccountSessionLoginRequest) -> AccountLoginResponse;
    async fn account_session_logout(&self, account_logout_request: AccountSessionLogoutRequest) -> AccountLogoutResponse;
    async fn account_delete(&self, account_delete_request: AccountDeleteRequest) -> AccountDeleteResponse;
    async fn account_modify(&self, account_modify_request: AccountModifyRequest) -> AccountModifyResponse;
}