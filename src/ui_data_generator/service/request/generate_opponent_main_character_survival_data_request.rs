use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;

#[derive(Debug)]
pub struct GenerateOpponentMainCharacterSurvivalDataRequest {
    main_character_status:StatusMainCharacterEnum
}

impl GenerateOpponentMainCharacterSurvivalDataRequest {
    pub fn new(main_character_status:StatusMainCharacterEnum) -> Self {
        GenerateOpponentMainCharacterSurvivalDataRequest {
            main_character_status

        }
    }

    pub fn get_main_character_status(&self) -> &StatusMainCharacterEnum { &self.main_character_status }

}