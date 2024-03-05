use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::battle_start::controller::battle_start_controller::BattleStartController;
use crate::battle_start::controller::request_form::battle_start_request_form::BattleStartRequestForm;
use crate::battle_start::controller::response_form::battle_start_response_form::BattleStartResponseForm;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_turn::service::game_turn_service::GameTurnService;
use crate::game_turn::service::game_turn_service_impl::GameTurnServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::rock_paper_scissors::entity::rock_paper_scissors_result::RockPaperScissorsResult::{LOSE, WIN};
use crate::rock_paper_scissors::service::rock_paper_scissors_service::RockPaperScissorsService;
use crate::rock_paper_scissors::service::rock_paper_scissors_service_impl::RockPaperScissorsServiceImpl;
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

pub struct BattleStartControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
    rock_paper_scissors_service: Arc<AsyncMutex<RockPaperScissorsServiceImpl>>,
}

impl BattleStartControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
               rock_paper_scissors_service: Arc<AsyncMutex<RockPaperScissorsServiceImpl>>,
    ) -> Self {
        BattleStartControllerImpl {
            game_hand_service,
            game_deck_service,
            game_turn_service,
            battle_room_service,
            redis_in_memory_service,
            game_field_energy_service,
            ui_data_generator_service,
            rock_paper_scissors_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<BattleStartControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleStartControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleStartControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameTurnServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            GameFieldEnergyServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance(),
                            RockPaperScissorsServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let mut redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;
        drop(redis_in_memory_service_guard);
        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    async fn get_opponent_unique_id(&self, find_opponent_by_account_id_request: FindOpponentByAccountIdRequest) -> i32 {
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(find_opponent_by_account_id_request).await;
        drop(battle_room_service_guard);
        find_opponent_by_account_id_response.get_opponent_unique_id()
    }
}

#[async_trait]
impl BattleStartController for BattleStartControllerImpl {
    async fn request_to_start_battle(
        &self, battle_start_request_form: BattleStartRequestForm)
        -> BattleStartResponseForm {

        println!("BattleStartControllerImpl: request_to_start_battle()");

        let account_unique_id = self.is_valid_session(
            battle_start_request_form
                .to_get_value_with_key_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return BattleStartResponseForm::default()
        }

        let opponent_unique_id =
            self.get_opponent_unique_id(
                battle_start_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await;

        if opponent_unique_id == -1 {
            println!("There is no opponent.");
            return BattleStartResponseForm::default()
        }

        let mut rock_paper_scissors_service_guard =
            self.rock_paper_scissors_service.lock().await;

        let check_rock_paper_scissors_winner_response =
            rock_paper_scissors_service_guard.check_rock_paper_scissors_winner(
                battle_start_request_form
                    .to_check_rock_paper_scissors_winner_request(account_unique_id, opponent_unique_id)).await;

        drop(rock_paper_scissors_service_guard);

        if check_rock_paper_scissors_winner_response.get_am_i_winner() == WIN {

            let mut game_turn_service_guard =
                self.game_turn_service.lock().await;

            game_turn_service_guard.next_turn(
                battle_start_request_form.to_next_turn_request(account_unique_id)).await;

            drop(game_turn_service_guard);

            let mut game_deck_service_guard =
                self.game_deck_service.lock().await;

            let first_turn_card_list_for_winner =
                game_deck_service_guard.draw_cards_from_deck(
                    battle_start_request_form
                        .to_draw_cards_from_deck_request(
                            account_unique_id)).await.get_drawn_card_list().clone();

            drop(game_deck_service_guard);

            let mut game_hand_service_guard =
                self.game_hand_service.lock().await;

            game_hand_service_guard.add_card_list_to_hand(
                battle_start_request_form
                    .to_add_card_list_to_hand_request(
                        account_unique_id,
                        first_turn_card_list_for_winner.clone())).await;

            drop(game_hand_service_guard);

            let mut game_field_energy_service_guard =
                self.game_field_energy_service.lock().await;

            game_field_energy_service_guard.add_field_energy_with_amount(
                battle_start_request_form
                    .to_add_field_energy_request(account_unique_id)).await;

            drop(game_field_energy_service_guard);

            let mut ui_data_generator_service_guard =
                self.ui_data_generator_service.lock().await;

            let generate_draw_my_deck_data_response =
                ui_data_generator_service_guard.generate_draw_my_deck_data(
                    battle_start_request_form
                        .to_generate_draw_my_deck_data_request(
                            first_turn_card_list_for_winner)).await;

            let generate_my_field_energy_data_response =
                ui_data_generator_service_guard.generate_my_field_energy_data(
                    battle_start_request_form
                        .to_generate_my_field_energy_data_request()).await;

            drop(ui_data_generator_service_guard);

            return BattleStartResponseForm::from_response_for_winner(
                generate_draw_my_deck_data_response,
                generate_my_field_energy_data_response)
        }

        if check_rock_paper_scissors_winner_response.get_am_i_winner() == LOSE {
            let mut ui_data_generator_service_guard =
                self.ui_data_generator_service.lock().await;

            let generate_draw_opponent_deck_data_response =
                ui_data_generator_service_guard.generate_draw_opponent_deck_data(
                    battle_start_request_form
                        .to_generate_draw_opponent_deck_data_request()).await;

            let generate_opponent_field_energy_response =
                ui_data_generator_service_guard.generate_opponent_field_energy_data(
                    battle_start_request_form
                        .to_generate_opponent_field_energy_data_request()).await;

            drop(ui_data_generator_service_guard);

            return BattleStartResponseForm::from_response_for_loser(
                generate_draw_opponent_deck_data_response,
                generate_opponent_field_energy_response)
        }

        BattleStartResponseForm::default()
    }
}