#[derive(Clone)]
pub struct SummaryActiveSkillEffectRequest {
    active_skill_usage_card_id: i32,
    usage_skill_index: i32,
}

impl SummaryActiveSkillEffectRequest {
    pub fn new(active_skill_usage_card_id: i32,
               usage_skill_index: i32) -> Self {

        SummaryActiveSkillEffectRequest {
            active_skill_usage_card_id,
            usage_skill_index
        }
    }

    pub fn get_active_skill_usage_card_id(&self) -> i32 {
        self.active_skill_usage_card_id
    }

    pub fn get_usage_skill_index(&self) -> i32 {
        self.usage_skill_index
    }
}
