use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_support::controller::game_card_support_controller::GameCardSupportController;
use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;

use crate::game_card_support::service::game_card_support_service_impl::GameCardSupportServiceImpl;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;

pub struct GameCardSupportControllerImpl {
    game_card_support_service: Arc<AsyncMutex<GameCardSupportServiceImpl>>,
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    battle_field: Arc<AsyncMutex<GameFie>>,
}

impl GameCardSupportControllerImpl {
    pub fn new(game_card_support_service: Arc<AsyncMutex<GameCardSupportServiceImpl>>,
               game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameCardSupportControllerImpl>>) -> Self {

        GameCardSupportControllerImpl {
            game_card_support_service,
            game_hand_service,
            game_deck_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardSupportController for GameCardSupportControllerImpl {
    async fn request_to_use_energy_boost_support(&self, energy_boost_support_request_form: EnergyBoostSupportRequestForm) -> EnergyBoostSupportResponseForm {
        println!("GameCardSupportControllerImpl: request_to_use_energy_boost_support()");
    }
}