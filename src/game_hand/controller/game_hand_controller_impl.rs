use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_hand::controller::game_hand_controller::GameHandController;
use crate::game_hand::controller::request_form::use_game_hand_energy_card_request_form::UseGameHandEnergyCardRequestForm;
use crate::game_hand::controller::request_form::use_game_hand_unit_card_request_form::UseGameHandUnitCardRequestForm;
use crate::game_hand::controller::response_form::use_game_hand_energy_card_response_form::UseGameHandEnergyCardResponseForm;
use crate::game_hand::controller::response_form::use_game_hand_unit_card_response_form::UseGameHandUnitCardResponseForm;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;

pub struct GameHandControllerImpl {
    service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
}

impl GameHandControllerImpl {
    pub fn new(service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>) -> Self {
        GameHandControllerImpl {
            service,
            game_deck_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameHandControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameHandControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameHandControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameHandController for GameHandControllerImpl {
    async fn use_game_hand_energy_card(
        &self, use_game_hand_energy_card_request_form: UseGameHandEnergyCardRequestForm) -> UseGameHandEnergyCardResponseForm {
        let request =
            use_game_hand_energy_card_request_form.to_use_game_hand_energy_card_request();

        let mut game_hand_service_guard = self.service.lock().await;
        let response = game_hand_service_guard.attach_energy_card_to_field_unit(request).await;

        response.to_use_game_hand_energy_card_response_form()
    }
    async fn use_game_hand_unit_card(
        &self, use_game_hand_unit_card_request_form: UseGameHandUnitCardRequestForm) -> UseGameHandUnitCardResponseForm {
        let request = use_game_hand_unit_card_request_form.to_use_game_hand_unit_card_request();

        let mut game_hand_service_guard = self.service.lock().await;
        let response = game_hand_service_guard.use_specific_card(request).await;

        response.to_use_game_hand_unit_card_response_form()
    }
}