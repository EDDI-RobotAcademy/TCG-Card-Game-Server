use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::game_field_energy::repository::game_field_energy_repository_impl::GameFieldEnergyRepositoryImpl;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::notify_player_action_info::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::field_unit_survival_info::FieldUnitSurvivalInfo;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::notify_player_action_info::repository::notify_player_action_info_repository_impl::NotifyPlayerActionInfoRepositoryImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_every_unit_by_using_hand_card_request::NoticeApplyDamageToEveryUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_boost_energy_to_specific_unit_request::{NoticeBoostEnergyToSpecificUnitRequest};
use crate::notify_player_action_info::service::request::notice_apply_damage_to_specific_unit_by_using_hand_card_request::{NoticeApplyDamageToSpecificUnitByUsingHandCardRequest};
use crate::notify_player_action_info::service::request::notice_attach_energy_to_specific_unit_by_using_hand_card_request::NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_draw_card_request::{NoticeDrawCardRequest};
use crate::notify_player_action_info::service::request::notice_instant_death_of_specific_unit_by_using_hand_card_request::NoticeInstantDeathOfSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_remove_energy_of_specific_unit_by_using_hand_card_request::NoticeRemoveEnergyOfSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_remove_field_energy_of_opponent_request::{NoticeRemoveFieldEnergyOfOpponentRequest};
use crate::notify_player_action_info::service::request::notice_search_card_request::{NoticeSearchCardRequest};
use crate::notify_player_action_info::service::request::notice_use_hand_card_request::NoticeUseHandCardRequest;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_every_unit_by_using_hand_card_response::NoticeApplyDamageToEveryUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_boost_energy_to_specific_unit_response::{NoticeBoostEnergyToSpecificUnitResponse};
use crate::notify_player_action_info::service::response::notice_apply_damage_to_specific_unit_by_using_hand_card_response::{NoticeApplyDamageToSpecificUnitByUsingHandCardResponse};
use crate::notify_player_action_info::service::response::notice_attach_energy_to_specific_unit_by_using_hand_card_response::NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_draw_card_response::{NoticeDrawCardResponse};
use crate::notify_player_action_info::service::response::notice_instant_death_of_specific_unit_by_using_hand_card_response::NoticeInstantDeathOfSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_remove_energy_of_specific_unit_by_using_hand_card_response::NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_remove_field_energy_of_opponent_response::{NoticeRemoveFieldEnergyOfOpponentResponse};
use crate::notify_player_action_info::service::response::notice_search_card_response::{NoticeSearchCardResponse};
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

pub struct NotifyPlayerActionInfoServiceImpl {
    notify_player_action_info_repository: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
    card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
}

impl NotifyPlayerActionInfoServiceImpl {
    pub fn new(
        notify_player_action_info_repository: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
        card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
        game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
        game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
    ) -> Self {

        NotifyPlayerActionInfoServiceImpl {
            notify_player_action_info_repository,
            card_kind_repository,
            game_field_unit_repository,
            game_field_energy_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionInfoServiceImpl::new(
                            NotifyPlayerActionInfoRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance(),
                            GameFieldEnergyRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl NotifyPlayerActionInfoService for NotifyPlayerActionInfoServiceImpl {
    async fn notice_use_hand_card(
        &mut self, notice_use_hand_card_request: NoticeUseHandCardRequest)
        -> NoticeUseHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_hand_card()");

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(
                &notice_use_hand_card_request.get_used_hand_card_id()).await;

        drop(card_kind_repository_guard);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_use_hand_card(
                notice_use_hand_card_request.get_opponent_unique_id(),
                notice_use_hand_card_request.get_used_hand_card_id(),
                hand_card_kind_enum).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseHandCardResponse::new(response)
    }

    async fn notice_boost_energy_to_specific_unit(
        &mut self,
        notice_boost_energy_to_specific_unit_request: NoticeBoostEnergyToSpecificUnitRequest)
        -> NoticeBoostEnergyToSpecificUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_boost_energy_to_specific_unit()");

        let mut field_unit_energy_map = HashMap::new();
        field_unit_energy_map.insert(
            notice_boost_energy_to_specific_unit_request.get_unit_index(),
            notice_boost_energy_to_specific_unit_request.get_updated_attached_energy_map().to_attached_energy_info());

        let field_unit_energy_info = FieldUnitEnergyInfo::new(field_unit_energy_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let player_deck_card_use_list_info =
            notify_player_action_info_repository_guard.notify_player_use_deck_card_list(
                notice_boost_energy_to_specific_unit_request.get_opponent_unique_id(),
                notice_boost_energy_to_specific_unit_request.get_found_energy_card_id_list().clone()).await;
        let player_field_unit_energy_info =
            notify_player_action_info_repository_guard.notify_player_energy_of_unit(
                notice_boost_energy_to_specific_unit_request.get_opponent_unique_id(),
                field_unit_energy_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeBoostEnergyToSpecificUnitResponse::new(
            player_deck_card_use_list_info, player_field_unit_energy_info)
    }

    async fn notice_draw_card(
        &mut self,
        notice_draw_card_request: NoticeDrawCardRequest)
        -> NoticeDrawCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_draw_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let player_drawn_card_list_info =
            notify_player_action_info_repository_guard.notify_player_draw_card(
                notice_draw_card_request.get_opponent_unique_id(),
                notice_draw_card_request.get_drawn_card_list().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeDrawCardResponse::new(player_drawn_card_list_info)
    }

    async fn notice_search_card(
        &mut self,
        notice_search_card_request: NoticeSearchCardRequest)
        -> NoticeSearchCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_search_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_search_card(
                notice_search_card_request.get_opponent_unique_id(),
                notice_search_card_request.get_found_card_list().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeSearchCardResponse::new(response)
    }

    async fn notice_remove_field_energy_of_opponent(
        &mut self,
        notice_remove_field_energy_of_opponent_request: NoticeRemoveFieldEnergyOfOpponentRequest)
        -> NoticeRemoveFieldEnergyOfOpponentResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_remove_field_energy_of_opponent()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_opponent_field_energy(
                notice_remove_field_energy_of_opponent_request.get_opponent_unique_id(),
                notice_remove_field_energy_of_opponent_request.get_remaining_field_energy()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeRemoveFieldEnergyOfOpponentResponse::new(response)
    }

    async fn notice_remove_energy_of_specific_unit_by_using_hand_card(
        &mut self, notice_remove_energy_of_specific_unit_by_using_hand_card_request: NoticeRemoveEnergyOfSpecificUnitByUsingHandCardRequest)
        -> NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_remove_energy_of_specific_unit_by_using_hand_card()");

        let mut game_field_unit_repository_guard=
            self.game_field_unit_repository.lock().await;

        // TODO: field unit service 에서 나온 값을 받아서 하는 것으로 바꿀 것
        let attached_energy_info =
            game_field_unit_repository_guard.acquire_energy_map_of_indexed_unit(
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_account_unique_id(),
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_unit_index()).to_attached_energy_info();

        drop(game_field_unit_repository_guard);

        let mut field_unit_energy_map = HashMap::new();
        field_unit_energy_map.insert(
            notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_unit_index(),
            attached_energy_info);

        let field_unit_energy_info = FieldUnitEnergyInfo::new(field_unit_energy_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_remove_energy_of_specific_opponent_unit(
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_opponent_unique_id(),
                field_unit_energy_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse::new(response)
    }

    async fn notice_apply_damage_to_specific_unit_by_using_hand_card(
        &mut self,
        notice_apply_damage_to_specific_unit_by_using_hand_card_request: NoticeApplyDamageToSpecificUnitByUsingHandCardRequest)
        -> NoticeApplyDamageToSpecificUnitByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_apply_damage_to_specific_unit_by_using_hand_card()");

        let mut game_field_unit_repository_guard=
            self.game_field_unit_repository.lock().await;

        // TODO: field unit service 에서 나온 값을 받아서 하는 것으로 바꿀 것
        let current_health_point_of_indexed_unit =
            game_field_unit_repository_guard.acquire_health_point_of_indexed_unit(
                notice_apply_damage_to_specific_unit_by_using_hand_card_request.get_opponent_unique_id(),
                notice_apply_damage_to_specific_unit_by_using_hand_card_request.get_unit_index())
                .get_current_health_point();

        let mut field_unit_health_point_map = HashMap::new();
        field_unit_health_point_map.insert(
            notice_apply_damage_to_specific_unit_by_using_hand_card_request.get_unit_index(),
            current_health_point_of_indexed_unit);

        let mut field_unit_survival_map = HashMap::new();

        let is_unit_alive = game_field_unit_repository_guard.acquire_survival_of_indexed_unit(
            notice_apply_damage_to_specific_unit_by_using_hand_card_request.get_opponent_unique_id(),
            notice_apply_damage_to_specific_unit_by_using_hand_card_request.get_unit_index());

        drop(game_field_unit_repository_guard);

        field_unit_survival_map.insert(
            notice_apply_damage_to_specific_unit_by_using_hand_card_request.get_unit_index(),
            is_unit_alive);

        let field_unit_health_info =
            FieldUnitHealthPointInfo::new(field_unit_health_point_map);
        let field_unit_survival_info =
            FieldUnitSurvivalInfo::new(field_unit_survival_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_apply_damage_to_specific_opponent_unit(
                notice_apply_damage_to_specific_unit_by_using_hand_card_request.get_opponent_unique_id(),
                field_unit_health_info,
                field_unit_survival_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeApplyDamageToSpecificUnitByUsingHandCardResponse::new(response)
    }

    async fn notice_apply_damage_to_every_unit_by_using_hand_card(
        &mut self, notice_apply_damage_to_every_unit_by_using_hand_card_request: NoticeApplyDamageToEveryUnitByUsingHandCardRequest)
        -> NoticeApplyDamageToEveryUnitByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_apply_damage_to_every_unit_by_using_hand_card()");

        let mut game_field_unit_repository_guard=
            self.game_field_unit_repository.lock().await;

        // TODO: field unit service 에서 나온 값을 받아서 하는 것으로 바꿀 것
        let field_unit_list_usize =
            game_field_unit_repository_guard.get_game_field_unit_map()
                .get(&notice_apply_damage_to_every_unit_by_using_hand_card_request
                    .get_opponent_unique_id()).unwrap().get_all_unit_list_in_game_field().len();

        let mut field_unit_health_point_map = HashMap::new();
        let mut field_unit_survival_map = HashMap::new();

        for unit_index in (0..field_unit_list_usize).rev() {
            let current_health_point_of_indexed_unit =
                game_field_unit_repository_guard.acquire_health_point_of_indexed_unit(
                    notice_apply_damage_to_every_unit_by_using_hand_card_request.get_opponent_unique_id(),
                    unit_index as i32).get_current_health_point();

            field_unit_health_point_map.insert(unit_index as i32, current_health_point_of_indexed_unit);

            let is_unit_alive =
                game_field_unit_repository_guard.acquire_survival_of_indexed_unit(
                    notice_apply_damage_to_every_unit_by_using_hand_card_request.get_opponent_unique_id(),
                    unit_index as i32);

            field_unit_survival_map.insert(unit_index as i32, is_unit_alive);
        }

        drop(game_field_unit_repository_guard);

        let field_unit_health_info =
            FieldUnitHealthPointInfo::new(field_unit_health_point_map);
        let field_unit_survival_info =
            FieldUnitSurvivalInfo::new(field_unit_survival_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_apply_damage_to_specific_opponent_unit(
                notice_apply_damage_to_every_unit_by_using_hand_card_request.get_opponent_unique_id(),
                field_unit_health_info,
                field_unit_survival_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeApplyDamageToEveryUnitByUsingHandCardResponse::new(response)
    }

    async fn notice_attach_energy_to_specific_unit_by_using_hand_card(
        &mut self, notice_attach_energy_to_specific_unit_by_using_hand_card_request: NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest)
        -> NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_attach_energy_to_specific_unit_by_using_hand_card()");

        // TODO: field unit service 에서 나온 값을 받아서 하는 것으로 바꿀 것
        let mut game_field_unit_repository_guard=
            self.game_field_unit_repository.lock().await;

        let attached_energy_info =
            game_field_unit_repository_guard.acquire_energy_map_of_indexed_unit(
                notice_attach_energy_to_specific_unit_by_using_hand_card_request.get_account_unique_id(),
                notice_attach_energy_to_specific_unit_by_using_hand_card_request.get_unit_index()).to_attached_energy_info();

        drop(game_field_unit_repository_guard);

        let mut field_unit_energy_map = HashMap::new();
        field_unit_energy_map.insert(
            notice_attach_energy_to_specific_unit_by_using_hand_card_request.get_unit_index(),
            attached_energy_info);

        let field_unit_energy_info = FieldUnitEnergyInfo::new(field_unit_energy_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_attach_energy_to_specific_unit(
                notice_attach_energy_to_specific_unit_by_using_hand_card_request.get_opponent_unique_id(),
                field_unit_energy_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse::new(response)
    }

    async fn notice_instant_death_of_specific_unit_by_using_hand_card(
        &mut self, notice_instant_death_of_specific_unit_by_using_hand_card_request: NoticeInstantDeathOfSpecificUnitByUsingHandCardRequest)
        -> NoticeInstantDeathOfSpecificUnitByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_instant_death_of_specific_unit_by_using_had_card()");

        // TODO: field unit service 에서 나온 값을 받아서 하는 것으로 바꿀 것
        let mut field_unit_survival_map = HashMap::new();
        field_unit_survival_map.insert(
            notice_instant_death_of_specific_unit_by_using_hand_card_request.get_dead_unit_index(), false);

        let field_unit_survival_info =
            FieldUnitSurvivalInfo::new(field_unit_survival_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_instant_death_of_specific_opponent_unit(
                notice_instant_death_of_specific_unit_by_using_hand_card_request.get_opponent_unique_id(),
                field_unit_survival_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeInstantDeathOfSpecificUnitByUsingHandCardResponse::new(response)
    }
}