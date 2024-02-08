use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeleteResponse {
    is_success: bool,
    redis_token: String,
}

impl AccountDeleteResponse {
    pub fn new(is_success: bool, redis_token: String) -> Self {
        AccountDeleteResponse { is_success, redis_token }
    }

    pub fn get_redis_token(&self) -> &str {
        &self.redis_token
    }
}
