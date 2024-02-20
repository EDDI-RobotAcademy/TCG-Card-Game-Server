use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;

use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_field_unit::service::request::get_game_field_unit_card_of_account_uique_id_request::GetGameFieldUnitCardOfAccountUniqueIdRequest;
use crate::game_field_unit::service::request::judge_death_of_unit_request::JudgeDeathOfUnitRequest;
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
use crate::game_round::service::game_round_service::GameRoundService;
use crate::game_round::service::game_round_service_impl::GameRoundServiceImpl;

pub struct GameTurnControllerImpl {
    game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
    game_round_service: Arc<AsyncMutex<GameRoundServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,


}

impl GameTurnControllerImpl {
    pub fn new(game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
               game_round_service: Arc<AsyncMutex<GameRoundServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
             ) -> Self {

        GameTurnControllerImpl {
            game_turn_service,
            game_round_service,
            game_deck_service,
            game_field_energy_service,
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
                            GameRoundServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameFieldEnergyServiceImpl::get_instance(),
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
}

#[async_trait]
impl GameTurnController for GameTurnControllerImpl {
    async fn request_turn_end(&self, turn_end_request_form: TurnEndRequestForm) -> TurnEndResponseForm {
        // 1. 세션 검증
        let account_unique_id =
            self.is_valid_session(turn_end_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return TurnEndResponseForm::new(false)
        }

        // 2. opponent unique id 찾기
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                turn_end_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 3. 현재 요청한 사람이 이번 턴의 주도권을 가지고 있던 사람인지 검증
        // TODO: 다음 과정은 모든 액션에 추가되어야 함
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                turn_end_request_form.to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            return TurnEndResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        // 4. 자신의 필드 유닛들 중 턴 종료 시 데미지를 받는 케이스를 적용 (예: 화상 데미지)
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let apply_status_effect_damage_iteratively_response =
            game_field_unit_service_guard.apply_status_effect_damage_iteratively(
                turn_end_request_form
                    .to_apply_status_effect_damage_iteratively_request(
                        account_unique_id)).await;

        // TODO: 필드 유닛 Domain 에서 자신 필드 유닛을 순회하며 사망 판정 하는 feature 필요 (턴 종료 데미지와 연결)

        // 5. 자신 필드 유닛 Turn Action Value 초기화
        game_field_unit_service_guard.reset_turn_action_of_all_field_unit(
            turn_end_request_form
                .to_reset_turn_action_of_all_field_unit_request(account_unique_id)).await;

        drop(game_field_unit_service_guard);

        // 6. 당신의 라운드 증가
        let mut game_round_service_guard =
            self.game_round_service.lock().await;

        game_round_service_guard.next_game_round_object(
            turn_end_request_form.to_next_round_request()).await;

        drop(game_round_service_guard);

        // 7. 상대방의 턴 증가
        let mut game_turn_service_guard =
            self.game_turn_service.lock().await;

        game_turn_service_guard.next_turn(
            turn_end_request_form.to_next_turn_request(opponent_unique_id)).await;

        drop(game_turn_service_guard);

        // 8. 상대방이 덱에서 카드를 드로우
        let mut game_deck_service_guard =
            self.game_deck_service.lock().await;

        game_deck_service_guard.draw_cards_from_deck(
            turn_end_request_form.to_draw_cards_from_deck_request(opponent_unique_id)).await;

        drop(game_deck_service_guard);

        // 9. 상대방이 필드에너지 획득
        let game_field_energy_service_guard =
            self.game_field_energy_service.lock().await;

        game_field_energy_service_guard.add_field_energy_with_amount(
                turn_end_request_form.to_add_field_energy_request(opponent_unique_id)).await;

        drop(game_field_energy_service_guard);

        // 10. TODO: 턴 종료 상황에서 상태 이상으로 죽은 유닛들, 데미지 및 상대 턴 시작 등등을 알려줘야함 (Notify)

        TurnEndResponseForm::new(true)
    }
}