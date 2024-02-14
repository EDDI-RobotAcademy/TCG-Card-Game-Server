use async_trait::async_trait;
use crate::game_main_character::service::request::apply_damage_to_main_character_request::ApplyDamageToMainCharacterRequest;
use crate::game_main_character::service::response::apply_damage_to_main_character_response::ApplyDamageToMainCharacterResponse;

#[async_trait]
pub trait GameMainCharacterService {
    async fn apply_damage_to_main_character(&self, apply_damage_to_main_character_request: ApplyDamageToMainCharacterRequest) -> ApplyDamageToMainCharacterResponse;
}