#[derive(Clone)]
pub struct SummaryDeployPassiveSkillEffectRequest {
    unit_card_id: i32,
}

impl SummaryDeployPassiveSkillEffectRequest {
    pub fn new(unit_card_id: i32) -> Self {
        SummaryDeployPassiveSkillEffectRequest {
            unit_card_id
        }
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
}
