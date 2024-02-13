use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;

use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::game_card_active_skill::controller::game_card_active_skill_controller::GameCardActiveSkillController;
use crate::game_card_active_skill::controller::request_form::targeting_active_skill_request_form::TargetingActiveSkillRequestForm;
use crate::game_card_active_skill::controller::response_form::targeting_active_skill_response_form::TargetingActiveSkillResponseForm;
use crate::game_card_active_skill::service::game_card_active_skill_service::GameCardActiveSkillService;
use crate::game_card_active_skill::service::game_card_active_skill_service_impl::GameCardActiveSkillServiceImpl;
use crate::game_card_energy::controller::response_form::attach_general_energy_card_response_form::AttachGeneralEnergyCardResponseForm;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardActiveSkillControllerImpl {
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_card_active_skill_service: Arc<AsyncMutex<GameCardActiveSkillServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
}

impl GameCardActiveSkillControllerImpl {
    pub fn new(battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_card_active_skill_service: Arc<AsyncMutex<GameCardActiveSkillServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>) -> Self {

        GameCardActiveSkillControllerImpl {
            battle_room_service,
            game_field_unit_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_card_active_skill_service,
            game_protocol_validation_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardActiveSkillControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardActiveSkillControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardActiveSkillControllerImpl::new(
                            BattleRoomServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameCardActiveSkillServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }
}

#[async_trait]
impl GameCardActiveSkillController for GameCardActiveSkillControllerImpl {
    async fn request_targeting_active_skill(&self, targeting_active_skill_request_form: TargetingActiveSkillRequestForm) -> TargetingActiveSkillResponseForm {
        println!("GameCardActiveSkillControllerImpl: request_targeting_active_skill()");

        // 1. 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(targeting_active_skill_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            return TargetingActiveSkillResponseForm::new(false)
        }

        let unit_card_index_string = targeting_active_skill_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        // 2. TODO: 프로토콜 검증 할 때가 아니라 패스

        // 3. Active Skill을 시전하는 인덱스 유닛의 카드 id 값 파악
        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let add_unit_to_game_field_response = game_field_unit_service_guard
            .find_active_skill_usage_unit_id_by_index(
                targeting_active_skill_request_form.to_find_active_skill_usage_unit_id_by_index_request(
                    account_unique_id, unit_card_index)).await;

        // 4. Active Skill Summary 획득
        let usage_skill_index_string = targeting_active_skill_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();

        let mut game_card_active_skill_service_guard = self.game_card_active_skill_service.lock().await;
        let summary_active_skill_effect_response = game_card_active_skill_service_guard
            .summary_active_skill(
                targeting_active_skill_request_form.to_summary_active_skill_effect_response(
                    unit_card_index, usage_skill_index)).await;

        // 5. 상대 고유값 찾기
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(
            targeting_active_skill_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;

        // 6. Attack Opponent
        let target_card_index_string = targeting_active_skill_request_form.get_opponent_target_card_index();
        let target_card_index = target_card_index_string.parse::<i32>().unwrap();

        game_field_unit_service_guard.apply_damage_to_target_unit_index(
            targeting_active_skill_request_form.to_apply_damage_to_target_unit_index(
                find_opponent_by_account_id_response.get_opponent_unique_id(),
                target_card_index,
                summary_active_skill_effect_response.get_skill_damage())).await;

        // 7. 공격 당한 유닛 사망 판정

        // 8. 사망 판정 값이 참이라면 무덤으로 보내기

        // 9. Notify Opponent

        TargetingActiveSkillResponseForm::new(true)
    }
}