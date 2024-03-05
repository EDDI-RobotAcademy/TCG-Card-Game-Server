use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::mulligan::controller::request_form::mulligan_request_form::MulliganRequestForm;
use crate::mulligan::controller::response_form::mulligan_response_form::MulliganResponseForm;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_turn::service::game_turn_service_impl::GameTurnServiceImpl;
use crate::mulligan::controller::mulligan_controller::MulliganController;
use crate::notify_player_action_info::service::notify_player_action_info_service_impl::NotifyPlayerActionInfoServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::rock_paper_scissors::service::rock_paper_scissors_service_impl::RockPaperScissorsServiceImpl;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

pub struct MulliganControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
    rock_paper_scissors_service: Arc<AsyncMutex<RockPaperScissorsServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
}

impl MulliganControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
               rock_paper_scissors_service: Arc<AsyncMutex<RockPaperScissorsServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
    ) -> Self {
        MulliganControllerImpl {
            game_hand_service,
            game_deck_service,
            game_turn_service,
            battle_room_service,
            redis_in_memory_service,
            game_field_energy_service,
            ui_data_generator_service,
            rock_paper_scissors_service,
            game_protocol_validation_service,
            notify_player_action_info_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<MulliganControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<MulliganControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        MulliganControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameTurnServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            GameFieldEnergyServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance(),
                            RockPaperScissorsServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            NotifyPlayerActionInfoServiceImpl::get_instance())));
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
}

#[async_trait]
impl MulliganController for MulliganControllerImpl {
    async fn execute_mulligan_procedure(
        &self, mulligan_request_form: MulliganRequestForm)
        -> MulliganResponseForm {

        println!("GameHandControllerImpl: execute_mulligan_procedure()");

        let account_unique_id = self.is_valid_session(
            mulligan_request_form
                .to_get_value_with_key_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return MulliganResponseForm::default()
        }

        // protocol validation service
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let check_hand_card_hacking_response =
            game_protocol_validation_service_guard.check_cards_from_hand(
                mulligan_request_form
                    .to_check_cards_from_hand_request()).await;

        if check_hand_card_hacking_response.get_is_success() == false {
            println!("Mulligan failed by protocol validation service");
            return MulliganResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // hand service
        let mut game_hand_service_guard =
            self.game_hand_service.lock().await;

        let put_cards_on_deck_response =
            game_hand_service_guard.put_hand_cards_to_deck(
                mulligan_request_form
                    .to_put_cards_on_deck_request()).await;

        if !put_cards_on_deck_response.get_is_success() {
            println!("Mulligan failed by hand service");
            return MulliganResponseForm::default()
        }

        // deck service
        let mut game_deck_service_guard =
            self.game_deck_service.lock().await;

        game_deck_service_guard.shuffle_deck(
            mulligan_request_form.to_shuffle_deck_request()).await;

        let redrawn_card_list =
            game_deck_service_guard.draw_cards_from_deck(
                mulligan_request_form
                    .to_draw_cards_from_deck_request(
                        account_unique_id)).await.get_drawn_card_list().clone();

        let deck_card_list_after_mulligan =
            game_deck_service_guard.get_deck(
                mulligan_request_form
                    .to_get_deck_request()).await.get_deck_card_list().clone();

        drop(game_deck_service_guard);

        game_hand_service_guard.add_card_list_to_hand(
            mulligan_request_form
                .to_add_card_list_to_hand_request(
                    account_unique_id,
                    redrawn_card_list.clone())).await;

        drop(game_hand_service_guard);

        MulliganResponseForm::new(
            redrawn_card_list.clone(), deck_card_list_after_mulligan)
    }
}