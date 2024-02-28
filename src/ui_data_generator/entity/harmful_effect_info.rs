use serde::{Deserialize, Serialize};
use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// 유닛이 적용받고 있는 해로운 효과 리스트
pub struct HarmfulStatusInfo {
    harmful_status_list: Vec<ExtraEffect>,
}

impl HarmfulStatusInfo {
    pub fn new(harmful_status_list: Vec<ExtraEffect>) -> Self {
        HarmfulStatusInfo {
            harmful_status_list
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_data() {
        todo!()
    }
}