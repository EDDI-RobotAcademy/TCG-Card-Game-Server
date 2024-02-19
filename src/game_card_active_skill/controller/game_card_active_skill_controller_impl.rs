use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;

use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::card_race::service::card_race_service::CardRaceService;
use crate::card_race::service::card_race_service_impl::CardRaceServiceImpl;
use crate::game_card_active_skill::controller::game_card_active_skill_controller::GameCardActiveSkillController;
use crate::game_card_active_skill::controller::request_form::non_targeting_active_skill_request_form::NonTargetingActiveSkillRequestForm;
use crate::game_card_active_skill::controller::request_form::targeting_active_skill_request_form::TargetingActiveSkillRequestForm;
use crate::game_card_active_skill::controller::response_form::non_targeting_active_skill_response_form::NonTargetingActiveSkillResponseForm;
use crate::game_card_active_skill::controller::response_form::targeting_active_skill_response_form::TargetingActiveSkillResponseForm;
use crate::game_card_active_skill::entity::active_skill_type::ActiveSkillType;
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
    card_race_service: Arc<AsyncMutex<CardRaceServiceImpl>>,
}

impl GameCardActiveSkillControllerImpl {
    pub fn new(battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_card_active_skill_service: Arc<AsyncMutex<GameCardActiveSkillServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               card_race_service: Arc<AsyncMutex<CardRaceServiceImpl>>) -> Self {

        GameCardActiveSkillControllerImpl {
            battle_room_service,
            game_field_unit_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_card_active_skill_service,
            game_protocol_validation_service,
            card_race_service
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
                            GameProtocolValidationServiceImpl::get_instance(),
                            CardRaceServiceImpl::get_instance())));
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
    async fn request_targeting_active_skill(
        &self, targeting_active_skill_request_form: TargetingActiveSkillRequestForm) -> TargetingActiveSkillResponseForm {

        println!("GameCardActiveSkillControllerImpl: request_targeting_active_skill()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(targeting_active_skill_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return TargetingActiveSkillResponseForm::new(false)
        }

        let unit_card_index_string = targeting_active_skill_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        // TODO: 프로토콜 검증 할 때가 아니라 패스

        // Action 가능한 턴인지 판별
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let check_turn_action_response =
            game_field_unit_service_guard.check_turn_action(
                targeting_active_skill_request_form
                    .to_check_turn_action_request(
                        account_unique_id,
                        unit_card_index)).await;

        if check_turn_action_response.has_already_taken_action() {
            println!("해당 유닛은 이미 액션을 취했습니다.");
            return TargetingActiveSkillResponseForm::new(false)
        }

        // Active Skill Summary 획득
        let usage_skill_index_string = targeting_active_skill_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();

        let mut game_card_active_skill_service_guard =
            self.game_card_active_skill_service.lock().await;

        let summary_active_skill_effect_response =
            game_card_active_skill_service_guard.summary_active_skill(
                targeting_active_skill_request_form
                    .to_summary_active_skill_effect_response(
                        unit_card_index,
                        usage_skill_index)).await;

        drop(game_card_active_skill_service_guard);

        // 스킬 사용에 필요한 에너지만큼 가지고 있는지 판정
        let required_energy_race_to_use_skill =
            *summary_active_skill_effect_response.get_required_energy().get_required_energy_race();
        let required_energy_count_to_use_skill =
            summary_active_skill_effect_response.get_required_energy().get_required_energy_count();

        let current_attached_energy_count_of_field_unit_index = game_field_unit_service_guard
            .get_current_attached_energy_of_field_unit_by_index(
                targeting_active_skill_request_form
                    .to_get_current_attached_energy_of_field_unit_by_index_request(
                        account_unique_id,
                        unit_card_index,
                        required_energy_race_to_use_skill)).await.get_current_attached_energy();

        if required_energy_count_to_use_skill > current_attached_energy_count_of_field_unit_index {
            println!("스킬 사용에 필요한 에너지를 충족하지 않아 스킬 사용이 불가합니다.");
            return TargetingActiveSkillResponseForm::new(false)
        }

        // 상대 고유값 찾기
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                targeting_active_skill_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 타게팅 데미지 적용
        // TODO: 현재에는 단일 타겟팅밖에 없으나 다중 타겟팅이 존재하는 경우 추가 처리 필요
        // TODO: 특수 에너지 효과 적용까지 추가 필요
        let target_card_index_string = targeting_active_skill_request_form.get_opponent_target_card_index();
        let target_card_index = target_card_index_string.parse::<i32>().unwrap();

        let target_skill_type = summary_active_skill_effect_response.get_skill_type();
        let target_skill_damage = summary_active_skill_effect_response.get_skill_damage();

        if target_skill_type == &ActiveSkillType::SingleTarget {
            let apply_damage_to_opponent_target_unit_response = game_field_unit_service_guard
                .apply_damage_to_target_unit_index(
                    targeting_active_skill_request_form
                        .to_apply_damage_to_target_unit_index(
                            opponent_unique_id,
                            target_card_index,
                            target_skill_damage)).await;

            if !apply_damage_to_opponent_target_unit_response.is_success() {
                println!("Targeting 스킬로 데미지를 입히는 데에 실패했습니다.");
                return TargetingActiveSkillResponseForm::new(false)
            }

            // TODO: 공격 당한 유닛 사망 판정 - 영찬

            // TODO: 사망 판정 값이 참이라면 무덤으로 보내기 - 영찬

            // TODO: 스킬 사용으로 인한 단일 타켓 데미지 알림 + 남은 체력 알림 + 사망 사실 알림 각각 따로따로 - 상근
        }

        // 12. 유닛의 이번 턴 Action 을 true 로 세팅
        game_field_unit_service_guard.execute_turn_action(
            targeting_active_skill_request_form
                .to_execute_turn_action_request(
                    account_unique_id,
                    unit_card_index)).await;

        TargetingActiveSkillResponseForm::new(true)
    }

    async fn request_non_targeting_active_skill(
        &self, non_targeting_active_skill_request_form: NonTargetingActiveSkillRequestForm) -> NonTargetingActiveSkillResponseForm {

        println!("GameCardActiveSkillControllerImpl: request_non_targeting_active_skill()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(non_targeting_active_skill_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return NonTargetingActiveSkillResponseForm::new(false)
        }

        // TODO: 프로토콜 검증 할 때가 아니라 패스

        // Action 가능한 턴인지 판별
        let unit_card_index_string = non_targeting_active_skill_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let check_turn_action_response =
            game_field_unit_service_guard.check_turn_action(
                non_targeting_active_skill_request_form
                    .to_check_turn_action_request(
                        account_unique_id,
                        unit_card_index)).await;

        if check_turn_action_response.has_already_taken_action() {
            println!("해당 유닛은 이미 액션을 취했습니다.");
            return NonTargetingActiveSkillResponseForm::new(false)
        }

        // Active Skill Summary 획득
        let usage_skill_index_string = non_targeting_active_skill_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();

        let mut game_card_active_skill_service_guard =
            self.game_card_active_skill_service.lock().await;

        let summary_active_skill_effect_response =
            game_card_active_skill_service_guard.summary_active_skill(
                non_targeting_active_skill_request_form
                    .to_summary_active_skill_effect_request(
                        unit_card_index,
                        usage_skill_index)).await;

        drop(game_card_active_skill_service_guard);

        // 스킬 사용에 필요한 에너지만큼 가지고 있는지 판정
        let required_energy_race_to_use_skill =
            *summary_active_skill_effect_response.get_required_energy().get_required_energy_race();
        let required_energy_count_to_use_skill =
            summary_active_skill_effect_response.get_required_energy().get_required_energy_count();

        let current_attached_energy_count_of_field_unit_index =
            game_field_unit_service_guard.get_current_attached_energy_of_field_unit_by_index(
                non_targeting_active_skill_request_form
                    .to_get_current_attached_energy_of_field_unit_by_index_request(
                        account_unique_id,
                        unit_card_index,
                        required_energy_race_to_use_skill)).await.get_current_attached_energy();

        if required_energy_count_to_use_skill > current_attached_energy_count_of_field_unit_index {
            println!("스킬 사용에 필요한 에너지를 충족하지 않아 스킬 사용이 불가합니다.");
            return NonTargetingActiveSkillResponseForm::new(false)
        }

        // 상대 고유값 찾기
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                non_targeting_active_skill_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 논타겟 데미지 효과 적용
        // TODO: 현재에는 전 유닛 데미지밖에 없으나 다중 랜덤 논타겟이 추가된다면 처리 필요함
        // TODO: 특수 에너지 효과 적용까지 추가 필요
        let non_target_skill_type = summary_active_skill_effect_response.get_skill_type();
        let non_target_skill_damage = summary_active_skill_effect_response.get_skill_damage();

        if non_target_skill_type == &ActiveSkillType::BroadArea {
            let apply_catastrophic_damage_to_opponent_field_unit_response =
                game_field_unit_service_guard.apply_catastrophic_damage_to_field_unit(
                    non_targeting_active_skill_request_form
                        .to_apply_catastrophic_damage_to_field_unit_request(
                            opponent_unique_id,
                            non_target_skill_damage)).await;

            if !apply_catastrophic_damage_to_opponent_field_unit_response.is_success() {
                println!("Non-Targeting 스킬로 데미지를 입히는 데에 실패했습니다.");
                return NonTargetingActiveSkillResponseForm::new(false)
            }

            // TODO: 공격 당한 유닛 사망 판정 - 영찬

            // TODO: 사망 판정 값이 참이라면 무덤으로 보내기 - 영찬

            // TODO: 스킬 사용으로 인한 광역 논타켓 데미지 알림 + 남은 체력 알림 + 사망 사실 알림 각각 따로따로
        }

        // 유닛의 이번 턴 Action 을 true 로 세팅
        game_field_unit_service_guard.execute_turn_action(
            non_targeting_active_skill_request_form
                .to_execute_turn_action_request(
                    account_unique_id,
                    unit_card_index)).await;

        NonTargetingActiveSkillResponseForm::new(true)
    }
}