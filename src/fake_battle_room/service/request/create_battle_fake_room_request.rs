#[derive(Clone)]
pub struct CreateFakeBattleRoomRequest {
    first_fake_account_id: i32,
    second_fake_account_id: i32,
}

impl CreateFakeBattleRoomRequest {
    pub fn new(first_fake_account_id: i32,
               second_fake_account_id: i32) -> Self {

        CreateFakeBattleRoomRequest {
            first_fake_account_id,
            second_fake_account_id
        }
    }

    pub fn get_first_fake_account_id(&self) -> i32 {
        self.first_fake_account_id
    }

    pub fn get_second_fake_account_id(&self) -> i32 {
        self.second_fake_account_id
    }
}
