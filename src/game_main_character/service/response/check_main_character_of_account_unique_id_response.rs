use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;

#[derive(Debug, Clone)]
pub struct CheckMainCharacterOfAccountUniqueIdResponse {
    status_main_character: StatusMainCharacterEnum,
}

impl CheckMainCharacterOfAccountUniqueIdResponse {
    pub fn new(status_main_character: StatusMainCharacterEnum) -> Self {
        CheckMainCharacterOfAccountUniqueIdResponse { status_main_character }
    }

    pub fn get_status_main_character(&self) -> StatusMainCharacterEnum {
        self.status_main_character
    }
}