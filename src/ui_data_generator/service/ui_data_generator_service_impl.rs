use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::ui_data_generator::repository::ui_data_generator_repository::UiDataGeneratorRepository;
use crate::ui_data_generator::repository::ui_data_generator_repository_impl::UiDataGeneratorRepositoryImpl;
use crate::ui_data_generator::service::request::generate_my_specific_unit_energy_data_request::{GenerateMySpecificUnitEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_my_field_energy_data_request::{GenerateMyFieldEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_death_data_request::{GenerateOpponentSpecificUnitDeathDataRequest};
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_deck_card_list_data_request::{GenerateUseMyDeckCardListDataRequest};
use crate::ui_data_generator::service::request::generate_draw_my_deck_data_request::{GenerateDrawMyDeckDataRequest};
use crate::ui_data_generator::service::request::generate_draw_opponent_deck_data_request::GenerateDrawOpponentDeckDataRequest;
use crate::ui_data_generator::service::request::generate_my_main_character_health_point_data_request::GenerateMyMainCharacterHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_my_main_character_survival_data_request::GenerateMyMainCharacterSurvivalDataRequest;
use crate::ui_data_generator::service::request::generate_my_multiple_unit_death_data_request::GenerateMyMultipleUnitDeathDataRequest;
use crate::ui_data_generator::service::request::generate_my_multiple_unit_extra_effect_data_request::GenerateMyMultipleUnitExtraEffectDataRequest;
use crate::ui_data_generator::service::request::generate_my_multiple_unit_harmful_effect_data_request::GenerateMyMultipleUnitHarmfulEffectDataRequest;
use crate::ui_data_generator::service::request::generate_my_multiple_unit_health_point_data_request::GenerateMyMultipleUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_active_skill_use_data_request::GenerateMySpecificUnitActiveSkillUseDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_basic_attack_data_request::GenerateMySpecificUnitBasicAttackDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_death_data_request::GenerateMySpecificUnitDeathDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_extra_effect_data_request::GenerateMySpecificUnitExtraEffectDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_harmful_effect_data_request::GenerateMySpecificUnitHarmfulEffectDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_health_point_data_request::GenerateMySpecificUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_passive_skill_use_data_request::GenerateMySpecificUnitPassiveSkillUseDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_deck_card_lost_data_request::GenerateOpponentDeckCardLostDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_field_energy_data_request::{GenerateOpponentFieldEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_opponent_main_character_health_point_data_request::GenerateOpponentMainCharacterHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_main_character_survival_data_request::GenerateOpponentMainCharacterSurvivalDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_multiple_unit_death_data_request::GenerateOpponentMultipleUnitDeathDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_multiple_unit_extra_effect_data_request::GenerateOpponentMultipleUnitExtraEffectDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_multiple_unit_harmful_effect_data_request::GenerateOpponentMultipleUnitHarmfulEffectDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_multiple_unit_health_point_data_request::GenerateOpponentMultipleUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_energy_data_request::GenerateOpponentSpecificUnitEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_extra_effect_data_request::GenerateOpponentSpecificUnitExtraEffectDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_harmful_effect_data_request::GenerateOpponentSpecificUnitHarmfulEffectDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_health_point_data_request::GenerateOpponentSpecificUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_search_my_deck_data_request::{GenerateSearchMyDeckDataRequest};
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::{GenerateMySpecificUnitEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_my_field_energy_data_response::{GenerateMyFieldEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_death_data_response::{GenerateOpponentSpecificUnitDeathDataResponse};
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_deck_card_list_data_response::{GenerateUseMyDeckCardListDataResponse};
use crate::ui_data_generator::service::response::generate_draw_my_deck_data_response::{GenerateDrawMyDeckDataResponse};
use crate::ui_data_generator::service::response::generate_draw_opponent_deck_data_response::GenerateDrawOpponentDeckDataResponse;
use crate::ui_data_generator::service::response::generate_my_main_character_health_point_data_response::GenerateMyMainCharacterHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_my_main_character_survival_data_response::GenerateMyMainCharacterSurvivalDataResponse;
use crate::ui_data_generator::service::response::generate_my_multiple_unit_death_data_response::GenerateMyMultipleUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_my_multiple_unit_extra_effect_data_response::GenerateMyMultipleUnitExtraEffectDataResponse;
use crate::ui_data_generator::service::response::generate_my_multiple_unit_harmful_effect_data_response::GenerateMyMultipleUnitHarmfulEffectDataResponse;
use crate::ui_data_generator::service::response::generate_my_multiple_unit_health_point_data_response::GenerateMyMultipleUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_active_skill_use_data_response::GenerateMySpecificUnitActiveSkillUseDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_basic_attack_data_response::GenerateMySpecificUnitBasicAttackDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_death_data_response::GenerateMySpecificUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_extra_effect_data_response::GenerateMySpecificUnitExtraEffectDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_harmful_effect_data_response::GenerateMySpecificUnitHarmfulEffectDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_health_point_data_response::GenerateMySpecificUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_passive_skill_use_data_response::GenerateMySpecificUnitPassiveSkillUseDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_deck_card_lost_data_response::GenerateOpponentDeckCardLostDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_field_energy_data_response::{GenerateOpponentFieldEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_opponent_main_character_health_point_data_response::GenerateOpponentMainCharacterHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_main_character_survival_data_response::GenerateOpponentMainCharacterSurvivalDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_death_data_response::GenerateOpponentMultipleUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_extra_effect_data_response::GenerateOpponentMultipleUnitExtraEffectDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_harmful_effect_data_response::GenerateOpponentMultipleUnitHarmfulEffectDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_health_point_data_response::GenerateOpponentMultipleUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_energy_data_response::GenerateOpponentSpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_extra_effect_data_response::GenerateOpponentSpecificUnitExtraEffectDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_harmful_effect_data_response::GenerateOpponentSpecificUnitHarmfulEffectDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_health_point_data_response::GenerateOpponentSpecificUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_search_my_deck_data_response::{GenerateSearchMyDeckDataResponse};
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

    async fn generate_my_specific_unit_health_point_data(
        &mut self, generate_my_specific_unit_health_point_data_request: GenerateMySpecificUnitHealthPointDataRequest)
        -> GenerateMySpecificUnitHealthPointDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_health_point_data()");

        let my_unit_index =
            generate_my_specific_unit_health_point_data_request.get_my_unit_index();
        let my_unit_updated_health_point =
            generate_my_specific_unit_health_point_data_request.get_my_unit_updated_health_point();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_health_point_data(
                my_unit_index,
                my_unit_updated_health_point).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitHealthPointDataResponse::new(
            info_tuple.0.get_player_field_unit_health_point_map().clone(),
            info_tuple.1.get_player_field_unit_health_point_map().clone())
    }

    async fn generate_my_main_character_health_point_data(
        &mut self, generate_my_main_character_health_point_data_request: GenerateMyMainCharacterHealthPointDataRequest)
        -> GenerateMyMainCharacterHealthPointDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_my_main_character_health_point_data()");

        let my_main_character_updated_health_point =
            generate_my_main_character_health_point_data_request.get_main_character_updated_health_point();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_main_character_health_point_data(
                my_main_character_updated_health_point).await;

        drop(ui_data_generator_repository_guard);

        GenerateMyMainCharacterHealthPointDataResponse::new(
            info_tuple.0.get_player_main_character_health_point_map().clone(),
            info_tuple.1.get_player_main_character_health_point_map().clone())
    }

    async fn generate_opponent_main_character_health_point_data(
        &mut self, generate_opponent_main_character_health_point_data_request: GenerateOpponentMainCharacterHealthPointDataRequest)
        -> GenerateOpponentMainCharacterHealthPointDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_opponent_main_character_health_point_data()");

        let opponent_main_character_updated_health_point =
            generate_opponent_main_character_health_point_data_request.get_main_character_updated_health_point();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_main_character_health_point_data(
                opponent_main_character_updated_health_point).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentMainCharacterHealthPointDataResponse::new(
            info_tuple.0.get_player_main_character_health_point_map().clone(),
            info_tuple.1.get_player_main_character_health_point_map().clone())
    }

    async fn generate_my_multiple_unit_health_point_data(
        &mut self, generate_my_multiple_unit_health_point_data_request: GenerateMyMultipleUnitHealthPointDataRequest)
        -> GenerateMyMultipleUnitHealthPointDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_multiple_unit_health_point_data()");

        let my_unit_health_point_tuple_list =
            generate_my_multiple_unit_health_point_data_request
                .get_my_unit_health_point_tuple_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_multiple_unit_health_point_data(
                my_unit_health_point_tuple_list).await;

        drop(ui_data_generator_repository_guard);

        GenerateMyMultipleUnitHealthPointDataResponse::new(
            info_tuple.0.get_player_field_unit_health_point_map().clone(),
            info_tuple.1.get_player_field_unit_health_point_map().clone())
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
    async fn generate_opponent_specific_unit_energy_data(
        &mut self,
        generate_opponent_specific_unit_energy_data_request: GenerateOpponentSpecificUnitEnergyDataRequest)
        -> GenerateOpponentSpecificUnitEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_specific_unit_energy_data()");

        let unit_index =
            generate_opponent_specific_unit_energy_data_request.get_unit_index();
        let updated_unit_energy_map =
            generate_opponent_specific_unit_energy_data_request.get_updated_unit_energy_map();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_specific_unit_energy_data(
                unit_index,
                updated_unit_energy_map.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentSpecificUnitEnergyDataResponse::new(
            info_tuple.0.get_player_field_unit_energy_map().clone(),
            info_tuple.1.get_player_field_unit_energy_map().clone())
    }


    async fn generate_my_field_energy_data(
        &mut self,
        generate_my_field_energy_data_request: GenerateMyFieldEnergyDataRequest)
        -> GenerateMyFieldEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_field_energy_data()");

        let remaining_field_energy =
            generate_my_field_energy_data_request.get_remaining_field_energy();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_my_field_energy_data(
                remaining_field_energy).await;

        drop(ui_data_generator_repository_guard);

        GenerateMyFieldEnergyDataResponse::new(
            info_tuple.0.get_player_field_energy_map().clone(),
            info_tuple.1.get_player_field_energy_map().clone())
    }

    async fn generate_use_my_deck_card_list_data(
        &mut self,
        generate_use_my_deck_card_list_data_request: GenerateUseMyDeckCardListDataRequest)
        -> GenerateUseMyDeckCardListDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_my_deck_card_list_data()");

        let deck_card_id_list =
            generate_use_my_deck_card_list_data_request.get_deck_card_id_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_my_deck_card_list_data(
                deck_card_id_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseMyDeckCardListDataResponse::new(
            info_tuple.0.get_player_deck_card_use_list_map().clone(),
            info_tuple.1.get_player_deck_card_use_list_map().clone())
    }

    async fn generate_draw_my_deck_data(
        &mut self,
        generate_draw_my_deck_data_request: GenerateDrawMyDeckDataRequest)
        -> GenerateDrawMyDeckDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_draw_my_deck_data()");

        let drawn_card_list =
            generate_draw_my_deck_data_request.get_drawn_card_list().clone();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_draw_my_deck_data(
                drawn_card_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateDrawMyDeckDataResponse::new(
            info_tuple.0.get_player_drawn_card_list_map().clone(),
            info_tuple.1.get_player_draw_count_map().clone())
    }

    async fn generate_draw_opponent_deck_data(
        &mut self,
        generate_draw_opponent_deck_data_request: GenerateDrawOpponentDeckDataRequest)
        -> GenerateDrawOpponentDeckDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_draw_opponent_deck_data()");

        let drawn_card_list =
            generate_draw_opponent_deck_data_request.get_drawn_card_list().clone();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_draw_opponent_deck_data(
                drawn_card_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateDrawOpponentDeckDataResponse::new(
            info_tuple.0.get_player_draw_count_map().clone(),
            info_tuple.1.get_player_drawn_card_list_map().clone()
            )
    }

    async fn generate_search_my_deck_data(
        &mut self,
        generate_search_my_deck_data_request: GenerateSearchMyDeckDataRequest)
        -> GenerateSearchMyDeckDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_search_my_deck_data()");

        let found_card_list =
            generate_search_my_deck_data_request.get_found_card_list().clone();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_search_my_deck_data(
                found_card_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateSearchMyDeckDataResponse::new(
            info_tuple.0.get_player_search_card_list_map().clone(),
            info_tuple.1.get_player_search_count_map().clone())
    }

    async fn generate_opponent_field_energy_data(
        &mut self,
        generate_opponent_field_energy_data_request: GenerateOpponentFieldEnergyDataRequest)
        -> GenerateOpponentFieldEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_field_energy_data()");

        let remaining_field_energy =
            generate_opponent_field_energy_data_request.get_remaining_field_energy();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_field_energy_data(
                remaining_field_energy).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentFieldEnergyDataResponse::new(
            info_tuple.0.get_player_field_energy_map().clone(),
            info_tuple.1.get_player_field_energy_map().clone())
    }

    async fn generate_opponent_specific_unit_death_data(
        &mut self,
        generate_opponent_specific_unit_death_data_request: GenerateOpponentSpecificUnitDeathDataRequest)
        -> GenerateOpponentSpecificUnitDeathDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_specific_unit_death_data()");

        let opponent_dead_unit_index =
            generate_opponent_specific_unit_death_data_request.get_dead_unit_index();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_specific_unit_death_data(
                opponent_dead_unit_index).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentSpecificUnitDeathDataResponse::new(
            info_tuple.0.get_player_field_unit_death_map().clone(),
            info_tuple.1.get_player_field_unit_death_map().clone())
    }

    async fn generate_opponent_multiple_unit_death_data(
        &mut self,
        generate_opponent_multiple_unit_death_data_request: GenerateOpponentMultipleUnitDeathDataRequest)
        -> GenerateOpponentMultipleUnitDeathDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_multiple_unit_death_data()");

        let opponent_dead_unit_index_list =
            generate_opponent_multiple_unit_death_data_request.get_opponent_dead_unit_index_list().clone();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_multiple_unit_death_data(
                opponent_dead_unit_index_list).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentMultipleUnitDeathDataResponse::new(
            info_tuple.0.get_player_field_unit_death_map().clone(),
            info_tuple.1.get_player_field_unit_death_map().clone())
    }
    async fn generate_my_multiple_unit_death_data(
        &mut self,
        generate_my_multiple_unit_death_data_request: GenerateMyMultipleUnitDeathDataRequest)
        -> GenerateMyMultipleUnitDeathDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_my_multiple_unit_death_data()");

        let my_dead_unit_index_list =
            generate_my_multiple_unit_death_data_request.get_my_dead_unit_index_list().clone();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_multiple_unit_death_data(
                my_dead_unit_index_list).await;

        drop(ui_data_generator_repository_guard);

        GenerateMyMultipleUnitDeathDataResponse::new(
            info_tuple.0.get_player_field_unit_death_map().clone(),
            info_tuple.1.get_player_field_unit_death_map().clone())
    }

    async fn generate_my_specific_unit_death_data(
        &mut self,
        generate_my_specific_unit_death_data_request: GenerateMySpecificUnitDeathDataRequest)
        -> GenerateMySpecificUnitDeathDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_death_data()");

        let my_dead_unit_index =
            generate_my_specific_unit_death_data_request.get_dead_unit_index();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_death_data(
                my_dead_unit_index).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitDeathDataResponse::new(
            info_tuple.0.get_player_field_unit_death_map().clone(),
            info_tuple.1.get_player_field_unit_death_map().clone())
    }

    async fn generate_opponent_specific_unit_health_point_data(
        &mut self,
        generate_opponent_specific_unit_health_point_data_request: GenerateOpponentSpecificUnitHealthPointDataRequest)
        -> GenerateOpponentSpecificUnitHealthPointDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_specific_unit_health_point_data()");

        let opponent_unit_index =
            generate_opponent_specific_unit_health_point_data_request.get_opponent_unit_index();
        let opponent_unit_updated_health_point =
            generate_opponent_specific_unit_health_point_data_request.get_opponent_unit_updated_health_point();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_specific_unit_health_point_data(
                opponent_unit_index,
                opponent_unit_updated_health_point).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentSpecificUnitHealthPointDataResponse::new(
            info_tuple.0.get_player_field_unit_health_point_map().clone(),
            info_tuple.1.get_player_field_unit_health_point_map().clone())
    }

    async fn generate_opponent_multiple_unit_health_point_data(
        &mut self, generate_opponent_multiple_unit_health_point_data_request: GenerateOpponentMultipleUnitHealthPointDataRequest)
        -> GenerateOpponentMultipleUnitHealthPointDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_specific_unit_health_point_data()");

        let opponent_unit_health_point_tuple_list =
            generate_opponent_multiple_unit_health_point_data_request
                .get_opponent_unit_health_point_tuple_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_multiple_unit_health_point_data(
                opponent_unit_health_point_tuple_list).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentMultipleUnitHealthPointDataResponse::new(
            info_tuple.0.get_player_field_unit_health_point_map().clone(),
            info_tuple.1.get_player_field_unit_health_point_map().clone())
    }

    async fn generate_my_main_character_survival_data(
        &mut self, generate_my_main_character_survival_data_request: GenerateMyMainCharacterSurvivalDataRequest)
        -> GenerateMyMainCharacterSurvivalDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_main_character_survival_data()");

        let my_main_character_status =
            generate_my_main_character_survival_data_request.get_main_character_status();


        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_main_character_survival_data(
                my_main_character_status.clone(),
                ).await;

        drop(ui_data_generator_repository_guard);

        GenerateMyMainCharacterSurvivalDataResponse::new(
            info_tuple.0.get_player_main_character_survival_map().clone(),
            info_tuple.1.get_player_main_character_survival_map().clone())
    }

    async fn generate_opponent_main_character_survival_data(
        &mut self, generate_opponent_main_character_survival_data_request: GenerateOpponentMainCharacterSurvivalDataRequest)
        -> GenerateOpponentMainCharacterSurvivalDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_main_character_survival_data()");

        let opponent_main_character_status =
            generate_opponent_main_character_survival_data_request.get_main_character_status();


        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_main_character_survival_data(
                opponent_main_character_status.clone(),
            ).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentMainCharacterSurvivalDataResponse::new(
            info_tuple.0.get_player_main_character_survival_map().clone(),
            info_tuple.1.get_player_main_character_survival_map().clone())
    }

    async fn generate_my_specific_unit_extra_effect_data(
        &mut self, generate_my_specific_unit_extra_effect_data_request: GenerateMySpecificUnitExtraEffectDataRequest)
        -> GenerateMySpecificUnitExtraEffectDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_extra_effect_data()");

        let my_unit_index =
            generate_my_specific_unit_extra_effect_data_request.get_my_unit_index();
        let my_unit_extra_effect_list=
            generate_my_specific_unit_extra_effect_data_request.get_my_unit_extra_effect_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_extra_effect_data(
                my_unit_index,
                my_unit_extra_effect_list.clone()
            ).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitExtraEffectDataResponse::new(
            info_tuple.0.get_player_field_unit_extra_effect_map().clone(),
            info_tuple.1.get_player_field_unit_extra_effect_map().clone())
    }
    async fn generate_opponent_specific_unit_extra_effect_data(
        &mut self, generate_opponent_specific_unit_extra_effect_data_request: GenerateOpponentSpecificUnitExtraEffectDataRequest)
        -> GenerateOpponentSpecificUnitExtraEffectDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_opponent_specific_unit_extra_effect_data()");

        let opponent_unit_index =
            generate_opponent_specific_unit_extra_effect_data_request.get_opponent_unit_index();
        let opponent_unit_extra_effect_list=
            generate_opponent_specific_unit_extra_effect_data_request.get_opponent_unit_extra_effect_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_specific_unit_extra_effect_data(
                opponent_unit_index,
                opponent_unit_extra_effect_list.clone()
            ).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentSpecificUnitExtraEffectDataResponse::new(
            info_tuple.0.get_player_field_unit_extra_effect_map().clone(),
            info_tuple.1.get_player_field_unit_extra_effect_map().clone())
    }
    async fn generate_my_specific_unit_harmful_effect_data(
        &mut self, generate_my_specific_unit_harmful_effect_data_request: GenerateMySpecificUnitHarmfulEffectDataRequest)
        -> GenerateMySpecificUnitHarmfulEffectDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_harmful_effect_data()");

        let my_unit_index =
            generate_my_specific_unit_harmful_effect_data_request.get_my_unit_index();
        let my_unit_extra_effect_list=
            generate_my_specific_unit_harmful_effect_data_request.get_harmful_status_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_harmful_effect_data(
                my_unit_index,
                my_unit_extra_effect_list.clone()
            ).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitHarmfulEffectDataResponse::new(
            info_tuple.0.get_player_field_unit_harmful_effect_map().clone(),
            info_tuple.1.get_player_field_unit_harmful_effect_map().clone())

    }
    async fn generate_opponent_specific_unit_harmful_effect_data(
        &mut self, generate_opponent_specific_unit_harmful_effect_data_request: GenerateOpponentSpecificUnitHarmfulEffectDataRequest)
        -> GenerateOpponentSpecificUnitHarmfulEffectDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_opponent_specific_unit_harmful_effect_data()");

        let opponent_unit_index =
            generate_opponent_specific_unit_harmful_effect_data_request.get_opponent_unit_index();
        let opponent_unit_extra_effect_list=
            generate_opponent_specific_unit_harmful_effect_data_request.get_opponent_unit_harmful_status_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_specific_unit_harmful_effect_data(
                opponent_unit_index,
                opponent_unit_extra_effect_list.clone()
            ).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentSpecificUnitHarmfulEffectDataResponse::new(
            info_tuple.0.get_player_field_unit_harmful_effect_map().clone(),
            info_tuple.1.get_player_field_unit_harmful_effect_map().clone())
    }

    async fn generate_my_multiple_unit_extra_effect_data(
        &mut self, generate_my_multiple_unit_extra_effect_data_request: GenerateMyMultipleUnitExtraEffectDataRequest)
        -> GenerateMyMultipleUnitExtraEffectDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_multiple_unit_extra_effect_data()");

        let my_unit_extra_effect_tuple_list =
            generate_my_multiple_unit_extra_effect_data_request
                .get_my_unit_extra_effect_tuple_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_multiple_unit_extra_effect_data(
                my_unit_extra_effect_tuple_list).await;

        drop(ui_data_generator_repository_guard);

        GenerateMyMultipleUnitExtraEffectDataResponse::new(
            info_tuple.0.get_player_field_unit_extra_effect_map().clone(),
            info_tuple.1.get_player_field_unit_extra_effect_map().clone())
    }
    async fn generate_opponent_multiple_unit_extra_effect_data(
        &mut self, generate_opponent_multiple_unit_extra_effect_data_request: GenerateOpponentMultipleUnitExtraEffectDataRequest)
        -> GenerateOpponentMultipleUnitExtraEffectDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_opponent_multiple_unit_extra_effect_data()");

        let opponent_unit_extra_effect_tuple_list =
            generate_opponent_multiple_unit_extra_effect_data_request
                .get_opponent_unit_extra_effect_tuple_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_multiple_unit_extra_effect_data(
                opponent_unit_extra_effect_tuple_list).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentMultipleUnitExtraEffectDataResponse::new(
            info_tuple.0.get_player_field_unit_extra_effect_map().clone(),
            info_tuple.1.get_player_field_unit_extra_effect_map().clone())
    }
    async fn generate_my_multiple_unit_harmful_effect_data(
        &mut self, generate_my_multiple_unit_harmful_effect_data_request: GenerateMyMultipleUnitHarmfulEffectDataRequest)
        -> GenerateMyMultipleUnitHarmfulEffectDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_my_multiple_unit_harmful_effect_data()");

        let my_unit_harmful_status_tuple_list =
            generate_my_multiple_unit_harmful_effect_data_request
                .get_my_unit_harmful_status_tuple_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_multiple_unit_harmful_effect_data(
                my_unit_harmful_status_tuple_list).await;

        drop(ui_data_generator_repository_guard);

        GenerateMyMultipleUnitHarmfulEffectDataResponse::new(
            info_tuple.0.get_player_field_unit_harmful_effect_map().clone(),
            info_tuple.1.get_player_field_unit_harmful_effect_map().clone())
    }
    async fn generate_opponent_multiple_unit_harmful_effect_data(
        &mut self, generate_opponent_multiple_unit_harmful_effect_data_request: GenerateOpponentMultipleUnitHarmfulEffectDataRequest)
        -> GenerateOpponentMultipleUnitHarmfulEffectDataResponse {
        println!("UiDataGeneratorServiceImpl: generate_opponent_multiple_unit_harmful_effect_data()");

        let opponent_unit_harmful_status_tuple_list =
            generate_opponent_multiple_unit_harmful_effect_data_request
                .get_opponent_unit_harmful_status_tuple_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_multiple_unit_harmful_effect_data(
                opponent_unit_harmful_status_tuple_list).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentMultipleUnitHarmfulEffectDataResponse::new(
            info_tuple.0.get_player_field_unit_harmful_effect_map().clone(),
            info_tuple.1.get_player_field_unit_harmful_effect_map().clone())
    }

    async fn generate_opponent_deck_card_lost_data(
        &mut self, generate_opponent_deck_card_lost_data_request: GenerateOpponentDeckCardLostDataRequest)
        -> GenerateOpponentDeckCardLostDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_deck_card_lost_data()");

        let opponent_lost_deck_card_list =
            generate_opponent_deck_card_lost_data_request.get_lost_deck_card_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_deck_card_lost_data(
                opponent_lost_deck_card_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentDeckCardLostDataResponse::new(
            info_tuple.0.get_player_deck_card_lost_list_map().clone(),
            info_tuple.1.get_player_deck_card_lost_list_map().clone())
    }

    async fn generate_my_specific_unit_basic_attack_data(
        &mut self, generate_my_specific_unit_basic_attack_data_request: GenerateMySpecificUnitBasicAttackDataRequest)
        -> GenerateMySpecificUnitBasicAttackDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_basic_attack_data()");

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_basic_attack_data(
                generate_my_specific_unit_basic_attack_data_request.get_attacker_unit_index(),
                generate_my_specific_unit_basic_attack_data_request.get_target_unit_index()).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitBasicAttackDataResponse::new(
            info_tuple.0.get_player_field_unit_attack_map().clone(),
            info_tuple.1.get_player_field_unit_attack_map().clone())
    }

    async fn generate_my_specific_unit_active_skill_use_data(
        &mut self, generate_my_specific_unit_active_skill_use_data_request: GenerateMySpecificUnitActiveSkillUseDataRequest)
        -> GenerateMySpecificUnitActiveSkillUseDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_active_skill_use_data()");

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_active_skill_use_data(
                generate_my_specific_unit_active_skill_use_data_request.get_attacker_unit_index(),
                generate_my_specific_unit_active_skill_use_data_request.get_target_unit_index(),
                generate_my_specific_unit_active_skill_use_data_request.get_active_skill_index()).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitActiveSkillUseDataResponse::new(
            info_tuple.0.get_player_field_unit_attack_map().clone(),
            info_tuple.1.get_player_field_unit_attack_map().clone())
    }
    async fn generate_my_specific_unit_passive_skill_use_data(
        &mut self, generate_my_specific_unit_passive_skill_use_data_request: GenerateMySpecificUnitPassiveSkillUseDataRequest)
        -> GenerateMySpecificUnitPassiveSkillUseDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_passive_skill_use_data()");

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_passive_skill_use_data(
                generate_my_specific_unit_passive_skill_use_data_request.get_attacker_unit_index(),
                generate_my_specific_unit_passive_skill_use_data_request.get_target_unit_index(),
                generate_my_specific_unit_passive_skill_use_data_request.get_active_skill_index()).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitPassiveSkillUseDataResponse::new(
            info_tuple.0.get_player_field_unit_attack_map().clone(),
            info_tuple.1.get_player_field_unit_attack_map().clone())
    }
}