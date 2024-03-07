use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormMulliganEnd {
    is_done: bool,
}

impl NotifyFormMulliganEnd {
    pub fn new(is_done: bool) -> Self {
        NotifyFormMulliganEnd { is_done }
    }
}
