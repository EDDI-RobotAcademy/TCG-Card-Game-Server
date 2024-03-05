use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckOpponentMulliganStatusResponseForm {
    is_done: bool,
}

impl CheckOpponentMulliganStatusResponseForm {
    pub fn new(is_done: bool) -> Self {
        CheckOpponentMulliganStatusResponseForm { is_done }
    }
}
