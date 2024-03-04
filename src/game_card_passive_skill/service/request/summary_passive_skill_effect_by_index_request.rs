#[derive(Clone)]
pub struct SummaryPassiveSkillEffectByIndexRequest {
    unit_card_id: i32,
    usage_skill_index: i32,
}

impl SummaryPassiveSkillEffectByIndexRequest {
    pub fn new(unit_card_id: i32, usage_skill_index: i32) -> Self {
        SummaryPassiveSkillEffectByIndexRequest {
            unit_card_id,
            usage_skill_index
        }
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
    pub fn get_usage_skill_index(&self) -> i32 {
        self.usage_skill_index
    }

}
