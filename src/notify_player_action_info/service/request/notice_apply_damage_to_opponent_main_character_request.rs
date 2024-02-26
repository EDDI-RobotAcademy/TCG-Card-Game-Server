use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;

#[derive(Debug)]
pub struct NoticeApplyDamageToOpponentMainCharacterRequest {
    opponent_unique_id: i32,
    damage: i32,
    updated_health_point: i32,
    opponent_survival_status: StatusMainCharacterEnum,
}

impl NoticeApplyDamageToOpponentMainCharacterRequest {
    pub fn new(opponent_unique_id: i32,
               damage: i32,
               updated_health_point: i32,
               opponent_survival_status: StatusMainCharacterEnum) -> Self {
        NoticeApplyDamageToOpponentMainCharacterRequest {
            opponent_unique_id,
            damage,
            updated_health_point,
            opponent_survival_status
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_damage(&self) -> i32 { self.damage }

    pub fn get_updated_health_point(&self) -> i32 { self.updated_health_point }

    pub fn get_opponent_survival_status(&self) -> &StatusMainCharacterEnum {
        &self.opponent_survival_status
    }
}