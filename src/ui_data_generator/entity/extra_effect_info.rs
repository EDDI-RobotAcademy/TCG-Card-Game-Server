use serde::{Deserialize, Serialize};
use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtraEffectInfo {
    extra_effect_list: Vec<ExtraEffect>,
}

impl ExtraEffectInfo {
    pub fn new(extra_effect_list: Vec<ExtraEffect>) -> Self {
        ExtraEffectInfo {
            extra_effect_list
        }
    }

}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_data() {
        let vec = vec![ExtraEffect::Freeze, ExtraEffect::DarkFire];
        let extra_effect_info = ExtraEffectInfo::new(vec);
        println!("{:?}", json!(extra_effect_info));
    }
}