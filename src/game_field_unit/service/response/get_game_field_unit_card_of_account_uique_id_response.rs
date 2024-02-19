use crate::game_field_unit::entity::game_field_unit_card::GameFieldUnitCard;

#[derive(Debug, Clone)]
pub struct GetGameFieldUnitCardOfAccountUniqueIdResponse {
    game_field_unit_card: Vec<GameFieldUnitCard>,
}

impl GetGameFieldUnitCardOfAccountUniqueIdResponse {
    pub fn new(game_field_unit_card: Vec<GameFieldUnitCard>) -> Self {
        GetGameFieldUnitCardOfAccountUniqueIdResponse { game_field_unit_card }
    }

    pub fn get_game_field_unit_card(&self) -> &Vec<GameFieldUnitCard> {
        &self.game_field_unit_card
    }
}