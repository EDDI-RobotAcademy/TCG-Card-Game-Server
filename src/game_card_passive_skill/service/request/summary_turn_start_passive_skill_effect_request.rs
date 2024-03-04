#[derive(Clone)]
pub struct SummaryTurnStartPassiveSkillEffectRequest {
    unit_card_id: i32,
}

impl SummaryTurnStartPassiveSkillEffectRequest {
    pub fn new(unit_card_id: i32) -> Self {
        SummaryTurnStartPassiveSkillEffectRequest {
            unit_card_id
        }
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
}
