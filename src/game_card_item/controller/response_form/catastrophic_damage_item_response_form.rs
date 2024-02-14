use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatastrophicDamageItemResponseForm {
    is_success: bool,
}

impl CatastrophicDamageItemResponseForm {
    pub fn new(is_success: bool) -> Self { CatastrophicDamageItemResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}