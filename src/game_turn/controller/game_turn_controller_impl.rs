use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use diesel::IntoSql;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::action_waiting_timer::service::action_waiting_timer_service::ActionWaitingTimerService;
use crate::action_waiting_timer::service::action_waiting_timer_service_impl::ActionWaitingTimerServiceImpl;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::game_card_passive_skill::entity::passive_skill_casting_condition::PassiveSkillCastingCondition::TurnStart;
use crate::game_card_passive_skill::service::game_card_passive_skill_service::GameCardPassiveSkillService;
use crate::game_card_passive_skill::service::game_card_passive_skill_service_impl::GameCardPassiveSkillServiceImpl;
use crate::game_card_support_usage_counter::service::game_card_support_usage_counter_service::GameCardSupportUsageCounterService;
use crate::game_card_support_usage_counter::service::game_card_support_usage_counter_service_impl::GameCardSupportUsageCounterServiceImpl;
use crate::game_card_unit::service::game_card_unit_service::GameCardUnitService;
use crate::game_card_unit::service::game_card_unit_service_impl::GameCardUnitServiceImpl;

use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::entity::extra_effect::ExtraEffect::Freeze;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;

use crate::game_turn::controller::game_turn_controller::GameTurnController;

use crate::game_turn::controller::request_form::turn_end_request_form::TurnEndRequestForm;

use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;
use crate::game_turn::service::game_turn_service::GameTurnService;

use crate::game_turn::service::game_turn_service_impl::GameTurnServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;

use crate::game_main_character::service::game_main_character_service_impl::GameMainCharacterServiceImpl;
use crate::game_round::service::game_round_service::GameRoundService;
use crate::game_round::service::game_round_service_impl::GameRoundServiceImpl;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_winner_check::service::game_winner_check_service::GameWinnerCheckService;
use crate::game_winner_check::service::game_winner_check_service_impl::GameWinnerCheckServiceImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::notify_player_action_info_service_impl::NotifyPlayerActionInfoServiceImpl;
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

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
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_card_support_usage_counter_service: Arc<AsyncMutex<GameCardSupportUsageCounterServiceImpl>>,
    game_card_unit_service: Arc<AsyncMutex<GameCardUnitServiceImpl>>,
    action_waiting_timer_service: Arc<AsyncMutex<ActionWaitingTimerServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
    notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
    game_winner_check_service: Arc<AsyncMutex<GameWinnerCheckServiceImpl>>,
    game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
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
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_card_support_usage_counter_service: Arc<AsyncMutex<GameCardSupportUsageCounterServiceImpl>>,
               game_card_unit_service: Arc<AsyncMutex<GameCardUnitServiceImpl>>,
               action_waiting_timer_service: Arc<AsyncMutex<ActionWaitingTimerServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
               notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
               game_winner_check_service: Arc<AsyncMutex<GameWinnerCheckServiceImpl>>,
               game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
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
            game_tomb_service,
            game_hand_service,
            game_card_support_usage_counter_service,
            game_card_unit_service,
            action_waiting_timer_service,
            ui_data_generator_service,
            notify_player_action_info_service,
            game_winner_check_service,
            game_card_passive_skill_service,
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
                            GameTombServiceImpl::get_instance(),
                            GameHandServiceImpl::get_instance(),
                            GameCardSupportUsageCounterServiceImpl::get_instance(),
                            GameCardUnitServiceImpl::get_instance(),
                            ActionWaitingTimerServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance(),
                            NotifyPlayerActionInfoServiceImpl::get_instance(),
                            GameWinnerCheckServiceImpl::get_instance(),
                            GameCardPassiveSkillServiceImpl::get_instance())));
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
        // 세션 검증
        let account_unique_id =
            self.is_valid_session(turn_end_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return TurnEndResponseForm::default()
        }

        // opponent unique id 찾기
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                turn_end_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 현재 요청한 사람이 이번 턴의 주도권을 가지고 있던 사람인지 검증
        // TODO: 다음 과정은 모든 액션에 추가되어야 함
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                turn_end_request_form.to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            return TurnEndResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // 자신의 필드 유닛들 중 턴 종료 시 데미지를 받는 케이스를 적용 (예: 화상 데미지)
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        game_field_unit_service_guard.apply_status_effect_damage_iteratively(
            turn_end_request_form
                .to_apply_status_effect_damage_iteratively_request(
                    account_unique_id)).await;

        let health_point_of_all_unit =
            game_field_unit_service_guard.get_current_health_point_of_all_field_unit(
                turn_end_request_form
                    .to_get_current_health_point_of_all_field_unit_request(
                        account_unique_id)).await.get_current_unit_health_point().clone();

        let harmful_effect_of_all_unit =
            game_field_unit_service_guard.acquire_harmful_status_effect_of_all_unit(
                turn_end_request_form
                    .to_acquire_harmful_status_effect_of_all_unit_request(
                        account_unique_id)).await.get_harmful_effect_list_of_all_unit();

        let judge_death_of_every_field_unit_response =
            game_field_unit_service_guard.judge_death_of_every_field_unit(
                turn_end_request_form
                    .to_judge_death_of_every_unit_request(account_unique_id)).await;

        if !judge_death_of_every_field_unit_response.get_dead_unit_id_list().is_empty() {
            let mut game_tomb_service_guard =
                self.game_tomb_service.lock().await;

            game_tomb_service_guard.add_dead_unit_list_to_tomb(
                turn_end_request_form.to_add_dead_unit_list_to_tomb_request(
                    account_unique_id,
                    judge_death_of_every_field_unit_response.get_dead_unit_id_list())).await;

            drop(game_tomb_service_guard);
        }

        // 자신 필드 유닛 Turn Action Value 초기화
        game_field_unit_service_guard.reset_turn_action_of_all_field_unit(
            turn_end_request_form
                .to_reset_turn_action_of_all_field_unit_request(account_unique_id)).await;

        // 패시브 스킬 사용 여부 초기화
        // TODO: Need Refactor
        let my_field_unit_list =
            game_field_unit_service_guard.get_game_field_unit_card_of_account_unique_id(
                turn_end_request_form
                    .to_get_game_field_unit_card_of_account_unique_id_request(account_unique_id)).await;

        let mut game_card_unit_service_guard =
            self.game_card_unit_service.lock().await;

        for (unit_index, field_unit) in my_field_unit_list.get_game_field_unit_card().iter().enumerate() {
            if field_unit.is_alive() {
                let unit_card_id = field_unit.get_card();
                println!("Passive Skill Reset Target: index - {}, card_id - {}", unit_index, unit_card_id);
                let get_summary_passive_default =
                    game_card_unit_service_guard.summary_unit_card_passive_default(
                        turn_end_request_form
                            .to_summary_unit_card_passive_default_request(unit_card_id)).await;

                let passive_default_list =
                    get_summary_passive_default.get_passive_default_list().clone();

                game_field_unit_service_guard.reset_all_passive_of_unit(
                    turn_end_request_form
                        .to_reset_all_passive_of_unit(
                            account_unique_id,
                            unit_index as i32,
                            passive_default_list)).await;
            }
        }

        drop(game_card_unit_service_guard);

        // 상대 필드 위 턴 시작 시 패시브를 발동하는 유닛들에 대한 정보 알려주기
        // TODO: Need Refactor
        let opponent_field_unit_list =
            game_field_unit_service_guard.get_game_field_unit_card_of_account_unique_id(
                turn_end_request_form
                    .to_get_game_field_unit_card_of_account_unique_id_request(opponent_unique_id)).await;

        let mut turn_start_passive_skill_list_of_unit_index_map = HashMap::new();

        let mut game_card_passive_skill_guard =
            self.game_card_passive_skill_service.lock().await;

        for (unit_index, field_unit) in opponent_field_unit_list.get_game_field_unit_card().iter().enumerate() {
            let mut turn_start_passive_skill_index_list_of_unit = Vec::new();
            if field_unit.is_alive() && !field_unit.get_harmful_status_list().contains(&Freeze){
                let passive_skill_effect_list =
                    game_card_passive_skill_guard.summary_turn_start_passive_skill(
                        turn_end_request_form
                            .to_summary_turn_start_passive_skill_effect_request(
                                field_unit.get_card())).await.get_passive_skill_effect_list();

                let mut passive_skill_index = 1;
                for passive_skill_effect in passive_skill_effect_list {
                    if passive_skill_effect.get_passive_skill_casting_condition().contains(&TurnStart) {
                        turn_start_passive_skill_index_list_of_unit.push(passive_skill_index);
                        passive_skill_index += 1;
                    }
                }
            }

            turn_start_passive_skill_list_of_unit_index_map
                .insert(unit_index as i32, turn_start_passive_skill_index_list_of_unit);
        }

        drop(game_card_passive_skill_guard);
        drop(game_field_unit_service_guard);

        let mut game_card_support_usage_counter_service_guard =
            self.game_card_support_usage_counter_service.lock().await;

        game_card_support_usage_counter_service_guard.reset_support_card_usage_count(
            turn_end_request_form
                .to_reset_support_card_usage_count_request(account_unique_id)).await;

        drop(game_card_support_usage_counter_service_guard);

        // 당신의 라운드 증가
        let mut game_round_service_guard =
            self.game_round_service.lock().await;

        game_round_service_guard.next_game_round_object(
            turn_end_request_form.to_next_round_request()).await;

        drop(game_round_service_guard);

        // 상대방의 턴 증가
        let mut game_turn_service_guard =
            self.game_turn_service.lock().await;

        game_turn_service_guard.next_turn(
            turn_end_request_form.to_next_turn_request(opponent_unique_id)).await;

        drop(game_turn_service_guard);

        // 상대방이 덱에서 카드를 드로우
        let mut game_deck_service_guard =
            self.game_deck_service.lock().await;

        let drawn_card_list =
            game_deck_service_guard.draw_cards_from_deck(
                turn_end_request_form
                    .to_draw_cards_from_deck_request(
                        opponent_unique_id)).await.get_drawn_card_list().clone();

        drop(game_deck_service_guard);

        if drawn_card_list.is_empty() {
            println!("계정 {}번의 덱사가 발생했습니다.", opponent_unique_id);

            let mut game_main_character_service_guard =
                self.game_main_character_service.lock().await;

            game_main_character_service_guard.set_main_character_as_death(
                turn_end_request_form
                    .to_set_main_character_as_death_request(opponent_unique_id)).await;

            drop(game_main_character_service_guard);

            let mut game_winner_check_service_guard =
                self.game_winner_check_service.lock().await;

            game_winner_check_service_guard.set_game_winner(
                turn_end_request_form
                    .to_set_game_winner_request(
                        account_unique_id, opponent_unique_id)).await;

            drop(game_winner_check_service_guard);

            let mut notify_player_action_info_service_guard =
                self.notify_player_action_info_service.lock().await;

            notify_player_action_info_service_guard.notice_my_turn_end(
                turn_end_request_form
                    .to_notice_my_turn_end_in_case_of_no_more_opponent_deck(opponent_unique_id)).await;

            drop(notify_player_action_info_service_guard);

            return TurnEndResponseForm::from_no_more_opponent_deck_response()
        }

        // 상대방이 드로우한 카드 핸드에 추가
        let mut game_hand_service_guard =
            self.game_hand_service.lock().await;

        game_hand_service_guard.add_card_list_to_hand(
            turn_end_request_form
                .to_add_card_list_to_hand_request(opponent_unique_id, drawn_card_list.clone())).await;

        drop(game_hand_service_guard);

        // 상대방이 필드에너지 획득
        let game_field_energy_service_guard =
            self.game_field_energy_service.lock().await;

        game_field_energy_service_guard.add_field_energy_with_amount(
                turn_end_request_form.to_add_field_energy_request(opponent_unique_id)).await;

        let remaining_opponent_field_energy =
            game_field_energy_service_guard.get_current_field_energy(
                turn_end_request_form
                    .to_get_current_field_energy_request(
                        opponent_unique_id)).await.get_field_energy_count();

        drop(game_field_energy_service_guard);

        // TODO: Action Timer Setting

        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_draw_opponent_deck_data_response =
            ui_data_generator_service_guard.generate_draw_opponent_deck_data(
                turn_end_request_form
                    .to_generate_draw_opponent_deck_data(drawn_card_list.clone())).await;

        let generate_opponent_field_energy_data_response =
            ui_data_generator_service_guard.generate_opponent_field_energy_data(
                turn_end_request_form
                    .to_generate_opponent_field_energy_data_request(remaining_opponent_field_energy)).await;

        let generate_my_multiple_unit_health_point_data_response =
            ui_data_generator_service_guard.generate_my_multiple_unit_health_point_data(
                turn_end_request_form
                    .to_generate_my_multiple_unit_health_point_data_request(health_point_of_all_unit)).await;

        let generate_my_multiple_unit_harmful_effect_data_response =
            ui_data_generator_service_guard.generate_my_multiple_unit_harmful_effect_data(
                turn_end_request_form
                    .to_generate_my_multiple_unit_harmful_effect_data_request(harmful_effect_of_all_unit)).await;

        let generate_my_multiple_unit_death_data_response =
            ui_data_generator_service_guard.generate_my_multiple_unit_death_data(
                turn_end_request_form
                    .to_generate_my_multiple_unit_death_data_request(
                        judge_death_of_every_field_unit_response.get_dead_unit_index_list())).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_my_turn_end(
            turn_end_request_form
                .to_notice_my_turn_end_request(
                    opponent_unique_id,
                    generate_draw_opponent_deck_data_response
                        .get_player_drawn_card_list_map_for_notice().clone(),
                    generate_opponent_field_energy_data_response
                        .get_player_field_energy_map_for_notice().clone(),
                    generate_my_multiple_unit_health_point_data_response
                        .get_player_field_unit_health_point_map_for_notice().clone(),
                    generate_my_multiple_unit_harmful_effect_data_response
                        .get_player_field_unit_harmful_effect_map_for_notice().clone(),
                    generate_my_multiple_unit_death_data_response
                        .get_player_field_unit_death_map_for_notice().clone(),
                    turn_start_passive_skill_list_of_unit_index_map)).await;

        drop(notify_player_action_info_service_guard);

        TurnEndResponseForm::from_response(
            generate_draw_opponent_deck_data_response,
            generate_opponent_field_energy_data_response,
            generate_my_multiple_unit_health_point_data_response,
            generate_my_multiple_unit_harmful_effect_data_response,
            generate_my_multiple_unit_death_data_response)
    }
}