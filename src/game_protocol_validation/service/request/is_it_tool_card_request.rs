#[derive(Debug)]
pub struct IsItToolCardRequest {
    tool_card_number: i32,
}

impl IsItToolCardRequest {
    pub fn new(tool_card_number: i32) -> Self {
        IsItToolCardRequest {
            tool_card_number
        }
    }

    pub fn get_tool_card_number(&self) -> i32 {
        self.tool_card_number
    }
}