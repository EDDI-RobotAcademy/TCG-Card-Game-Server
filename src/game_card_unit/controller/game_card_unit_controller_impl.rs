use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use diesel::IntoSql;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::game_card_passive_skill::entity::passive_skill_casting_condition::PassiveSkillCastingCondition;

use crate::game_card_passive_skill::service::game_card_passive_skill_service::GameCardPassiveSkillService;
use crate::game_card_passive_skill::service::game_card_passive_skill_service_impl::GameCardPassiveSkillServiceImpl;
use crate::game_card_unit::controller::game_card_unit_controller::GameCardUnitController;
use crate::game_card_unit::controller::request_form::attack_game_main_character_request_form::AttackGameMainCharacterRequestForm;
use crate::game_card_unit::controller::request_form::attack_unit_request_form::AttackUnitRequestForm;
use crate::game_card_unit::controller::request_form::deploy_unit_request_form::DeployUnitRequestForm;
use crate::game_card_unit::controller::response_form::attack_game_main_character_response_form::AttackGameMainCharacterResponseForm;
use crate::game_card_unit::controller::response_form::attack_unit_response_form::AttackUnitResponseForm;
use crate::game_card_unit::controller::response_form::deploy_unit_response_form::DeployUnitResponseForm;
use crate::game_card_unit::entity::passive_status::PassiveStatus;
use crate::game_card_unit::service::game_card_unit_service::GameCardUnitService;

use crate::game_card_unit::service::game_card_unit_service_impl::GameCardUnitServiceImpl;
use crate::game_field_unit::entity::extra_effect::ExtraEffect::{DarkFire, Freeze};
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
use crate::game_field_unit::entity::harmful_status_effect::HarmfulStatusEffect;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service::GameFieldUnitActionPossibilityValidatorService;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service_impl::GameFieldUnitActionPossibilityValidatorServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_main_character::service::game_main_character_service_impl::GameMainCharacterServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_winner_check::service::game_winner_check_service::GameWinnerCheckService;
use crate::game_winner_check::service::game_winner_check_service_impl::GameWinnerCheckServiceImpl;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::notify_player_action_info_service_impl::NotifyPlayerActionInfoServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

pub struct GameCardUnitControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_card_unit_service: Arc<AsyncMutex<GameCardUnitServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_field_unit_action_possibility_validator_service: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>>,
    game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
    game_winner_check_service: Arc<AsyncMutex<GameWinnerCheckServiceImpl>>,
    notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
}

impl GameCardUnitControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_card_unit_service: Arc<AsyncMutex<GameCardUnitServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_field_unit_action_possibility_validator_service: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>>,
               game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
               game_winner_check_service: Arc<AsyncMutex<GameWinnerCheckServiceImpl>>,
               notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
    ) -> Self {

        GameCardUnitControllerImpl {
            game_hand_service,
            battle_room_service,
            game_card_unit_service,
            game_field_unit_service,
            redis_in_memory_service,
            game_tomb_service,
            notify_player_action_service,
            game_card_passive_skill_service,
            game_protocol_validation_service,
            game_field_unit_action_possibility_validator_service,
            game_main_character_service,
            game_winner_check_service,
            notify_player_action_info_service,
            ui_data_generator_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardUnitControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardUnitControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardUnitControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameCardUnitServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameCardPassiveSkillServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            GameFieldUnitActionPossibilityValidatorServiceImpl::get_instance(),
                            GameMainCharacterServiceImpl::get_instance(),
                            GameWinnerCheckServiceImpl::get_instance(),
                            NotifyPlayerActionInfoServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    async fn is_able_to_use(&self, can_use_card_request: CanUseCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(can_use_card_request).await;
        drop(game_protocol_validation_service_guard);
        can_use_card_response.is_success()
    }
}

#[async_trait]
impl GameCardUnitController for GameCardUnitControllerImpl {
    async fn request_to_deploy_unit(&self, deploy_unit_request_form: DeployUnitRequestForm) -> DeployUnitResponseForm {
        println!("GameCardUnitControllerImpl: request_to_deploy_unit()");

        // 1. 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(deploy_unit_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return DeployUnitResponseForm::new(false, -1)
        }

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let unit_id_string = deploy_unit_request_form.get_unit_id();
        let unit_card_id = unit_id_string.parse::<i32>().unwrap();

        // 2. Game Protocol Validation Service 호출하여 필수 요소 검증
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let check_protocol_hacking_response =
            game_protocol_validation_service_guard.check_protocol_hacking(
                deploy_unit_request_form
                    .to_check_protocol_hacking_request(account_unique_id, unit_card_id)).await;

        if !check_protocol_hacking_response.is_success() {
            println!("핸드에 존재하지 않는 유닛 소환 요청 - 해킹범을 검거합니다!");
            return DeployUnitResponseForm::new(false, -1)
        }

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                deploy_unit_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return DeployUnitResponseForm::new(false, -1)
        }

        // 3. Card Kinds Service 를 호출하여 실제 유닛 카드가 맞는지 확인
        let is_it_unit_response =
            game_protocol_validation_service_guard.is_it_unit_card(
                deploy_unit_request_form.to_is_it_unit_card_request(unit_card_id)).await;

        if !is_it_unit_response.is_success() {
            println!("유닛 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return DeployUnitResponseForm::new(false, -1)
        }

        // 4. 신화 등급의 경우 라운드 체크하도록 함
        let can_use_card_response =
            game_protocol_validation_service_guard.can_use_card(
                deploy_unit_request_form
                    .to_can_use_card_request(account_unique_id, unit_card_id)).await;

        if !can_use_card_response.is_success() {
            println!("신화 등급 카드는 5라운드부터 사용 가능합니다.");
            return DeployUnitResponseForm::new(false, -1)
        }

        drop(game_protocol_validation_service_guard);

        // 5. Hand Service 호출하여 카드 사용
        let mut game_hand_service_guard =
            self.game_hand_service.lock().await;

        let use_game_hand_unit_card_response =
            game_hand_service_guard.use_unit_card(
                deploy_unit_request_form
                    .to_use_game_hand_unit_card_request(account_unique_id, unit_card_id)).await;

        drop(game_hand_service_guard);

        let usage_hand_card_id = use_game_hand_unit_card_response.get_found_unit_card_id();

        // 6. Battle Field 에 유닛 배치
        let mut game_card_service_guard =
            self.game_card_unit_service.lock().await;

        let unit_card_info_response =
            game_card_service_guard.summary_unit_card(
                deploy_unit_request_form.to_summary_unit_card_info_request(unit_card_id)).await;

        drop(game_card_service_guard);

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let add_unit_to_game_field_response =
            game_field_unit_service_guard.add_unit_to_game_field(
                deploy_unit_request_form.to_add_unit_to_game_field_request(
                    account_unique_id,
                    usage_hand_card_id,
                    unit_card_info_response.get_unit_race(),
                    unit_card_info_response.get_unit_grade(),
                    unit_card_info_response.get_unit_attack_point(),
                    unit_card_info_response.get_unit_health_point(),
                    unit_card_info_response.get_unit_attack_required_energy(),
                    unit_card_info_response.has_first_passive_skill(),
                    unit_card_info_response.has_second_passive_skill(),
                    unit_card_info_response.has_third_passive_skill(),
                    unit_card_info_response.get_passive_status_list().clone())).await;

        drop(game_field_unit_service_guard);

        if add_unit_to_game_field_response.get_placed_unit_index() == -1 {
            println!("필드에 유닛 배치 중 문제가 발생하였습니다.");
            return DeployUnitResponseForm::new(false, -1)
        }

        // 7. 유닛이 출격하자마자 발동하는 스킬이 있는지 확인하여 카운트
        let game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let passive_skill_response =
            game_card_passive_skill_service_guard.summary_passive_skill(
                deploy_unit_request_form.to_summary_passive_skill_request(usage_hand_card_id)).await;

        drop(game_card_passive_skill_service_guard);

        let mut passive_skill_list =
            passive_skill_response.get_passive_skill_effect_list().clone();

        let mut number_of_passive_skill_triggered_by_deploying_unit = 0;
        for passive_skill in passive_skill_list {
            if passive_skill.get_passive_skill_casting_condition().contains(&PassiveSkillCastingCondition::Deploy) {
                number_of_passive_skill_triggered_by_deploying_unit += 1;
            }
        }

        // 8. 핸드에 있던 유닛 카드 사용에 대한 데이터 생성
        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_use_my_hand_card_data_response =
            ui_data_generator_service_guard.generate_use_my_hand_card_data(
                deploy_unit_request_form
                    .to_generate_use_my_hand_card_data_request(unit_card_id)).await;

        drop(ui_data_generator_service_guard);

        // 9. 상대방의 고유 id 값을 확보
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let find_opponent_by_account_id_response =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                deploy_unit_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await;

        drop(battle_room_service_guard);

        // 10. 상대방에게 당신이 무엇을 했는지 알려줘야 합니다
        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_use_unit_card(
            deploy_unit_request_form
                .to_notice_use_unit_card_request(
                    find_opponent_by_account_id_response
                        .get_opponent_unique_id(),
                    generate_use_my_hand_card_data_response
                        .get_player_hand_use_map_for_notice().clone())).await;

        drop(notify_player_action_info_service_guard);

        DeployUnitResponseForm::new(true, number_of_passive_skill_triggered_by_deploying_unit)
    }

    async fn request_to_attack_unit(
        &self, attack_unit_request_form: AttackUnitRequestForm) -> AttackUnitResponseForm {

        println!("GameCardUnitControllerImpl: request_to_attack_unit()");

        // 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(
            attack_unit_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return AttackUnitResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                attack_unit_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return AttackUnitResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // Battle Field 에서 공격하는 유닛의 index 를 토대로 id 값 확보
        let attacker_unit_card_index_string = attack_unit_request_form.get_attacker_unit_index();
        let attacker_unit_card_index = attacker_unit_card_index_string.parse::<i32>().unwrap();

        // 액션 가능한 턴인지 검증
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let attacker_unit_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                attack_unit_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_found_opponent_unit_id();

        if attacker_unit_id == -1 {
            println!("필드에 존재하지 않는 유닛을 지정하여 보냈으므로 해킹범입니다!");
            return AttackUnitResponseForm::default()
        }

        let mut game_card_unit_service_guard =
            self.game_card_unit_service.lock().await;

        let attacker_unit_required_energy =
            game_card_unit_service_guard.summary_unit_card(
                attack_unit_request_form.to_summary_unit_card_info_request(
                    attacker_unit_id)).await.get_unit_attack_required_energy();

        drop(game_card_unit_service_guard);

        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_unit_basic_attack_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_unit_basic_attack_possible(
                attack_unit_request_form.to_is_unit_basic_attack_possible_request(
                    account_unique_id, attacker_unit_card_index, attacker_unit_required_energy)).await;

        if !is_unit_basic_attack_possible_response.is_possible() {
            return AttackUnitResponseForm::default()
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 유닛 인덱스에서 기본 공격력 정보 확보
        let find_attacker_unit_attack_point_response =
            game_field_unit_service_guard.acquire_unit_attack_point(
                attack_unit_request_form
                    .to_acquire_unit_attack_point_request(
                        account_unique_id,
                        attacker_unit_card_index)).await;

        // extra effect 가지고 있는지 여부
        let attacker_unit_extra_effect_list =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                attack_unit_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_extra_status_effect_list().clone();

        // 공격을 위해 상대방 고유값 획득
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                attack_unit_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 피격 유닛이 기본 공격 면역을 가지고 있는지 확인
        let opponent_target_unit_card_index_string = attack_unit_request_form.get_target_unit_index();
        let opponent_target_unit_card_index = opponent_target_unit_card_index_string.parse::<i32>().unwrap();

        let opponent_target_unit_passive_status_list =
            game_field_unit_service_guard.acquire_unit_passive_status_list(
                attack_unit_request_form
                    .to_acquire_unit_passive_status_list_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_passive_status_effect_list().clone();

        if opponent_target_unit_passive_status_list.contains(&PassiveStatus::PhysicalImmunity) {
            println!("기본 공격 면역 패시브로 인해 공격을 가할 수 없습니다.");
            return AttackUnitResponseForm::default()
        }

        // 적 타겟 유닛을 효과를 가지고 공격
        let attack_opponent_target_unit_with_extra_effect_response =
            game_field_unit_service_guard.attack_target_unit_with_extra_effect(
                attack_unit_request_form
                    .to_attack_target_unit_with_extra_effect_request(
                        opponent_unique_id,
                        find_attacker_unit_attack_point_response.get_attack_point(),
                        &attacker_unit_extra_effect_list,
                        opponent_target_unit_card_index)).await;

        if !attack_opponent_target_unit_with_extra_effect_response.is_success() {
            println!("필드에 존재하지 않는 유닛을 공격 대상으로 지정하여 공격에 실패했습니다.");
            return AttackUnitResponseForm::default()
        }

        let opponent_target_unit_health_point =
            game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                attack_unit_request_form
                    .to_get_current_health_point_of_field_unit_by_index_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_current_unit_health_point();

        let opponent_target_unit_harmful_effect_list =
            game_field_unit_service_guard.acquire_unit_harmful_status_effect(
                attack_unit_request_form
                    .to_acquire_unit_harmful_status_effect_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_harmful_effect_list().clone();

        // 반격 이전 공격 유닛의 기본 지속 상태 확보
        let attacker_unit_passive_status_list =
            game_field_unit_service_guard.acquire_unit_passive_status_list(
                attack_unit_request_form
                    .to_acquire_unit_passive_status_list_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_passive_status_effect_list().clone();

        // 공격 유닛이 기본 공격 면역일 경우 반격 무효 처리
        if attacker_unit_passive_status_list.contains(&PassiveStatus::PhysicalImmunity) {
            println!("공격한 유닛이 기본 공격 면역이 존재하여 반격이 적용되지 않습니다.");

            // 액션 완료 설정
            game_field_unit_service_guard.execute_turn_action(
                attack_unit_request_form
                    .to_execute_turn_action_request(
                        account_unique_id,
                        attacker_unit_card_index)).await;

            // 피격 유닛이 죽었는지 판정
            let judge_death_of_opponent_unit_response =
                game_field_unit_service_guard.judge_death_of_unit(
                    attack_unit_request_form
                        .to_judge_death_of_unit_request(
                            opponent_unique_id,
                            opponent_target_unit_card_index)).await;

            drop(game_field_unit_service_guard);

            // 죽은 경우 묘지에 추가
            let mut game_tomb_service_guard =
                self.game_tomb_service.lock().await;

            if judge_death_of_opponent_unit_response.get_dead_unit_id() != -1 {
                println!("공격 당한 유닛이 사망했으므로 묘지로 이동합니다.");

                game_tomb_service_guard.add_used_card_to_tomb(
                    attack_unit_request_form
                        .to_place_dead_unit_to_tomb_request(
                            opponent_unique_id,
                            judge_death_of_opponent_unit_response.get_dead_unit_id())).await;
            }

            drop(game_tomb_service_guard);

            let mut ui_data_generator_service_guard =
                self.ui_data_generator_service.lock().await;

            let generate_opponent_specific_unit_health_point_data_response =
                ui_data_generator_service_guard.generate_opponent_specific_unit_health_point_data(
                    attack_unit_request_form
                        .to_generate_opponent_specific_unit_health_point_data_request(
                            opponent_target_unit_card_index,
                            opponent_target_unit_health_point)).await;

            let generate_opponent_specific_unit_harmful_effect_data_response =
                ui_data_generator_service_guard.generate_opponent_specific_unit_harmful_effect_data(
                    attack_unit_request_form
                        .to_generate_opponent_specific_unit_harmful_effect_data_request(
                            opponent_target_unit_card_index,
                            opponent_target_unit_harmful_effect_list)).await;

            let generate_opponent_specific_unit_death_data_response =
                ui_data_generator_service_guard.generate_opponent_specific_unit_death_data(
                    attack_unit_request_form
                        .to_generate_opponent_specific_unit_death_data_request(
                            judge_death_of_opponent_unit_response.get_dead_unit_index())).await;

            drop(ui_data_generator_service_guard);

            let mut notify_player_action_info_service_guard =
                self.notify_player_action_info_service.lock().await;

            notify_player_action_info_service_guard.notice_basic_attack_to_unit(
                attack_unit_request_form
                    .to_notice_basic_attack_to_unit_request(
                        opponent_unique_id,
                        generate_opponent_specific_unit_health_point_data_response
                            .get_player_field_unit_health_point_map_for_notice().clone(),
                        generate_opponent_specific_unit_harmful_effect_data_response
                            .get_player_field_unit_harmful_effect_map_for_notice().clone(),
                        generate_opponent_specific_unit_death_data_response
                            .get_player_field_unit_death_map_for_notice().clone())).await;

            drop(notify_player_action_info_service_guard);

            return AttackUnitResponseForm::new(
                true,
                generate_opponent_specific_unit_health_point_data_response
                    .get_player_field_unit_health_point_map_for_response().clone(),
                generate_opponent_specific_unit_harmful_effect_data_response
                    .get_player_field_unit_harmful_effect_map_for_response().clone(),
                generate_opponent_specific_unit_death_data_response
                    .get_player_field_unit_death_map_for_response().clone())
        }

        // 반격을 위해 피격 유닛의 공격력 확보
        let find_opponent_target_unit_attack_point_response =
            game_field_unit_service_guard.acquire_unit_attack_point(
                attack_unit_request_form
                    .to_acquire_unit_attack_point_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await;

        // 피격 유닛이 extra effect 를 가지고 있는지 여부
        let opponent_target_unit_extra_effect_list =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                attack_unit_request_form
                    .to_acquire_unit_extra_effect_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_extra_status_effect_list().clone();

        // 공격한 유닛에게 피격 유닛의 효과와 함께 반격 적용
        game_field_unit_service_guard.attack_target_unit_with_extra_effect(
            attack_unit_request_form
                .to_attack_target_unit_with_extra_effect_request(
                    account_unique_id,
                    find_opponent_target_unit_attack_point_response.get_attack_point(),
                    &opponent_target_unit_extra_effect_list,
                    attacker_unit_card_index)).await;

        let attacker_unit_health_point =
            game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                attack_unit_request_form
                    .to_get_current_health_point_of_field_unit_by_index_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_current_unit_health_point();

        let attacker_unit_harmful_effect_list =
            game_field_unit_service_guard.acquire_unit_harmful_status_effect(
                attack_unit_request_form
                    .to_acquire_unit_harmful_status_effect_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_harmful_effect_list().clone();

        // 액션 완료 설정
        game_field_unit_service_guard.execute_turn_action(
            attack_unit_request_form
                .to_execute_turn_action_request(
                    account_unique_id,
                    attacker_unit_card_index)).await;

        // 유닛들이 죽었는지 판정
        let judge_death_of_opponent_unit_response =
            game_field_unit_service_guard.judge_death_of_unit(
                attack_unit_request_form
                    .to_judge_death_of_unit_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await;


        // 죽은 유닛의 경우 묘지에 추가
        let mut game_tomb_service_guard =
            self.game_tomb_service.lock().await;

        if judge_death_of_opponent_unit_response.get_dead_unit_id() != -1 {
            println!("공격 당한 유닛이 사망했으므로 묘지로 이동합니다.");

            game_tomb_service_guard.add_used_card_to_tomb(
                attack_unit_request_form
                    .to_place_dead_unit_to_tomb_request(
                        opponent_unique_id,
                        judge_death_of_opponent_unit_response.get_dead_unit_id())).await;
        }

        let judge_death_of_attacker_unit_response =
            game_field_unit_service_guard.judge_death_of_unit(
                attack_unit_request_form
                    .to_judge_death_of_unit_request(
                        account_unique_id,
                        attacker_unit_card_index)).await;

        if judge_death_of_attacker_unit_response.get_dead_unit_id() != -1 {
            println!("반격 당한 유닛이 사망했으므로 묘지로 이동합니다.");

            game_tomb_service_guard.add_used_card_to_tomb(
                attack_unit_request_form
                    .to_place_dead_unit_to_tomb_request(
                        account_unique_id,
                        judge_death_of_attacker_unit_response.get_dead_unit_id())).await;
        }

        drop(game_field_unit_service_guard);
        drop(game_tomb_service_guard);

        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_opponent_specific_unit_health_point_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_health_point_data(
                attack_unit_request_form
                    .to_generate_opponent_specific_unit_health_point_data_request(
                        opponent_target_unit_card_index,
                        opponent_target_unit_health_point)).await;

        let generate_opponent_specific_unit_harmful_effect_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_harmful_effect_data(
                attack_unit_request_form
                    .to_generate_opponent_specific_unit_harmful_effect_data_request(
                        opponent_target_unit_card_index,
                        opponent_target_unit_harmful_effect_list)).await;

        let generate_opponent_specific_unit_death_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_death_data(
                attack_unit_request_form
                    .to_generate_opponent_specific_unit_death_data_request(
                        judge_death_of_opponent_unit_response.get_dead_unit_index())).await;

        let generate_my_specific_unit_health_point_data_response =
            ui_data_generator_service_guard.generate_my_specific_unit_health_point_data(
                attack_unit_request_form
                    .to_generate_my_specific_unit_health_point_data_request(
                        attacker_unit_card_index,
                        attacker_unit_health_point)).await;

        let generate_my_specific_unit_harmful_effect_data_response =
            ui_data_generator_service_guard.generate_my_specific_unit_harmful_effect_data(
                attack_unit_request_form
                    .to_generate_my_specific_unit_harmful_effect_data_request(
                        attacker_unit_card_index,
                        attacker_unit_harmful_effect_list)).await;

        let generate_my_specific_unit_death_data_response =
            ui_data_generator_service_guard.generate_my_specific_unit_death_data(
                attack_unit_request_form
                    .to_generate_my_specific_unit_death_data_request(
                        judge_death_of_attacker_unit_response.get_dead_unit_index())).await;

        drop(ui_data_generator_service_guard);

        // TODO: Need Refactor
        let mut combined_unit_health_point_data_for_response = HashMap::new();
        let mut combined_unit_harmful_effect_data_for_response = HashMap::new();
        let mut combined_unit_death_data_for_response = HashMap::new();
        let mut combined_unit_health_point_data_for_notice = HashMap::new();
        let mut combined_unit_harmful_effect_data_for_notice = HashMap::new();
        let mut combined_unit_death_data_for_notice = HashMap::new();

        combined_unit_health_point_data_for_response.extend(
            generate_opponent_specific_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_response().clone());
        combined_unit_health_point_data_for_response.extend(
            generate_my_specific_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_response().clone());
        combined_unit_harmful_effect_data_for_response.extend(
            generate_opponent_specific_unit_harmful_effect_data_response
                .get_player_field_unit_harmful_effect_map_for_response().clone());
        combined_unit_harmful_effect_data_for_response.extend(
            generate_my_specific_unit_harmful_effect_data_response
                .get_player_field_unit_harmful_effect_map_for_response().clone());
        combined_unit_death_data_for_response.extend(
            generate_opponent_specific_unit_death_data_response
                .get_player_field_unit_death_map_for_response().clone());
        combined_unit_death_data_for_response.extend(
            generate_my_specific_unit_death_data_response
                .get_player_field_unit_death_map_for_response().clone());

        combined_unit_health_point_data_for_notice.extend(
            generate_opponent_specific_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_notice().clone());
        combined_unit_health_point_data_for_notice.extend(
            generate_my_specific_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_notice().clone());
        combined_unit_harmful_effect_data_for_notice.extend(
            generate_opponent_specific_unit_harmful_effect_data_response
                .get_player_field_unit_harmful_effect_map_for_notice().clone());
        combined_unit_harmful_effect_data_for_notice.extend(
            generate_my_specific_unit_harmful_effect_data_response
                .get_player_field_unit_harmful_effect_map_for_notice().clone());
        combined_unit_death_data_for_notice.extend(
            generate_opponent_specific_unit_death_data_response
                .get_player_field_unit_death_map_for_notice().clone());
        combined_unit_death_data_for_notice.extend(
            generate_my_specific_unit_death_data_response
                .get_player_field_unit_death_map_for_notice().clone());

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_basic_attack_to_unit(
            attack_unit_request_form
                .to_notice_basic_attack_to_unit_request(
                    opponent_unique_id,
                    combined_unit_health_point_data_for_notice,
                    combined_unit_harmful_effect_data_for_notice,
                    combined_unit_death_data_for_notice)).await;

        drop(notify_player_action_info_service_guard);

        AttackUnitResponseForm::new(
            true,
            combined_unit_health_point_data_for_response,
            combined_unit_harmful_effect_data_for_response,
            combined_unit_death_data_for_response)
    }

    async fn request_to_attack_game_main_character(
        &self, attack_game_main_character_request_form: AttackGameMainCharacterRequestForm)
        -> AttackGameMainCharacterResponseForm {

        println!("GameCardUnitControllerImpl: request_to_attack_game_main_character()");

        // 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(
            attack_game_main_character_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return AttackGameMainCharacterResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                attack_game_main_character_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return AttackGameMainCharacterResponseForm::default()
        }

        // Battle Field 에서 공격하는 유닛의 index 를 토대로 id 값 확보
        let attacker_unit_card_index_string = attack_game_main_character_request_form.get_attacker_unit_index();
        let attacker_unit_card_index = attacker_unit_card_index_string.parse::<i32>().unwrap();

        // 액션 가능한 턴인지 검증
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let attacker_unit_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                attack_game_main_character_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_found_opponent_unit_id();

        if attacker_unit_id == -1 {
            println!("필드에 존재하지 않는 유닛을 지정하여 보냈으므로 해킹범입니다!");
            return AttackGameMainCharacterResponseForm::default()
        }

        let mut game_card_unit_service_guard =
            self.game_card_unit_service.lock().await;

        let attacker_unit_required_energy =
            game_card_unit_service_guard.summary_unit_card(
                attack_game_main_character_request_form.to_summary_unit_card_info_request(
                    attacker_unit_id)).await.get_unit_attack_required_energy();

        drop(game_card_unit_service_guard);

        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_unit_basic_attack_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_unit_basic_attack_possible(
                attack_game_main_character_request_form
                    .to_is_unit_basic_attack_possible_request(
                        account_unique_id,
                        attacker_unit_card_index,
                        attacker_unit_required_energy)).await;

        if !is_unit_basic_attack_possible_response.is_possible() {
            return AttackGameMainCharacterResponseForm::default()
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 유닛 인덱스에서 기본 공격력 정보 확보
        let attacker_unit_attack_point =
            game_field_unit_service_guard.acquire_unit_attack_point(
                attack_game_main_character_request_form
                    .to_acquire_unit_attack_point_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_attack_point();

        // 공격을 위해 상대방 고유값 획득
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                attack_game_main_character_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut game_main_character_service_guard =
            self.game_main_character_service.lock().await;

        game_main_character_service_guard.apply_damage_to_main_character(
           attack_game_main_character_request_form
               .to_apply_damage_to_main_character_request(
                   opponent_unique_id,
                   attacker_unit_attack_point)).await;

        // 액션 완료 설정
        game_field_unit_service_guard.execute_turn_action(
            attack_game_main_character_request_form
                .to_execute_turn_action_request(
                    account_unique_id,
                    attacker_unit_card_index)).await;

        drop(game_field_unit_service_guard);

        let check_main_character_of_account_unique_id_response =
            game_main_character_service_guard.check_main_character_of_account_unique_id(
                attack_game_main_character_request_form
                    .to_check_main_character_of_account_unique_id_request(opponent_unique_id)).await;

        // 사망하면 상대 패배 결정
        if check_main_character_of_account_unique_id_response.get_status_main_character() == &StatusMainCharacterEnum::Death {
            let mut game_winner_check_service_guard =
                self.game_winner_check_service.lock().await;

            game_winner_check_service_guard.check_health_of_main_character_for_setting_game_winner(
                attack_game_main_character_request_form
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
                attack_game_main_character_request_form
                    .to_generate_opponent_main_character_health_point_data_request(
                        check_main_character_of_account_unique_id_response.get_current_health_point())).await;

        let generate_opponent_main_character_survival_data_response =
            ui_data_generator_service_guard.generate_opponent_main_character_survival_data(
                attack_game_main_character_request_form
                    .to_generate_opponent_main_character_survival_data_request(
                        check_main_character_of_account_unique_id_response.get_status_main_character().clone())).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_basic_attack_to_main_character(
            attack_game_main_character_request_form
                .to_notice_basic_attack_to_main_character_request(
                    opponent_unique_id,
                    generate_opponent_main_character_health_point_data_response
                        .get_player_main_character_health_point_map_for_notice().clone(),
                    generate_opponent_main_character_survival_data_response
                        .get_player_main_character_survival_map_for_notice().clone())).await;

        drop(notify_player_action_info_service_guard);

        AttackGameMainCharacterResponseForm::from_response(
            generate_opponent_main_character_health_point_data_response,
            generate_opponent_main_character_survival_data_response)
    }
}

#[cfg(test)]
mod tests {
    use crate::game_field_unit::entity::extra_effect::ExtraEffect::DarkFire;
    use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
    use crate::game_field_unit::entity::harmful_status_effect::HarmfulStatusEffect;
    use super::*;

    #[tokio::test]
    async fn test_attack_with_extra_effect_freeze_to_frozen_unit() {

        // Vec<ExtraStatusEffect> 생성
        let mut extra_effect_list: Vec<ExtraStatusEffect> = Vec::new();
        let extra_effect_list_01: ExtraStatusEffect = ExtraStatusEffect::new(DarkFire, 10, 9, 8);
        let extra_effect_list_02: ExtraStatusEffect = ExtraStatusEffect::new(Freeze, 7,6, 5);
        extra_effect_list.push(extra_effect_list_01);
        extra_effect_list.push(extra_effect_list_02);
        println!("extra_effect_list: {:?}", extra_effect_list);

        // Vec<HarmfulStatusEffect> 생성
        let mut harmful_effect_list: Vec<HarmfulStatusEffect> = Vec::new();
        let harmful_status_effect_01: HarmfulStatusEffect = HarmfulStatusEffect::new(Freeze, 6,5,4);
        let harmful_status_effect_02: HarmfulStatusEffect = HarmfulStatusEffect::new(DarkFire, 3,2,1);
        harmful_effect_list.push(harmful_status_effect_01);
        harmful_effect_list.push(harmful_status_effect_02);
        println!("harmful_effect_list: {:?}", harmful_effect_list);

        // 피격 유닛에게 Freeze 효과가 있고, 공격 유닛에게도 Freeze 가 있다면, attacker_extra_effect 에서 Freeze 제거
        for harmful_effect_index in (0..harmful_effect_list.len()).rev() {
            if *harmful_effect_list[harmful_effect_index].get_harmful_effect() == Freeze {
                for attacker_index in (0..extra_effect_list.len()).rev() {
                    if *extra_effect_list[attacker_index].get_extra_effect() == Freeze {
                        extra_effect_list.swap_remove(attacker_index);
                    }
                }
            }
        }

        for harmful_effect_index in (0..harmful_effect_list.len()).rev() {
            if *harmful_effect_list[harmful_effect_index].get_harmful_effect() == DarkFire {
                for attacker_index in (0..extra_effect_list.len()).rev() {
                    if *extra_effect_list[attacker_index].get_extra_effect() == DarkFire {
                        harmful_effect_list.swap_remove(harmful_effect_index);
                    }
                }
            }
        }

        println!("extra_effect_list after test: {:?}", extra_effect_list);
        println!("harmful_effect_list after test: {:?}", harmful_effect_list);
    }
}