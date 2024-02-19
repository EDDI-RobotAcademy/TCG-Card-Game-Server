use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RockpaperscissorsResponseForm {
    is_success:bool
}

impl RockpaperscissorsResponseForm {
    pub fn new(is_success: bool) -> Self {
        RockpaperscissorsResponseForm
        { is_success }
    }
    pub fn get_is_success(&self) -> bool { self.is_success }

}