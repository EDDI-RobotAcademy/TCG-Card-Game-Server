use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::ui_data_generator::repository::ui_data_generator_repository::UiDataGeneratorRepository;
use crate::ui_data_generator::repository::ui_data_generator_repository_impl::UiDataGeneratorRepositoryImpl;
use crate::ui_data_generator::service::request::generate_my_specific_unit_energy_data_request::{GenerateMySpecificUnitEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_use_my_field_energy_data_request::{GenerateUseMyFieldEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_instant_death_of_your_specific_unit_data_request::{GenerateInstantDeathOfYourSpecificUnitDataRequest};
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request::{GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataRequest};
use crate::ui_data_generator::service::request::generate_use_support_card_to_draw_my_deck_data_request::{GenerateUseSupportCardToDrawMyDeckDataRequest};
use crate::ui_data_generator::service::request::generate_use_support_card_to_remove_your_field_energy_data_request::GenerateUseSupportCardToRemoveYourFieldEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_search_unit_from_my_deck_data_request::{GenerateUseSupportCardToSearchUnitFromMyDeckDataRequest};
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::{GenerateMySpecificUnitEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_use_my_field_energy_data_response::{GenerateUseMyFieldEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_instant_death_of_your_specific_unit_data_response::{GenerateInstantDeathOfYourSpecificUnitDataResponse};
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_boost_energy_to_my_specific_unit_data_response::{GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataResponse};
use crate::ui_data_generator::service::response::generate_use_support_card_to_draw_my_deck_data_response::{GenerateUseSupportCardToDrawMyDeckDataResponse};
use crate::ui_data_generator::service::response::generate_use_support_card_to_remove_your_field_energy_data_response::GenerateUseSupportCardToRemoveYourFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_search_unit_from_my_deck_data_response::{GenerateUseSupportCardToSearchUnitFromMyDeckDataResponse};
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;

pub struct UiDataGeneratorServiceImpl {
    ui_data_generator_repository: Arc<AsyncMutex<UiDataGeneratorRepositoryImpl>>,
    card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
}

impl UiDataGeneratorServiceImpl {
    pub fn new(
        ui_data_generator_repository: Arc<AsyncMutex<UiDataGeneratorRepositoryImpl>>,
        card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    ) -> Self {

        UiDataGeneratorServiceImpl {
            ui_data_generator_repository,
            card_kind_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<UiDataGeneratorServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<UiDataGeneratorServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        UiDataGeneratorServiceImpl::new(
                            UiDataGeneratorRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl UiDataGeneratorService for UiDataGeneratorServiceImpl {
    async fn generate_use_my_hand_card_data(
        &mut self,
        generate_use_my_hand_card_data_request: GenerateUseMyHandCardDataRequest)
        -> GenerateUseMyHandCardDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_my_hand_card_data()");

        let used_hand_card_id =
            generate_use_my_hand_card_data_request.get_used_hand_card_id();

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(&used_hand_card_id).await;

        drop(card_kind_repository_guard);

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_my_hand_card_data(
                used_hand_card_id,
                hand_card_kind_enum).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseMyHandCardDataResponse::new(
            info_tuple.0,
            info_tuple.1.get_player_hand_card_use_map().clone())
    }
    async fn generate_my_specific_unit_energy_data(
        &mut self,
        generate_my_specific_unit_energy_data_request: GenerateMySpecificUnitEnergyDataRequest)
        -> GenerateMySpecificUnitEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_energy_data()");

        let unit_index =
            generate_my_specific_unit_energy_data_request.get_unit_index();
        let updated_unit_energy_map =
            generate_my_specific_unit_energy_data_request.get_updated_unit_energy_map();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_energy_data(
                unit_index,
                updated_unit_energy_map.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitEnergyDataResponse::new(
            info_tuple.0.get_player_field_unit_energy_map().clone(),
            info_tuple.1.get_player_field_unit_energy_map().clone())
    }

    async fn generate_use_my_field_energy_data(
        &mut self,
        generate_use_my_field_energy_data_request: GenerateUseMyFieldEnergyDataRequest)
        -> GenerateUseMyFieldEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_field_energy_to_my_specific_unit_data()");

        let remaining_field_energy =
            generate_use_my_field_energy_data_request.get_remaining_field_energy();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_my_field_energy_data(
                remaining_field_energy).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseMyFieldEnergyDataResponse::new(
            info_tuple.0.get_player_field_energy_map().clone(),
            info_tuple.1.get_player_field_energy_map().clone())
    }

    async fn generate_use_support_card_to_boost_energy_to_my_specific_unit_data(
        &mut self,
        generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request: GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataRequest)
        -> GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_support_card_to_boost_energy_to_my_specific_unit()");

        let used_hand_card_id =
            generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request.get_used_hand_card_id();
        let found_energy_card_id_list =
            generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request.get_found_energy_card_id_list();
        let unit_index =
            generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request.get_unit_index();
        let updated_unit_energy_map =
            generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request.get_updated_attached_energy_map();

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(&used_hand_card_id).await;

        drop(card_kind_repository_guard);

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_support_card_to_boost_energy_to_my_specific_unit_data(
                used_hand_card_id,
                hand_card_kind_enum,
                found_energy_card_id_list.clone(),
                unit_index,
                updated_unit_energy_map.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataResponse::new(
            info_tuple.0.get_player_deck_card_use_list_map().clone(),
            info_tuple.1.get_player_field_unit_energy_map().clone(),
            info_tuple.2.get_player_hand_card_use_map().clone(),
            info_tuple.3.get_player_deck_card_use_list_map().clone(),
            info_tuple.4.get_player_field_unit_energy_map().clone(),)
    }

    async fn generate_use_support_card_to_draw_my_deck_data(
        &mut self,
        generate_use_support_card_to_draw_my_deck_data_request: GenerateUseSupportCardToDrawMyDeckDataRequest)
        -> GenerateUseSupportCardToDrawMyDeckDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_support_card_to_draw_my_deck_data()");

        let used_hand_card_id =
            generate_use_support_card_to_draw_my_deck_data_request.get_used_hand_card_id();
        let drawn_card_list =
            generate_use_support_card_to_draw_my_deck_data_request.get_drawn_card_list().clone();

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(&used_hand_card_id).await;

        drop(card_kind_repository_guard);

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_support_card_to_draw_my_deck_data(
                used_hand_card_id,
                hand_card_kind_enum,
                drawn_card_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseSupportCardToDrawMyDeckDataResponse::new(
            info_tuple.0.get_player_drawn_card_list_map().clone(),
            info_tuple.1.get_player_hand_card_use_map().clone(),
            info_tuple.2.get_player_draw_count_map().clone())
    }

    async fn generate_use_support_card_to_search_unit_from_my_deck_data(
        &mut self,
        generate_use_support_card_to_search_unit_from_my_deck_data_request: GenerateUseSupportCardToSearchUnitFromMyDeckDataRequest)
        -> GenerateUseSupportCardToSearchUnitFromMyDeckDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_support_card_to_search_unit_from_my_deck_data()");

        let used_hand_card_id =
            generate_use_support_card_to_search_unit_from_my_deck_data_request.get_used_hand_card_id();
        let found_card_list =
            generate_use_support_card_to_search_unit_from_my_deck_data_request.get_found_card_list().clone();

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(&used_hand_card_id).await;

        drop(card_kind_repository_guard);

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_support_card_to_search_unit_from_my_deck_data(
                used_hand_card_id,
                hand_card_kind_enum,
                found_card_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseSupportCardToSearchUnitFromMyDeckDataResponse::new(
            info_tuple.0.get_player_search_card_list_map().clone(),
            info_tuple.1.get_player_hand_card_use_map().clone(),
            info_tuple.2.get_player_search_count_map().clone())
    }

    async fn generate_use_support_card_to_remove_your_field_energy_data(
        &mut self,
        generate_use_support_card_to_remove_your_field_energy_data_request: GenerateUseSupportCardToRemoveYourFieldEnergyDataRequest)
        -> GenerateUseSupportCardToRemoveYourFieldEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_support_card_to_remove_your_field_energy_data()");

        let used_hand_card_id =
            generate_use_support_card_to_remove_your_field_energy_data_request.get_used_hand_card_id();
        let remaining_field_energy_of_opponent =
            generate_use_support_card_to_remove_your_field_energy_data_request.get_remaining_field_energy();

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(&used_hand_card_id).await;

        drop(card_kind_repository_guard);

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_support_card_to_remove_your_field_energy_data(
                used_hand_card_id,
                hand_card_kind_enum,
                remaining_field_energy_of_opponent).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseSupportCardToRemoveYourFieldEnergyDataResponse::new(
            info_tuple.0.get_player_field_energy_map().clone(),
            info_tuple.1.get_player_hand_card_use_map().clone(),
            info_tuple.2.get_player_field_energy_map().clone())
    }

    async fn generate_instant_death_of_your_specific_unit_data(
        &mut self,
        generate_instant_death_of_your_specific_unit_data_request: GenerateInstantDeathOfYourSpecificUnitDataRequest)
        -> GenerateInstantDeathOfYourSpecificUnitDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_instant_death_of_your_specific_unit_data()");

        let opponent_dead_unit_index =
            generate_instant_death_of_your_specific_unit_data_request.get_dead_unit_index();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_instant_death_of_your_specific_unit_data(
                opponent_dead_unit_index).await;

        drop(ui_data_generator_repository_guard);

        GenerateInstantDeathOfYourSpecificUnitDataResponse::new(
            info_tuple.0.get_player_field_unit_death_map().clone(),
            info_tuple.1.get_player_field_unit_death_map().clone())
    }
}