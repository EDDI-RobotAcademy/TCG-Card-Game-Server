#[derive(Clone)]
pub struct SummaryPassiveSkillEffectRequest {
    unit_card_id: i32,
}

impl SummaryPassiveSkillEffectRequest {
    pub fn new(unit_card_id: i32) -> Self {
        SummaryPassiveSkillEffectRequest {
            unit_card_id
        }
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
}
