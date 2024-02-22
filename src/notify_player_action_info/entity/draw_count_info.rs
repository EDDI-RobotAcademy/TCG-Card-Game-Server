use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawCountInfo {
    draw_count: i32
}

impl DrawCountInfo {
    pub fn new(draw_count: i32) -> Self {
        DrawCountInfo {
            draw_count
        }
    }
}