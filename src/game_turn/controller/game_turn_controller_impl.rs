use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;

use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_field_unit::service::request::get_game_field_unit_card_of_account_uique_id_request::GetGameFieldUnitCardOfAccountUniqueIdRequest;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum::{Death, Survival};
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;

use crate::game_turn::controller::game_turn_controller::GameTurnController;

use crate::game_turn::controller::request_form::turn_end_request_form::TurnEndRequestForm;

use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;
use crate::game_turn::service::game_turn_service::GameTurnService;

use crate::game_turn::service::game_turn_service_impl::GameTurnServiceImpl;
use crate::game_turn::service::request::first_turn_decision_request::FirstTurnDecisionRequest;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::game_main_character::service::game_main_character_service_impl::GameMainCharacterServiceImpl;
use crate::game_main_character::service::request::check_main_character_of_account_unique_id_request::CheckMainCharacterOfAccountUniqueIdRequest;

pub struct GameTurnControllerImpl {
    game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,


}

impl GameTurnControllerImpl {
    pub fn new(game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
             ) -> Self {

        GameTurnControllerImpl {
            game_turn_service,
            battle_room_service,
            game_field_unit_service,
            redis_in_memory_service,
            game_protocol_validation_service,
            game_main_character_service,

        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameTurnControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameTurnControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameTurnControllerImpl::new(
                            GameTurnServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            GameMainCharacterServiceImpl::get_instance(),
                            )));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;
        drop(redis_in_memory_service_guard);
        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    async fn check_current_health_field_unit_card(&self, account_id: i32) {
        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let account_unique_id_game_filed_unit_request = GetGameFieldUnitCardOfAccountUniqueIdRequest::new(account_id);
        let game_field_unit_list_of_account_unique_id_response = game_field_unit_service_guard.get_game_field_unit_card_of_account_unique_id(account_unique_id_game_filed_unit_request);
        let game_field_unit_list_of_account_unique_id = game_field_unit_list_of_account_unique_id_response.await.get_game_field_unit_card().clone();
        let game_field_unit_list_of_account_unique_id_option = game_field_unit_list_of_account_unique_id.get(1);

        if let Some(_) = game_field_unit_list_of_account_unique_id_option {

            for unit in game_field_unit_list_of_account_unique_id {

                let mut curren_health_of_unit = unit.get_unit_health_point().get_current_health_point();

                if curren_health_of_unit <= 0 {
                    let died_card_id = unit.get_card();
                    println!("Place to TOMB : account_unique_id: {:?}, card_id: {:?}", account_id, died_card_id);
                    PlaceToTombRequest::new(account_id,died_card_id);
                }
            }
        }
    }
}

#[async_trait]
impl GameTurnController for GameTurnControllerImpl {
    async fn request_turn_end(&self, turn_end_request_form: TurnEndRequestForm) -> TurnEndResponseForm {
        // 1. Redis에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(turn_end_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return TurnEndResponseForm::new(false)
        }

        // 6. opponent id도 찾아야함
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(
            turn_end_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;
        let opponent_account_unique_id = find_opponent_by_account_id_response.get_opponent_unique_id();

        // 2. 현재 요청한 사람이 이번 턴의 주도권을 가지고 있던 사람인지 검증
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let account_request = IsThisYourTurnRequest::new(account_unique_id);
        let is_this_your_turn_response = game_protocol_validation_service_guard.is_this_your_turn(account_request);
        if !is_this_your_turn_response.await.get_is_success() {
            return TurnEndResponseForm::new(false)
        }

        // 3. 자신의 필드 유닛들 중 턴 종료 시 데미지를 받는 케이스를 적용 (현재 상황에서 화상 데미지)
        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let apply_status_effect_damage_iteratively_response = game_field_unit_service_guard.apply_status_effect_damage_iteratively(
            turn_end_request_form.to_apply_status_effect_damage_iteratively_request(
                account_unique_id)).await;

        // 4. 본체 사망 여부 정보를 가져옵니다.
        let mut game_main_character_service_guard = self.game_main_character_service.lock().await;
        let check_main_character_request_by_account_unique_id = CheckMainCharacterOfAccountUniqueIdRequest::new(account_unique_id);
        let status_account_unique_id = game_main_character_service_guard
            .check_main_character_of_account_unique_id(check_main_character_request_by_account_unique_id).await.get_status_main_character();

        let mut game_main_character_service_guard = self.game_main_character_service.lock().await;
        let check_main_character_request_by_opponent_account_unique_id = CheckMainCharacterOfAccountUniqueIdRequest::new(opponent_account_unique_id);
        let status_opponent_account_unique_id = game_main_character_service_guard
            .check_main_character_of_account_unique_id(check_main_character_request_by_opponent_account_unique_id).await.get_status_main_character();


        // TODO: 본체 상태 조건에 따라 승패 판정 요청할 것
        if status_account_unique_id == Death && status_opponent_account_unique_id == Death {
        }
        if status_account_unique_id == Survival && status_opponent_account_unique_id == Death {
        }
        if status_account_unique_id == Death && status_opponent_account_unique_id == Survival {
        }

        // 5. 죽은 유닛들이 있는지 전체 순회하며 확인하여 죽은 유닛은 무덤으로 배치
        self.check_current_health_field_unit_card(account_unique_id).await;
        self.check_current_health_field_unit_card(opponent_account_unique_id).await;

        // 7. 당신의 턴 증가
        let mut game_turn_service_guard = self.game_turn_service.lock().await;
        let next_turn_response = game_turn_service_guard
            .next_turn(turn_end_request_form.to_next_turn_request(account_unique_id)).await;

        // 8. 상대방의 턴 증가
        let next_turn_response = game_turn_service_guard
            .next_turn(turn_end_request_form.to_next_turn_request(opponent_account_unique_id)).await;

        // 9. 턴 종료 상황에서 상태 이상으로 죽은 유닛들, 데미지 등등을 알려줘야함

        TurnEndResponseForm::new(true)
    }
}