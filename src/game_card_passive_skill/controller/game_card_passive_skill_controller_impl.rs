use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use diesel::IntoSql;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;

use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::game_card_passive_skill::controller::game_card_passive_skill_controller::GameCardPassiveSkillController;
use crate::game_card_passive_skill::controller::request_form::deploy_non_targeting_attack_passive_skill_request_form::DeployNonTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::deploy_targeting_attack_passive_skill_request_form::DeployTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::response_form::deploy_non_targeting_attack_passive_skill_response_form::DeployNonTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::deploy_targeting_attack_passive_skill_response_form::DeployTargetingAttackPassiveSkillResponseForm;

use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_card_passive_skill::service::game_card_passive_skill_service::GameCardPassiveSkillService;
use crate::game_card_passive_skill::service::game_card_passive_skill_service_impl::GameCardPassiveSkillServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service::GameFieldUnitActionPossibilityValidatorService;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service_impl::GameFieldUnitActionPossibilityValidatorServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardActiveSkillControllerImpl {
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_field_unit_action_possibility_validator_service: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>>,
}

impl GameCardActiveSkillControllerImpl {
    pub fn new(game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_field_unit_action_possibility_validator_service: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>>,) -> Self {

        GameCardActiveSkillControllerImpl {
            game_tomb_service,
            battle_room_service,
            game_field_unit_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_card_passive_skill_service,
            game_protocol_validation_service,
            game_field_unit_action_possibility_validator_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardActiveSkillControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardActiveSkillControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardActiveSkillControllerImpl::new(
                            GameTombServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameCardPassiveSkillServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            GameFieldUnitActionPossibilityValidatorServiceImpl::get_instance())));
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
impl GameCardPassiveSkillController for GameCardActiveSkillControllerImpl {
    async fn request_deploy_targeting_attack_passive_skill(
        &self, deploy_targeting_attack_passive_skill_request_form: DeployTargetingAttackPassiveSkillRequestForm) -> DeployTargetingAttackPassiveSkillResponseForm {

        println!("GameCardPassiveSkillControllerImpl: request_deploy_targeting_passive_skill()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(deploy_targeting_attack_passive_skill_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return DeployTargetingAttackPassiveSkillResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                deploy_targeting_attack_passive_skill_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return DeployTargetingAttackPassiveSkillResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        let unit_card_index_string = deploy_targeting_attack_passive_skill_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let unit_card_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                deploy_targeting_attack_passive_skill_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_found_opponent_unit_id();

        drop(game_field_unit_service_guard);

        // TODO: 프로토콜 검증 할 때가 아니라 패스

        // Passive Skill Summary 획득
        let usage_skill_index_string = deploy_targeting_attack_passive_skill_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_passive_skill_effect_by_index_response =
            game_card_passive_skill_service_guard.summary_passive_skill_by_index(
                deploy_targeting_attack_passive_skill_request_form
                    .to_summary_passive_skill_effect_by_index_request(
                        unit_card_id,
                        usage_skill_index)).await;

        drop(game_card_passive_skill_service_guard);

        // 스킬 사용 가능 여부 판정

        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_using_deploy_passive_skill_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_using_deploy_passive_skill_possible(
                deploy_targeting_attack_passive_skill_request_form
                    .to_is_using_deploy_passive_skill_possible_request(
                        account_unique_id,
                        unit_card_id,
                        usage_skill_index,
                        summary_passive_skill_effect_by_index_response.get_passive_skill_casting_condition().clone())).await;

        if !is_using_deploy_passive_skill_possible_response.is_possible() {
            return DeployTargetingAttackPassiveSkillResponseForm::new(false)
        }
        drop(game_field_unit_action_possibility_validator_service_guard);

        // 상대 고유값 찾기
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                deploy_targeting_attack_passive_skill_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 타게팅 데미지 적용
        // TODO: 현재에는 단일 타겟팅밖에 없으나 다중 타겟팅이 존재하는 경우 추가 처리 필요
        let target_card_index_string = deploy_targeting_attack_passive_skill_request_form.get_opponent_target_card_index();
        let target_card_index = target_card_index_string.parse::<i32>().unwrap();

        let target_skill_type = summary_passive_skill_effect_by_index_response.get_passive_skill_type();
        let target_skill_damage = summary_passive_skill_effect_by_index_response.get_skill_damage();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let extra_effect_list_of_unit_using_skill =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                deploy_targeting_attack_passive_skill_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        unit_card_index)).await.get_extra_status_effect_list().clone();

        if target_skill_type == &PassiveSkillType::SingleTarget {

            // extra effect 가 존재하는 경우 특수 효과가 가미된 공격 진행
            if !extra_effect_list_of_unit_using_skill.is_empty() {
                game_field_unit_service_guard.attack_target_unit_with_extra_effect(
                    deploy_targeting_attack_passive_skill_request_form
                        .to_attack_target_with_extra_effect_request(
                            opponent_unique_id,
                            target_skill_damage,
                            extra_effect_list_of_unit_using_skill, target_card_index)).await;
            }

            // 특수 효과가 없는 경우 일반 공격 진행
            game_field_unit_service_guard.apply_damage_to_target_unit_index(
                deploy_targeting_attack_passive_skill_request_form
                    .to_apply_damage_to_target_unit_index_request(
                        opponent_unique_id,
                        target_card_index,
                        target_skill_damage)).await;

            let maybe_dead_unit_id =
                game_field_unit_service_guard.judge_death_of_unit(
                    deploy_targeting_attack_passive_skill_request_form
                        .to_judge_death_of_unit_request(
                            opponent_unique_id,
                            target_card_index)).await.get_dead_unit_id();

            if maybe_dead_unit_id != -1 {
                let mut game_tomb_service_guard =
                    self.game_tomb_service.lock().await;

                game_tomb_service_guard.add_dead_unit_to_tomb(
                    deploy_targeting_attack_passive_skill_request_form
                        .to_add_dead_unit_to_tomb_request(
                            opponent_unique_id,
                            maybe_dead_unit_id)).await;
            }

            // TODO: 스킬 사용으로 인한 단일 타켓 데미지 알림 + 남은 체력 알림 + 사망 사실 알림 각각 따로따로
        }

        // 12. 유닛의 해당 패시브 스킬을 false로 세팅
        game_field_unit_service_guard.execute_index_passive_of_unit(
            deploy_targeting_attack_passive_skill_request_form
                .to_execute_index_passive_of_unit_request(
                    account_unique_id,
                    unit_card_index,
                    usage_skill_index)).await;

        drop(game_field_unit_service_guard);

        // 13. 해당 유닛의 소환시 발동되는 패시브 스킬이 더 있는지 알려줘야함 .

        DeployTargetingAttackPassiveSkillResponseForm::new(true)
    }

    async fn request_deploy_non_targeting_attack_passive_skill(
        &self, non_targeting_passive_skill_request_form: DeployNonTargetingAttackPassiveSkillRequestForm) -> DeployNonTargetingAttackPassiveSkillResponseForm {

        println!("GameCardActiveSkillControllerImpl: request_deploy_non_targeting_attack_passive_skill()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(non_targeting_passive_skill_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return DeployNonTargetingAttackPassiveSkillResponseForm::new(false)
        }

        // Action 가능한 턴인지 판별
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                non_targeting_passive_skill_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return DeployNonTargetingAttackPassiveSkillResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 프로토콜 검증 할 때가 아니라 패스

        // Active Skill Summary 획득
        let unit_card_index_string = non_targeting_passive_skill_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let usage_skill_index_string = non_targeting_passive_skill_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let unit_card_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                non_targeting_passive_skill_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_found_opponent_unit_id();

        drop(game_field_unit_service_guard);

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_passive_skill_effect_by_index_response =
            game_card_passive_skill_service_guard.summary_passive_skill_by_index(
                non_targeting_passive_skill_request_form
                    .to_summary_passive_skill_effect_by_index_request(
                        unit_card_id,
                        usage_skill_index)).await;

        drop(game_card_passive_skill_service_guard);
        //
        // 스킬 사용 가능 여부 판정
        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_using_deploy_passive_skill_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_using_deploy_passive_skill_possible(
                non_targeting_passive_skill_request_form
                    .to_is_using_deploy_passive_skill_possible_request(
                        account_unique_id,
                        unit_card_id,
                        usage_skill_index,
                        summary_passive_skill_effect_by_index_response.get_passive_skill_casting_condition().clone())).await;

        if !is_using_deploy_passive_skill_possible_response.is_possible() {
            return DeployNonTargetingAttackPassiveSkillResponseForm::new(false)
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 상대 고유값 찾기
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                non_targeting_passive_skill_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 논타겟 데미지 효과 적용
        // TODO: 현재에는 전 유닛 데미지밖에 없으나 단일/다중 랜덤 논타겟이 추가된다면 처리 필요함
        let non_target_skill_type = summary_passive_skill_effect_by_index_response.get_passive_skill_type();
        let non_target_skill_damage = summary_passive_skill_effect_by_index_response.get_skill_damage();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let extra_effect_list_of_unit_using_skill =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                non_targeting_passive_skill_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        unit_card_index)).await.get_extra_status_effect_list().clone();

        if non_target_skill_type == &PassiveSkillType::BroadArea {

            if !extra_effect_list_of_unit_using_skill.is_empty() {
                game_field_unit_service_guard.attack_every_unit_with_extra_effect(
                    non_targeting_passive_skill_request_form
                        .to_attack_every_unit_with_extra_effect_request(
                            opponent_unique_id,
                            non_target_skill_damage,
                            extra_effect_list_of_unit_using_skill)).await;
            }

            game_field_unit_service_guard.apply_catastrophic_damage_to_field_unit(
                non_targeting_passive_skill_request_form
                    .to_apply_catastrophic_damage_to_field_unit_request(
                        opponent_unique_id,
                        non_target_skill_damage)).await;

            let dead_unit_list =
                game_field_unit_service_guard.judge_death_of_every_field_unit(
                    non_targeting_passive_skill_request_form
                        .to_judge_death_of_every_unit_request(
                            opponent_unique_id)).await.get_dead_unit_id_list().clone();

            if !dead_unit_list.is_empty() {
                let mut game_tomb_service_guard =
                    self.game_tomb_service.lock().await;

                game_tomb_service_guard.add_dead_unit_list_to_tomb(
                    non_targeting_passive_skill_request_form
                        .to_add_dead_unit_list_to_tomb_request(
                            opponent_unique_id,
                            dead_unit_list)).await;
            }

            // TODO: 스킬 사용으로 인한 광역 논타켓 데미지 알림 + 남은 체력 알림 + 사망 사실 알림 각각 따로따로
        }

        // 유닛의 해당 패시브 스킬을 false로 세팅
        game_field_unit_service_guard.execute_index_passive_of_unit(
            non_targeting_passive_skill_request_form
                .to_execute_index_passive_of_unit_request(
                    account_unique_id,
                    unit_card_index,
                    usage_skill_index)).await;

        drop(game_field_unit_service_guard);

        // 13. 해당 유닛의 소환시 발동되는 패시브 스킬이 더 있는지 알려줘야함 .

        DeployNonTargetingAttackPassiveSkillResponseForm::new(true)
    }
}