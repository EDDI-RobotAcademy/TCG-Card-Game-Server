use crate::account::service::response::account_register_response::AccountRegisterResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseType {
    ACCOUNT_REGISTER(AccountRegisterResponse),
}