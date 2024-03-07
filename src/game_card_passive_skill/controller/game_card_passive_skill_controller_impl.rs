use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use diesel::IntoSql;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;

use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::game_card_active_skill::controller::response_form::non_targeting_active_skill_response_form::NonTargetingActiveSkillResponseForm;
use crate::game_card_active_skill::controller::response_form::targeting_active_skill_response_form::TargetingActiveSkillResponseForm;
use crate::game_card_passive_skill::controller::game_card_passive_skill_controller::GameCardPassiveSkillController;
use crate::game_card_passive_skill::controller::request_form::deploy_non_targeting_attack_passive_skill_request_form::DeployNonTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::deploy_targeting_attack_passive_skill_request_form::DeployTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::deploy_targeting_attack_to_game_main_character_request_form::DeployTargetingAttackToGameMainCharacterRequestForm;
use crate::game_card_passive_skill::controller::request_form::turn_start_non_targeting_attack_passive_skill_request_form::TurnStartNonTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::turn_start_targeting_attack_passive_skill_request_form::TurnStartTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::turn_start_targeting_attack_to_game_main_character_request_form::TurnStartTargetingAttackToGameMainCharacterRequestForm;
use crate::game_card_passive_skill::controller::response_form::deploy_non_targeting_attack_passive_skill_response_form::DeployNonTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::deploy_targeting_attack_passive_skill_response_form::DeployTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::deploy_targeting_attack_to_game_main_character_response_form::DeployTargetingAttackToGameMainCharacterResponseForm;
use crate::game_card_passive_skill::controller::response_form::turn_start_non_targeting_attack_passive_skill_response_form::TurnStartNonTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::turn_start_targeting_attack_passive_skill_response_form::TurnStartTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::turn_start_targeting_attack_to_game_main_character_response_form::TurnStartTargetingAttackToGameMainCharacterResponseForm;

use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_card_passive_skill::service::game_card_passive_skill_service::GameCardPassiveSkillService;
use crate::game_card_passive_skill::service::game_card_passive_skill_service_impl::GameCardPassiveSkillServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service::GameFieldUnitActionPossibilityValidatorService;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service_impl::GameFieldUnitActionPossibilityValidatorServiceImpl;
use crate::game_main_character::entity::game_main_character::GameMainCharacter;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_main_character::service::game_main_character_service_impl::GameMainCharacterServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_winner_check::service::game_winner_check_service::GameWinnerCheckService;
use crate::game_winner_check::service::game_winner_check_service_impl::GameWinnerCheckServiceImpl;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::notify_player_action_info_service_impl::NotifyPlayerActionInfoServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

pub struct GameCardPassiveSkillControllerImpl {
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_field_unit_action_possibility_validator_service: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>>,
    game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
    game_winner_check_service: Arc<AsyncMutex<GameWinnerCheckServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
    notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
}

impl GameCardPassiveSkillControllerImpl {
    pub fn new(game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_field_unit_action_possibility_validator_service: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>>,
               game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
               game_winner_check_service: Arc<AsyncMutex<GameWinnerCheckServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
               notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,) -> Self {

        GameCardPassiveSkillControllerImpl {
            game_tomb_service,
            battle_room_service,
            game_field_unit_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_card_passive_skill_service,
            game_protocol_validation_service,
            game_field_unit_action_possibility_validator_service,
            game_main_character_service,
            game_winner_check_service,
            ui_data_generator_service,
            notify_player_action_info_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardPassiveSkillControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardPassiveSkillControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardPassiveSkillControllerImpl::new(
                            GameTombServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameCardPassiveSkillServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            GameFieldUnitActionPossibilityValidatorServiceImpl::get_instance(),
                            GameMainCharacterServiceImpl::get_instance(),
                            GameWinnerCheckServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance(),
                            NotifyPlayerActionInfoServiceImpl::get_instance())));
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
impl GameCardPassiveSkillController for GameCardPassiveSkillControllerImpl {
    async fn request_deploy_targeting_attack_passive_skill(
        &self, deploy_targeting_attack_passive_skill_request_form: DeployTargetingAttackPassiveSkillRequestForm) -> DeployTargetingAttackPassiveSkillResponseForm {

        println!("GameCardPassiveSkillControllerImpl: request_deploy_targeting_passive_skill()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(deploy_targeting_attack_passive_skill_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return DeployTargetingAttackPassiveSkillResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                deploy_targeting_attack_passive_skill_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return DeployTargetingAttackPassiveSkillResponseForm::default()
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
                        unit_card_index,
                        usage_skill_index,
                        summary_passive_skill_effect_by_index_response.get_passive_skill_casting_condition().clone())).await;

        if !is_using_deploy_passive_skill_possible_response.is_possible() {
            return DeployTargetingAttackPassiveSkillResponseForm::default()
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
        let opponent_target_unit_card_index = target_card_index_string.parse::<i32>().unwrap();

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
                            extra_effect_list_of_unit_using_skill, opponent_target_unit_card_index)).await;
            }

            // 특수 효과가 없는 경우 일반 공격 진행
            game_field_unit_service_guard.apply_damage_to_target_unit_index(
                deploy_targeting_attack_passive_skill_request_form
                    .to_apply_damage_to_target_unit_index_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index,
                        target_skill_damage)).await;
        }
        else { return DeployTargetingAttackPassiveSkillResponseForm::default() }

        let opponent_target_unit_health_point =
            game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                deploy_targeting_attack_passive_skill_request_form
                    .to_get_current_health_point_of_field_unit_by_index_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_current_unit_health_point();

        let opponent_target_unit_harmful_effect_list =
            game_field_unit_service_guard.acquire_unit_harmful_status_effect(
                deploy_targeting_attack_passive_skill_request_form
                    .to_acquire_unit_harmful_status_effect_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_harmful_effect_list().clone();

        // 피격 유닛이 죽었는지 판정
        let judge_death_of_opponent_unit_response =
            game_field_unit_service_guard.judge_death_of_unit(
                deploy_targeting_attack_passive_skill_request_form
                    .to_judge_death_of_unit_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await;

        // 죽은 경우 묘지에 추가
        let mut game_tomb_service_guard =
            self.game_tomb_service.lock().await;

        if judge_death_of_opponent_unit_response.get_dead_unit_id() != -1 {
            println!("공격 당한 유닛이 사망했으므로 묘지로 이동합니다.");

            game_tomb_service_guard.add_used_card_to_tomb(
                deploy_targeting_attack_passive_skill_request_form
                    .to_place_dead_unit_to_tomb_request(
                        opponent_unique_id,
                        judge_death_of_opponent_unit_response.get_dead_unit_id())).await;
        }

        drop(game_tomb_service_guard);
        // 12. 유닛의 해당 패시브 스킬을 false로 세팅
        game_field_unit_service_guard.execute_index_passive_of_unit(
            deploy_targeting_attack_passive_skill_request_form
                .to_execute_index_passive_of_unit_request(
                    account_unique_id,
                    unit_card_index,
                    usage_skill_index)).await;

        // 13. 해당 유닛의 소환시 발동되는 패시브 스킬이 더 있는지 알려줘야함 .

        let passive_usable_list =
            game_field_unit_service_guard.get_passive_skill_usable(
                deploy_targeting_attack_passive_skill_request_form
                    .to_get_passive_skill_usable_list_request(
                        account_unique_id,
                        unit_card_index)).await.get_passive_skill_usable_list();

        drop(game_field_unit_service_guard);

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_deploy_passive_skill_effect_list =
            game_card_passive_skill_service_guard.summary_deploy_passive_skill(
                deploy_targeting_attack_passive_skill_request_form
                    .to_summary_deploy_passive_skill_effect_request(
                        unit_card_id)).await.get_passive_skill_effect_list();

        let mut passive_type_list = Vec::new();

        for i in 0..3 {
            if passive_usable_list[i] == true && summary_deploy_passive_skill_effect_list[i].get_passive_skill_type() != &PassiveSkillType::Dummy {
                passive_type_list.push(summary_deploy_passive_skill_effect_list[i].get_passive_skill_type());
            }
            else {
                passive_type_list.push(&PassiveSkillType::Dummy);
            }
        }
        drop(game_card_passive_skill_service_guard);

        // TODO: 스킬 사용으로 인한 단일 타켓 데미지 알림 + 남은 체력 알림 + 사망 사실 알림 각각 따로따로
        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_opponent_specific_unit_health_point_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_health_point_data(
                deploy_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_specific_unit_health_point_data_request(
                        opponent_target_unit_card_index,
                        opponent_target_unit_health_point)).await;

        let generate_opponent_specific_unit_harmful_effect_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_harmful_effect_data(
                deploy_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_specific_unit_harmful_effect_data_request(
                        opponent_target_unit_card_index,
                        opponent_target_unit_harmful_effect_list)).await;

        let generate_opponent_specific_unit_death_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_death_data(
                deploy_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_specific_unit_death_data_request(
                        judge_death_of_opponent_unit_response.get_dead_unit_index())).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_deploy_targeting_attack_passive_skill_to_unit(
            deploy_targeting_attack_passive_skill_request_form
                .to_notice_deploy_targeting_attack_passive_skill_to_unit_request(
                    opponent_unique_id,
                    generate_opponent_specific_unit_health_point_data_response.get_player_field_unit_health_point_map_for_notice().clone(),
                    generate_opponent_specific_unit_harmful_effect_data_response.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                    generate_opponent_specific_unit_death_data_response.get_player_field_unit_death_map_for_notice().clone())).await;

        drop(notify_player_action_info_service_guard);

        // TODO: 패시브 스킬 정보 responseform 에 넣어야함
        DeployTargetingAttackPassiveSkillResponseForm::new(
            true,
            generate_opponent_specific_unit_health_point_data_response.get_player_field_unit_health_point_map_for_response().clone(),
            generate_opponent_specific_unit_harmful_effect_data_response.get_player_field_unit_harmful_effect_map_for_response().clone(),
            generate_opponent_specific_unit_death_data_response.get_player_field_unit_death_map_for_response().clone())

    }

    async fn request_deploy_non_targeting_attack_passive_skill(
        &self, deploy_non_targeting_attack_passive_skill_request_form: DeployNonTargetingAttackPassiveSkillRequestForm) -> DeployNonTargetingAttackPassiveSkillResponseForm {

        println!("GameCardActiveSkillControllerImpl: request_deploy_non_targeting_attack_passive_skill()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(deploy_non_targeting_attack_passive_skill_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return DeployNonTargetingAttackPassiveSkillResponseForm::default()
        }

        // Action 가능한 턴인지 판별
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return DeployNonTargetingAttackPassiveSkillResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 프로토콜 검증 할 때가 아니라 패스

        // Passive Skill Summary 획득
        let unit_card_index_string = deploy_non_targeting_attack_passive_skill_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let usage_skill_index_string = deploy_non_targeting_attack_passive_skill_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let unit_card_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_found_opponent_unit_id();

        drop(game_field_unit_service_guard);

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_passive_skill_effect_by_index_response =
            game_card_passive_skill_service_guard.summary_passive_skill_by_index(
                deploy_non_targeting_attack_passive_skill_request_form
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
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_is_using_deploy_passive_skill_possible_request(
                        account_unique_id,
                        unit_card_index,
                        usage_skill_index,
                        summary_passive_skill_effect_by_index_response.get_passive_skill_casting_condition().clone())).await;

        if !is_using_deploy_passive_skill_possible_response.is_possible() {
            return DeployNonTargetingAttackPassiveSkillResponseForm::default()
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 상대 고유값 찾기
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                deploy_non_targeting_attack_passive_skill_request_form
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
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        unit_card_index)).await.get_extra_status_effect_list().clone();

        if non_target_skill_type == &PassiveSkillType::BroadArea {

            if !extra_effect_list_of_unit_using_skill.is_empty() {
                game_field_unit_service_guard.attack_every_unit_with_extra_effect(
                    deploy_non_targeting_attack_passive_skill_request_form
                        .to_attack_every_unit_with_extra_effect_request(
                            opponent_unique_id,
                            non_target_skill_damage,
                            extra_effect_list_of_unit_using_skill)).await;
            } else {
                game_field_unit_service_guard.apply_catastrophic_damage_to_field_unit(
                    deploy_non_targeting_attack_passive_skill_request_form
                        .to_apply_catastrophic_damage_to_field_unit_request(
                            opponent_unique_id,
                            non_target_skill_damage)).await;
            }
        } else {
            println!("현재 구현되지 않은 논타겟 스킬 기능입니다.");
            return DeployNonTargetingAttackPassiveSkillResponseForm::default()
        }


        // TODO: 사망 판정 전에 데이터를 가져와야 전송 가능하여 위치 변경 (데미지 - 데이터 - 사망판정 순서)
        let opponent_all_unit_health_point =
            game_field_unit_service_guard.get_current_health_point_of_all_field_unit(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_get_current_health_point_of_all_field_unit_request(
                        opponent_unique_id)).await.get_current_unit_health_point().clone();

        let opponent_all_unit_harmful_effect =
            game_field_unit_service_guard.acquire_harmful_status_effect_of_all_unit(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_acquire_harmful_status_effect_of_all_unit_request(
                        opponent_unique_id)).await.get_harmful_effect_list_of_all_unit();

        // 유닛 사망 처리
        let judge_death_of_every_unit_response =
            game_field_unit_service_guard.judge_death_of_every_field_unit(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_judge_death_of_every_unit_request(
                        opponent_unique_id)).await;

        if !judge_death_of_every_unit_response.get_dead_unit_id_list().is_empty() {
            let mut game_tomb_service_guard =
                self.game_tomb_service.lock().await;

            game_tomb_service_guard.add_dead_unit_list_to_tomb(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_add_dead_unit_list_to_tomb_request(
                        opponent_unique_id,
                        judge_death_of_every_unit_response.get_dead_unit_id_list())).await;

            drop(game_tomb_service_guard);
        }

        // 유닛의 해당 패시브 스킬을 false로 세팅
        game_field_unit_service_guard.execute_index_passive_of_unit(
            deploy_non_targeting_attack_passive_skill_request_form
                .to_execute_index_passive_of_unit_request(
                    account_unique_id,
                    unit_card_index,
                    usage_skill_index)).await;

        // 13. 해당 유닛의 소환시 발동되는 패시브 스킬이 더 있는지 알려줘야함 .
        let passive_usable_list =
            game_field_unit_service_guard.get_passive_skill_usable(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_get_passive_skill_usable_list_request(
                        account_unique_id,
                        unit_card_index)).await.get_passive_skill_usable_list();

        drop(game_field_unit_service_guard);

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_deploy_passive_skill_effect_list =
            game_card_passive_skill_service_guard.summary_deploy_passive_skill(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_summary_deploy_passive_skill_effect_request(
                        unit_card_id)).await.get_passive_skill_effect_list();

        let mut passive_type_list = Vec::new();

        for i in 0..3 {
            if passive_usable_list[i] == true && summary_deploy_passive_skill_effect_list[i].get_passive_skill_type() != &PassiveSkillType::Dummy {
                passive_type_list.push(summary_deploy_passive_skill_effect_list[i].get_passive_skill_type());
            }
            else {
                passive_type_list.push(&PassiveSkillType::Dummy);

            }
        }
        drop(game_card_passive_skill_service_guard);
        // TODO: 스킬 사용으로 인한 광역 논타켓 데미지 알림 + 남은 체력 알림 + 사망 사실 알림 각각 따로따로
        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_opponent_multiple_unit_health_point_data_response =
            ui_data_generator_service_guard.generate_opponent_multiple_unit_health_point_data(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_multiple_unit_health_point_data_request(opponent_all_unit_health_point)).await;

        let generate_opponent_multiple_unit_harmful_effect_data_response =
            ui_data_generator_service_guard.generate_opponent_multiple_unit_harmful_effect_data(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_multiple_unit_harmful_effect_data_request(opponent_all_unit_harmful_effect)).await;

        let generate_opponent_multiple_unit_death_data_response =
            ui_data_generator_service_guard.generate_opponent_multiple_unit_death_data(
                deploy_non_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_multiple_unit_death_data_request(
                        judge_death_of_every_unit_response.get_dead_unit_index_list())).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_deploy_non_targeting_attack_passive_skill(
            deploy_non_targeting_attack_passive_skill_request_form
                .to_notice_deploy_non_targeting_attack_passive_skill_request(
                    opponent_unique_id,
                    generate_opponent_multiple_unit_health_point_data_response
                        .get_player_field_unit_health_point_map_for_notice().clone(),
                    generate_opponent_multiple_unit_harmful_effect_data_response
                        .get_player_field_unit_harmful_effect_map_for_notice().clone(),
                    generate_opponent_multiple_unit_death_data_response
                        .get_player_field_unit_death_map_for_notice().clone())).await;

        drop(notify_player_action_info_service_guard);

        // TODO: 패시브 스킬 정보 responseform 에 넣어야함
        DeployNonTargetingAttackPassiveSkillResponseForm::new(
            true,
            generate_opponent_multiple_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_response().clone(),
            generate_opponent_multiple_unit_harmful_effect_data_response
                .get_player_field_unit_harmful_effect_map_for_response().clone(),
            generate_opponent_multiple_unit_death_data_response
                .get_player_field_unit_death_map_for_response().clone())
    }

    async fn request_deploy_targeting_attack_to_game_main_character(
        &self, deploy_targeting_attack_to_game_main_character_request_form: DeployTargetingAttackToGameMainCharacterRequestForm)
        -> DeployTargetingAttackToGameMainCharacterResponseForm {

        println!("GameCardUnitControllerImpl: request_to_attack_game_main_character()");

        // 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(
            deploy_targeting_attack_to_game_main_character_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return DeployTargetingAttackToGameMainCharacterResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return DeployTargetingAttackToGameMainCharacterResponseForm::default()
        }

        // Battle Field 에서 공격하는 유닛의 index 를 토대로 id 값 확보
        let unit_card_index_string = deploy_targeting_attack_to_game_main_character_request_form.get_attacker_unit_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let usage_skill_index_string = deploy_targeting_attack_to_game_main_character_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();


        // 액션 가능한 턴인지 검증
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let unit_card_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_found_opponent_unit_id();

        if unit_card_id == -1 {
            println!("필드에 존재하지 않는 유닛을 지정하여 보냈으므로 해킹범입니다!");
            return DeployTargetingAttackToGameMainCharacterResponseForm::default()
        }

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_passive_skill_effect_by_index_response =
            game_card_passive_skill_service_guard.summary_passive_skill_by_index(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_summary_passive_skill_effect_by_index_request(
                        unit_card_id,
                        usage_skill_index)).await;

        if summary_passive_skill_effect_by_index_response.get_passive_skill_type() != &PassiveSkillType::SingleTarget {
            println!("단일 공격 패시브가 아닙니다.");
            return DeployTargetingAttackToGameMainCharacterResponseForm::default()
        }

        drop(game_card_passive_skill_service_guard);

        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_using_deploy_passive_skill_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_using_deploy_passive_skill_possible(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_is_using_deploy_passive_skill_possible_request(
                        account_unique_id,
                        unit_card_index,
                        usage_skill_index,
                        summary_passive_skill_effect_by_index_response.get_passive_skill_casting_condition().clone())).await;

        if !is_using_deploy_passive_skill_possible_response.is_possible() {
            return DeployTargetingAttackToGameMainCharacterResponseForm::default()
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 패시브 정보 에서 공격력 정보 확보
        let attacker_unit_attack_point =
            summary_passive_skill_effect_by_index_response.get_skill_damage();

        // 공격을 위해 상대방 고유값 획득
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut game_main_character_service_guard =
            self.game_main_character_service.lock().await;

        game_main_character_service_guard.apply_damage_to_main_character(
            deploy_targeting_attack_to_game_main_character_request_form
                .to_apply_damage_to_main_character_request(
                    opponent_unique_id,
                    attacker_unit_attack_point)).await;

        // 액션 완료 설정
        game_field_unit_service_guard.execute_index_passive_of_unit(
            deploy_targeting_attack_to_game_main_character_request_form
                .to_execute_index_passive_of_unit_request(
                    account_unique_id,
                    unit_card_index,
                    usage_skill_index)).await;

        drop(game_field_unit_service_guard);

        let check_main_character_of_account_unique_id_response =
            game_main_character_service_guard.check_main_character_of_account_unique_id(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_check_main_character_of_account_unique_id_request(opponent_unique_id)).await;

        // 사망하면 상대 패배 결정
        if check_main_character_of_account_unique_id_response.get_status_main_character() == &StatusMainCharacterEnum::Death {
            let mut game_winner_check_service_guard =
                self.game_winner_check_service.lock().await;

            game_winner_check_service_guard.check_health_of_main_character_for_setting_game_winner(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_check_main_character_for_setting_game_winner_request(
                        account_unique_id,
                        opponent_unique_id)).await;

            drop(game_winner_check_service_guard);
        }

        drop(game_main_character_service_guard);

        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_opponent_main_character_health_point_data_response =
            ui_data_generator_service_guard.generate_opponent_main_character_health_point_data(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_generate_opponent_main_character_health_point_data_request(
                        check_main_character_of_account_unique_id_response.get_current_health_point())).await;

        let generate_opponent_main_character_survival_data_response =
            ui_data_generator_service_guard.generate_opponent_main_character_survival_data(
                deploy_targeting_attack_to_game_main_character_request_form
                    .to_generate_opponent_main_character_survival_data_request(
                        check_main_character_of_account_unique_id_response.get_status_main_character().clone())).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_basic_attack_to_main_character(
            deploy_targeting_attack_to_game_main_character_request_form
                .to_notice_basic_attack_to_main_character_request(
                    opponent_unique_id,
                    generate_opponent_main_character_health_point_data_response
                        .get_player_main_character_health_point_map_for_notice().clone(),
                    generate_opponent_main_character_survival_data_response
                        .get_player_main_character_survival_map_for_notice().clone())).await;

        drop(notify_player_action_info_service_guard);

        DeployTargetingAttackToGameMainCharacterResponseForm::from_response(
            generate_opponent_main_character_health_point_data_response,
            generate_opponent_main_character_survival_data_response)
    }

    async fn request_turn_start_targeting_attack_passive_skill(
        &self, turn_start_targeting_attack_passive_skill_request_form: TurnStartTargetingAttackPassiveSkillRequestForm) -> TurnStartTargetingAttackPassiveSkillResponseForm {
        println!("GameCardPassiveSkillControllerImpl: request_deploy_targeting_passive_skill()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(turn_start_targeting_attack_passive_skill_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return TurnStartTargetingAttackPassiveSkillResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return TurnStartTargetingAttackPassiveSkillResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        let unit_card_index_string = turn_start_targeting_attack_passive_skill_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let unit_card_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_found_opponent_unit_id();

        drop(game_field_unit_service_guard);

        // TODO: 프로토콜 검증 할 때가 아니라 패스

        // Passive Skill Summary 획득
        let usage_skill_index_string = turn_start_targeting_attack_passive_skill_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_passive_skill_effect_by_index_response =
            game_card_passive_skill_service_guard.summary_passive_skill_by_index(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_summary_passive_skill_effect_by_index_request(
                        unit_card_id,
                        usage_skill_index)).await;

        drop(game_card_passive_skill_service_guard);

        // 스킬 사용 가능 여부 판정

        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_using_turn_start_passive_skill_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_using_turn_start_passive_skill_possible(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_is_using_turn_start_passive_skill_possible_request(
                        account_unique_id,
                        unit_card_index,
                        usage_skill_index,
                        summary_passive_skill_effect_by_index_response.get_passive_skill_casting_condition().clone())).await;

        if !is_using_turn_start_passive_skill_possible_response.is_possible() {
            return TurnStartTargetingAttackPassiveSkillResponseForm::default()
        }
        drop(game_field_unit_action_possibility_validator_service_guard);

        // 상대 고유값 찾기
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 타게팅 데미지 적용
        // TODO: 현재에는 단일 타겟팅밖에 없으나 다중 타겟팅이 존재하는 경우 추가 처리 필요
        let target_card_index_string = turn_start_targeting_attack_passive_skill_request_form.get_opponent_target_card_index();
        let opponent_target_unit_card_index = target_card_index_string.parse::<i32>().unwrap();

        let target_skill_type = summary_passive_skill_effect_by_index_response.get_passive_skill_type();
        let target_skill_damage = summary_passive_skill_effect_by_index_response.get_skill_damage();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let extra_effect_list_of_unit_using_skill =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        unit_card_index)).await.get_extra_status_effect_list().clone();

        if target_skill_type == &PassiveSkillType::SingleTarget {

            // extra effect 가 존재하는 경우 특수 효과가 가미된 공격 진행
            if !extra_effect_list_of_unit_using_skill.is_empty() {
                game_field_unit_service_guard.attack_target_unit_with_extra_effect(
                    turn_start_targeting_attack_passive_skill_request_form
                        .to_attack_target_with_extra_effect_request(
                            opponent_unique_id,
                            target_skill_damage,
                            extra_effect_list_of_unit_using_skill, opponent_target_unit_card_index)).await;
            }

            // 특수 효과가 없는 경우 일반 공격 진행
            game_field_unit_service_guard.apply_damage_to_target_unit_index(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_apply_damage_to_target_unit_index_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index,
                        target_skill_damage)).await;
        }
        else { return TurnStartTargetingAttackPassiveSkillResponseForm::default() }

        let opponent_target_unit_health_point =
            game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_get_current_health_point_of_field_unit_by_index_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_current_unit_health_point();

        let opponent_target_unit_harmful_effect_list =
            game_field_unit_service_guard.acquire_unit_harmful_status_effect(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_acquire_unit_harmful_status_effect_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_harmful_effect_list().clone();

        // 피격 유닛이 죽었는지 판정
        let judge_death_of_opponent_unit_response =
            game_field_unit_service_guard.judge_death_of_unit(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_judge_death_of_unit_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await;

        // 죽은 경우 묘지에 추가
        let mut game_tomb_service_guard =
            self.game_tomb_service.lock().await;

        if judge_death_of_opponent_unit_response.get_dead_unit_id() != -1 {
            println!("공격 당한 유닛이 사망했으므로 묘지로 이동합니다.");

            game_tomb_service_guard.add_used_card_to_tomb(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_place_dead_unit_to_tomb_request(
                        opponent_unique_id,
                        judge_death_of_opponent_unit_response.get_dead_unit_id())).await;
        }

        // 12. 유닛의 해당 패시브 스킬을 false로 세팅
        game_field_unit_service_guard.execute_index_passive_of_unit(
            turn_start_targeting_attack_passive_skill_request_form
                .to_execute_index_passive_of_unit_request(
                    account_unique_id,
                    unit_card_index,
                    usage_skill_index)).await;


        // 13. 해당 유닛의 소환시 발동되는 패시브 스킬이 더 있는지 알려줘야함 .
        let passive_usable_list =
            game_field_unit_service_guard.get_passive_skill_usable(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_get_passive_skill_usable_list_request(
                        account_unique_id,
                        unit_card_index)).await.get_passive_skill_usable_list();

        drop(game_field_unit_service_guard);

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_turn_start_passive_skill_effect_list =
            game_card_passive_skill_service_guard.summary_turn_start_passive_skill(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_summary_turn_start_passive_skill_effect_request(
                        unit_card_id)).await.get_passive_skill_effect_list();

        let mut passive_type_list = Vec::new();

        for i in 0..3 {
            if passive_usable_list[i] == true && summary_turn_start_passive_skill_effect_list[i].get_passive_skill_type() != &PassiveSkillType::Dummy {
                passive_type_list.push(summary_turn_start_passive_skill_effect_list[i].get_passive_skill_type());
            }
            else {
                passive_type_list.push(&PassiveSkillType::Dummy);

            }
        }
        drop(game_card_passive_skill_service_guard);

        // TODO: 스킬 사용으로 인한 단일 타켓 데미지 알림 + 남은 체력 알림 + 사망 사실 알림 각각 따로따로
        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_opponent_specific_unit_health_point_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_health_point_data(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_specific_unit_health_point_data_request(
                        opponent_target_unit_card_index,
                        opponent_target_unit_health_point)).await;

        let generate_opponent_specific_unit_harmful_effect_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_harmful_effect_data(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_specific_unit_harmful_effect_data_request(
                        opponent_target_unit_card_index,
                        opponent_target_unit_harmful_effect_list)).await;

        let generate_opponent_specific_unit_death_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_death_data(
                turn_start_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_specific_unit_death_data_request(
                        judge_death_of_opponent_unit_response.get_dead_unit_index())).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_turn_start_targeting_attack_passive_skill_to_unit(
            turn_start_targeting_attack_passive_skill_request_form
                .to_notice_turn_start_targeting_attack_passive_skill_to_unit_request(
                    opponent_unique_id,
                    generate_opponent_specific_unit_health_point_data_response.get_player_field_unit_health_point_map_for_notice().clone(),
                    generate_opponent_specific_unit_harmful_effect_data_response.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                    generate_opponent_specific_unit_death_data_response.get_player_field_unit_death_map_for_notice().clone())).await;

        drop(notify_player_action_info_service_guard);

        TurnStartTargetingAttackPassiveSkillResponseForm::new(
            true,
            generate_opponent_specific_unit_health_point_data_response.get_player_field_unit_health_point_map_for_response().clone(),
            generate_opponent_specific_unit_harmful_effect_data_response.get_player_field_unit_harmful_effect_map_for_response().clone(),
            generate_opponent_specific_unit_death_data_response.get_player_field_unit_death_map_for_response().clone())
    }

    async fn request_turn_start_non_targeting_attack_passive_skill(
        &self, turn_start_non_targeting_attack_passive_skill_request_form: TurnStartNonTargetingAttackPassiveSkillRequestForm) -> TurnStartNonTargetingAttackPassiveSkillResponseForm {

        println!("GameCardActiveSkillControllerImpl: request_deploy_non_targeting_attack_passive_skill()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(turn_start_non_targeting_attack_passive_skill_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return TurnStartNonTargetingAttackPassiveSkillResponseForm::default()
        }

        // Action 가능한 턴인지 판별
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return TurnStartNonTargetingAttackPassiveSkillResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 프로토콜 검증 할 때가 아니라 패스

        // Passive Skill Summary 획득
        let unit_card_index_string = turn_start_non_targeting_attack_passive_skill_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let usage_skill_index_string = turn_start_non_targeting_attack_passive_skill_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let unit_card_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_found_opponent_unit_id();

        drop(game_field_unit_service_guard);

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_passive_skill_effect_by_index_response =
            game_card_passive_skill_service_guard.summary_passive_skill_by_index(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_summary_passive_skill_effect_by_index_request(
                        unit_card_id,
                        usage_skill_index)).await;

        drop(game_card_passive_skill_service_guard);

        // 스킬 사용 가능 여부 판정
        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_using_turn_start_passive_skill_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_using_turn_start_passive_skill_possible(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_is_using_turn_start_passive_skill_possible_request(
                        account_unique_id,
                        unit_card_index,
                        usage_skill_index,
                        summary_passive_skill_effect_by_index_response.get_passive_skill_casting_condition().clone())).await;

        if !is_using_turn_start_passive_skill_possible_response.is_possible() {
            return TurnStartNonTargetingAttackPassiveSkillResponseForm::default()
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 상대 고유값 찾기
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                turn_start_non_targeting_attack_passive_skill_request_form
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
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        unit_card_index)).await.get_extra_status_effect_list().clone();

        if non_target_skill_type == &PassiveSkillType::BroadArea {

            if !extra_effect_list_of_unit_using_skill.is_empty() {
                game_field_unit_service_guard.attack_every_unit_with_extra_effect(
                    turn_start_non_targeting_attack_passive_skill_request_form
                        .to_attack_every_unit_with_extra_effect_request(
                            opponent_unique_id,
                            non_target_skill_damage,
                            extra_effect_list_of_unit_using_skill)).await;
            } else {
                game_field_unit_service_guard.apply_catastrophic_damage_to_field_unit(
                    turn_start_non_targeting_attack_passive_skill_request_form
                        .to_apply_catastrophic_damage_to_field_unit_request(
                            opponent_unique_id,
                            non_target_skill_damage)).await;
            }
        } else {
            println!("현재 구현되지 않은 논타겟 스킬 기능입니다.");
            return TurnStartNonTargetingAttackPassiveSkillResponseForm::default()
        }
        // TODO: 사망 판정 전에 데이터를 가져와야 전송 가능하여 위치 변경 (데미지 - 데이터 - 사망판정 순서)
        let opponent_all_unit_health_point =
            game_field_unit_service_guard.get_current_health_point_of_all_field_unit(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_get_current_health_point_of_all_field_unit_request(
                        opponent_unique_id)).await.get_current_unit_health_point().clone();

        let opponent_all_unit_harmful_effect =
            game_field_unit_service_guard.acquire_harmful_status_effect_of_all_unit(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_acquire_harmful_status_effect_of_all_unit_request(
                        opponent_unique_id)).await.get_harmful_effect_list_of_all_unit();

        // 유닛 사망 처리
        let judge_death_of_every_unit_response =
            game_field_unit_service_guard.judge_death_of_every_field_unit(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_judge_death_of_every_unit_request(
                        opponent_unique_id)).await;

        if !judge_death_of_every_unit_response.get_dead_unit_id_list().is_empty() {
            let mut game_tomb_service_guard =
                self.game_tomb_service.lock().await;

            game_tomb_service_guard.add_dead_unit_list_to_tomb(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_add_dead_unit_list_to_tomb_request(
                        opponent_unique_id,
                        judge_death_of_every_unit_response.get_dead_unit_id_list())).await;

            drop(game_tomb_service_guard);
        }

        // 유닛의 해당 패시브 스킬을 false로 세팅
        game_field_unit_service_guard.execute_index_passive_of_unit(
            turn_start_non_targeting_attack_passive_skill_request_form
                .to_execute_index_passive_of_unit_request(
                    account_unique_id,
                    unit_card_index,
                    usage_skill_index)).await;



        // 13. 해당 유닛의 소환시 발동되는 패시브 스킬이 더 있는지 알려줘야함 .
        let passive_usable_list =
            game_field_unit_service_guard.get_passive_skill_usable(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_get_passive_skill_usable_list_request(
                        account_unique_id,
                        unit_card_index)).await.get_passive_skill_usable_list();

        drop(game_field_unit_service_guard);


        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_turn_start_passive_skill_effect_list =
            game_card_passive_skill_service_guard.summary_turn_start_passive_skill(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_summary_turn_start_passive_skill_effect_request(
                        unit_card_id)).await.get_passive_skill_effect_list();

        let mut passive_type_list = Vec::new();

        for i in 0..3 {
            if passive_usable_list[i] == true && summary_turn_start_passive_skill_effect_list[i].get_passive_skill_type() != &PassiveSkillType::Dummy {
                passive_type_list.push(summary_turn_start_passive_skill_effect_list[i].get_passive_skill_type());
            }
            else {
                passive_type_list.push(&PassiveSkillType::Dummy);

            }
        }
        drop(game_card_passive_skill_service_guard);
        // TODO: 스킬 사용으로 인한 광역 논타켓 데미지 알림 + 남은 체력 알림 + 사망 사실 알림 각각 따로따로
        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_opponent_multiple_unit_health_point_data_response =
            ui_data_generator_service_guard.generate_opponent_multiple_unit_health_point_data(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_multiple_unit_health_point_data_request(opponent_all_unit_health_point)).await;

        let generate_opponent_multiple_unit_harmful_effect_data_response =
            ui_data_generator_service_guard.generate_opponent_multiple_unit_harmful_effect_data(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_multiple_unit_harmful_effect_data_request(opponent_all_unit_harmful_effect)).await;

        let generate_opponent_multiple_unit_death_data_response =
            ui_data_generator_service_guard.generate_opponent_multiple_unit_death_data(
                turn_start_non_targeting_attack_passive_skill_request_form
                    .to_generate_opponent_multiple_unit_death_data_request(
                        judge_death_of_every_unit_response.get_dead_unit_index_list())).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_turn_start_non_targeting_attack_passive_skill(
            turn_start_non_targeting_attack_passive_skill_request_form
                .to_notice_turn_start_non_targeting_attack_passive_skill_request(
                    opponent_unique_id,
                    generate_opponent_multiple_unit_health_point_data_response
                        .get_player_field_unit_health_point_map_for_notice().clone(),
                    generate_opponent_multiple_unit_harmful_effect_data_response
                        .get_player_field_unit_harmful_effect_map_for_notice().clone(),
                    generate_opponent_multiple_unit_death_data_response
                        .get_player_field_unit_death_map_for_notice().clone())).await;

        drop(notify_player_action_info_service_guard);

        TurnStartNonTargetingAttackPassiveSkillResponseForm::new(
            true,
            generate_opponent_multiple_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_response().clone(),
            generate_opponent_multiple_unit_harmful_effect_data_response
                .get_player_field_unit_harmful_effect_map_for_response().clone(),
            generate_opponent_multiple_unit_death_data_response
                .get_player_field_unit_death_map_for_response().clone())
    }
    async fn request_turn_start_targeting_attack_to_game_main_character(
        &self, turn_start_targeting_attack_to_game_main_character_request_form: TurnStartTargetingAttackToGameMainCharacterRequestForm)
        -> TurnStartTargetingAttackToGameMainCharacterResponseForm {

        println!("GameCardUnitControllerImpl: request_to_attack_game_main_character()");

        // 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(
            turn_start_targeting_attack_to_game_main_character_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return TurnStartTargetingAttackToGameMainCharacterResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return TurnStartTargetingAttackToGameMainCharacterResponseForm::default()
        }

        // Battle Field 에서 공격하는 유닛의 index 를 토대로 id 값 확보
        let unit_card_index_string = turn_start_targeting_attack_to_game_main_character_request_form.get_attacker_unit_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let usage_skill_index_string = turn_start_targeting_attack_to_game_main_character_request_form.get_usage_skill_index();
        let usage_skill_index = usage_skill_index_string.parse::<i32>().unwrap();


        // 액션 가능한 턴인지 검증
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let unit_card_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_found_opponent_unit_id();

        if unit_card_id == -1 {
            println!("필드에 존재하지 않는 유닛을 지정하여 보냈으므로 해킹범입니다!");
            return TurnStartTargetingAttackToGameMainCharacterResponseForm::default()
        }

        let mut game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let summary_passive_skill_effect_by_index_response =
            game_card_passive_skill_service_guard.summary_passive_skill_by_index(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_summary_passive_skill_effect_by_index_request(
                        unit_card_id,
                        usage_skill_index)).await;

        if summary_passive_skill_effect_by_index_response.get_passive_skill_type() != &PassiveSkillType::SingleTarget {
            println!("단일 공격 패시브가 아닙니다.");
            return TurnStartTargetingAttackToGameMainCharacterResponseForm::default()
        }

        drop(game_card_passive_skill_service_guard);

        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_using_turn_start_passive_skill_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_using_turn_start_passive_skill_possible(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_is_using_turn_start_passive_skill_possible_request(
                        account_unique_id,
                        unit_card_index,
                        usage_skill_index,
                        summary_passive_skill_effect_by_index_response.get_passive_skill_casting_condition().clone())).await;

        if !is_using_turn_start_passive_skill_possible_response.is_possible() {
            return TurnStartTargetingAttackToGameMainCharacterResponseForm::default()
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 패시브 정보 에서 공격력 정보 확보
        let attacker_unit_attack_point =
            summary_passive_skill_effect_by_index_response.get_skill_damage();

        // 공격을 위해 상대방 고유값 획득
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut game_main_character_service_guard =
            self.game_main_character_service.lock().await;

        game_main_character_service_guard.apply_damage_to_main_character(
            turn_start_targeting_attack_to_game_main_character_request_form
                .to_apply_damage_to_main_character_request(
                    opponent_unique_id,
                    attacker_unit_attack_point)).await;

        // 액션 완료 설정
        game_field_unit_service_guard.execute_index_passive_of_unit(
            turn_start_targeting_attack_to_game_main_character_request_form
                .to_execute_index_passive_of_unit_request(
                    account_unique_id,
                    unit_card_index,
                    usage_skill_index)).await;

        drop(game_field_unit_service_guard);

        let check_main_character_of_account_unique_id_response =
            game_main_character_service_guard.check_main_character_of_account_unique_id(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_check_main_character_of_account_unique_id_request(opponent_unique_id)).await;

        // 사망하면 상대 패배 결정
        if check_main_character_of_account_unique_id_response.get_status_main_character() == &StatusMainCharacterEnum::Death {
            let mut game_winner_check_service_guard =
                self.game_winner_check_service.lock().await;

            game_winner_check_service_guard.check_health_of_main_character_for_setting_game_winner(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_check_main_character_for_setting_game_winner_request(
                        account_unique_id,
                        opponent_unique_id)).await;

            drop(game_winner_check_service_guard);
        }

        drop(game_main_character_service_guard);

        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_opponent_main_character_health_point_data_response =
            ui_data_generator_service_guard.generate_opponent_main_character_health_point_data(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_generate_opponent_main_character_health_point_data_request(
                        check_main_character_of_account_unique_id_response.get_current_health_point())).await;

        let generate_opponent_main_character_survival_data_response =
            ui_data_generator_service_guard.generate_opponent_main_character_survival_data(
                turn_start_targeting_attack_to_game_main_character_request_form
                    .to_generate_opponent_main_character_survival_data_request(
                        check_main_character_of_account_unique_id_response.get_status_main_character().clone())).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_basic_attack_to_main_character(
            turn_start_targeting_attack_to_game_main_character_request_form
                .to_notice_basic_attack_to_main_character_request(
                    opponent_unique_id,
                    generate_opponent_main_character_health_point_data_response
                        .get_player_main_character_health_point_map_for_notice().clone(),
                    generate_opponent_main_character_survival_data_response
                        .get_player_main_character_survival_map_for_notice().clone())).await;

        drop(notify_player_action_info_service_guard);

        TurnStartTargetingAttackToGameMainCharacterResponseForm::from_response(
            generate_opponent_main_character_health_point_data_response,
            generate_opponent_main_character_survival_data_response)
    }
}