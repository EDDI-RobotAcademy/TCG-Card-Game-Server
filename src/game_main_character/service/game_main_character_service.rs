use async_trait::async_trait;
use crate::game_main_character::service::request::apply_damage_to_main_character_request::ApplyDamageToMainCharacterRequest;
use crate::game_main_character::service::request::check_main_character_of_account_unique_id_request::CheckMainCharacterOfAccountUniqueIdRequest;
use crate::game_main_character::service::response::apply_damage_to_main_character_response::ApplyDamageToMainCharacterResponse;
use crate::game_main_character::service::response::check_main_character_of_account_unique_id_response::CheckMainCharacterOfAccountUniqueIdResponse;

#[async_trait]
pub trait GameMainCharacterService {
    async fn apply_damage_to_main_character(&self, apply_damage_to_main_character_request: ApplyDamageToMainCharacterRequest) -> ApplyDamageToMainCharacterResponse;
    async fn check_main_character_of_account_unique_id(&mut self, get_main_character_of_account_unique_id_request: CheckMainCharacterOfAccountUniqueIdRequest) -> CheckMainCharacterOfAccountUniqueIdResponse;
}