use crate::account::service::response::account_register_response::AccountRegisterResponse;
use serde::{Deserialize, Serialize};
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::account::service::response::account_logout_response::AccountLogoutResponse;
use crate::client_program::service::response::client_program_exit_response::ClientProgramExitResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    ACCOUNT_REGISTER(AccountRegisterResponse),
    ACCOUNT_LOGIN(AccountLoginResponse),
    ACCOUNT_SESSION_LOGIN(AccountLoginResponse),
    ACCOUNT_LOGOUT(AccountLogoutResponse),

    PROGRAM_EXIT(ClientProgramExitResponse),
}