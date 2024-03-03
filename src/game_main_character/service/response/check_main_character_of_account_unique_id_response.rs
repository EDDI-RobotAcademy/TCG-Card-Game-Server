use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;

#[derive(Debug, Clone)]
pub struct CheckMainCharacterOfAccountUniqueIdResponse {
    current_health_point: i32,
    status_main_character: StatusMainCharacterEnum,
}

impl CheckMainCharacterOfAccountUniqueIdResponse {
    pub fn new(
        current_health_point: i32,
        status_main_character: StatusMainCharacterEnum
    ) -> Self {

        CheckMainCharacterOfAccountUniqueIdResponse {
            current_health_point,
            status_main_character,
        }
    }

    pub fn get_current_health_point(&self) -> i32 {
        self.current_health_point
    }

    pub fn get_status_main_character(&self) -> &StatusMainCharacterEnum {
        &self.status_main_character
    }
}