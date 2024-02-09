use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_hand::controller::game_hand_controller::GameHandController;
use crate::game_hand::controller::request_form::mulligan_request_form::MulliganRequestForm;
use crate::game_hand::controller::response_form::mulligan_response_form::MulliganResponseForm;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;

pub struct GameHandControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    // redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
}

impl GameHandControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>) -> Self {
        GameHandControllerImpl {
            game_hand_service,
            game_deck_service,
            game_protocol_validation_service,
            // redis_in_memory_service
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameHandControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameHandControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameHandControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameHandController for GameHandControllerImpl {
    async fn execute_mulligan_procedure(&self, mulligan_request_form: MulliganRequestForm) -> MulliganResponseForm {
        println!("GameHandControllerImpl: execute_mulligan_procedure()");

        // protocol validation service
        let check_hand_card_hacking_request = mulligan_request_form.to_check_cards_from_hand_request();
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;

        let check_hand_card_hacking_response =
            game_protocol_validation_service_guard.check_cards_from_hand(check_hand_card_hacking_request).await;
        if check_hand_card_hacking_response.get_is_success() == false {
            println!("Mulligan failed by protocol validation service");
            return MulliganResponseForm::new(Vec::new(), Vec::new())
        }

        drop(game_protocol_validation_service_guard);

        // hand service
        let put_cards_on_deck_request = mulligan_request_form.to_put_cards_on_deck_request();
        let mut game_hand_service_guard = self.game_hand_service.lock().await;

        let put_cards_on_deck_response =
            game_hand_service_guard.put_hand_cards_to_deck(put_cards_on_deck_request).await;
        if put_cards_on_deck_response.get_is_success() == false {
            println!("Mulligan failed by hand service");
            return MulliganResponseForm::new(Vec::new(), Vec::new())
        }

        drop(game_hand_service_guard);

        // deck service
        let mut game_deck_service_guard = self.game_deck_service.lock().await;

        let shuffle_deck_request = mulligan_request_form.to_shuffle_deck_request();
        game_deck_service_guard.shuffle_deck(shuffle_deck_request).await;

        let draw_deck_request = mulligan_request_form.to_draw_deck_request();
        let draw_deck_response = game_deck_service_guard.draw_deck(draw_deck_request).await;
        let draw_card_list_by_mulligan = draw_deck_response.get_drawn_card_list().clone();

        let get_deck_request = mulligan_request_form.to_get_deck_request();
        let get_deck_response = game_deck_service_guard.get_deck(get_deck_request).await;
        let deck_card_list_after_mulligan = get_deck_response.get_deck_card_list().clone();

        drop(game_deck_service_guard);

        MulliganResponseForm::new(draw_card_list_by_mulligan, deck_card_list_after_mulligan)
    }
}