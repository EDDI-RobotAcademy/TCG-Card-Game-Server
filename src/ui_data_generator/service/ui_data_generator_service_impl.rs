use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::ui_data_generator::repository::ui_data_generator_repository::UiDataGeneratorRepository;
use crate::ui_data_generator::repository::ui_data_generator_repository_impl::UiDataGeneratorRepositoryImpl;
use crate::ui_data_generator::service::request::generate_use_energy_card_to_my_specific_unit_data_request::GenerateUseEnergyCardToMySpecificUnitDataRequest;
use crate::ui_data_generator::service::request::generate_use_field_energy_to_my_specific_unit_request::GenerateUseFieldEnergyToMySpecificUnitRequest;
use crate::ui_data_generator::service::response::generate_use_energy_card_to_my_specific_unit_data_response::GenerateUseEnergyCardToMySpecificUnitDataResponse;
use crate::ui_data_generator::service::response::generate_use_field_energy_to_my_specific_unit_response::GenerateUseFieldEnergyToMySpecificUnitResponse;
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
    async fn generate_use_energy_card_to_my_specific_unit_data(
        &mut self,
        generate_use_energy_card_to_my_specific_unit_data_request: GenerateUseEnergyCardToMySpecificUnitDataRequest)
        -> GenerateUseEnergyCardToMySpecificUnitDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_energy_card_to_my_specific_unit_data()");

        let used_hand_card_id =
            generate_use_energy_card_to_my_specific_unit_data_request.get_used_hand_card_id();
        let unit_index =
            generate_use_energy_card_to_my_specific_unit_data_request.get_unit_index();
        let updated_unit_energy_map =
            generate_use_energy_card_to_my_specific_unit_data_request.get_updated_unit_energy_map();

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(&used_hand_card_id).await;

        drop(card_kind_repository_guard);

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_energy_card_to_my_specific_unit_data(
                used_hand_card_id,
                hand_card_kind_enum,
                unit_index,
                updated_unit_energy_map.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseEnergyCardToMySpecificUnitDataResponse::new(
            info_tuple.0.get_player_field_unit_energy_map().clone(),
            info_tuple.1.get_player_hand_card_use_map().clone(),
            info_tuple.2.get_player_field_unit_energy_map().clone())
    }

    async fn generate_use_field_energy_to_my_specific_unit_data(
        &mut self,
        generate_use_field_energy_to_my_specific_unit_request: GenerateUseFieldEnergyToMySpecificUnitRequest)
        -> GenerateUseFieldEnergyToMySpecificUnitResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_field_energy_to_my_specific_unit_data()");

        let unit_index =
            generate_use_field_energy_to_my_specific_unit_request.get_unit_index();
        let updated_unit_energy_map =
            generate_use_field_energy_to_my_specific_unit_request.get_updated_unit_energy_map();
        let remaining_field_energy =
            generate_use_field_energy_to_my_specific_unit_request.get_remaining_field_energy();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_field_energy_to_my_specific_unit_data(
                unit_index,
                updated_unit_energy_map.clone(),
                remaining_field_energy).await;
        
        GenerateUseFieldEnergyToMySpecificUnitResponse::new(
            info_tuple.0.get_player_field_energy_map().clone(),
            info_tuple.1.get_player_field_unit_energy_map().clone(),
            info_tuple.2.get_player_field_energy_map().clone(),
            info_tuple.3.get_player_field_unit_energy_map().clone())
    }
}