use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToDrawSupportUsage {
    usage_support_card_id: i32,
    draw_card_count: i32,
}

impl NotifyOpponentToDrawSupportUsage {
    pub fn new(usage_support_card_id: i32, draw_card_count: i32) -> Self {
        NotifyOpponentToDrawSupportUsage {
            usage_support_card_id,
            draw_card_count,
        }
    }
    pub fn get_usage_support_card_id(&self) -> i32 { self.usage_support_card_id }
    pub fn get_draw_card_count(&self) -> i32 { self.draw_card_count }
}