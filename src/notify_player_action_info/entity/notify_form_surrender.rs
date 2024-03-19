use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormSurrender {
    is_done: bool,
}

impl NotifyFormSurrender {
    pub fn new(is_done: bool) -> Self {
        NotifyFormSurrender { is_done }
    }
}