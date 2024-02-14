#[derive(Clone)]
pub struct SummarizeToolCardEffectRequest {
    tool_card_id: i32,
}

impl SummarizeToolCardEffectRequest {
    pub fn new(tool_card_id: i32) -> Self {
        SummarizeToolCardEffectRequest {
            tool_card_id
        }
    }

    pub fn get_tool_card_id(&self) -> i32 {
        self.tool_card_id
    }
}