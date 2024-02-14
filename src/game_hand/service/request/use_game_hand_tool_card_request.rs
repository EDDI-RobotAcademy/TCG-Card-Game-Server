#[derive(Debug)]
pub struct UseGameHandToolCardRequest {
    account_unique_id: i32,
    tool_card_id: i32,
}

impl UseGameHandToolCardRequest {
    pub fn new(account_unique_id: i32, tool_card_id: i32) -> Self {
        UseGameHandToolCardRequest {
            account_unique_id,
            tool_card_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_tool_card_id(&self) -> i32 {
        self.tool_card_id
    }
}