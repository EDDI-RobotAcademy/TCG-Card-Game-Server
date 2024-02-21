use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;

use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;

use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::request::acquire_unit_attack_point_request::AcquireUnitAttackPointRequest;
use crate::game_field_unit::service::request::acquire_unit_extra_effect_request::AcquireUnitExtraEffectRequest;

use crate::game_field_unit::service::request::add_unit_to_game_field_request::AddUnitToGameFieldRequest;
use crate::game_field_unit::service::request::apply_catastrophic_damage_to_field_unit_request::ApplyCatastrophicDamageToFieldUnitRequest;
use crate::game_field_unit::service::request::apply_damage_to_target_unit_index_request::ApplyDamageToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::apply_instant_death_to_target_unit_index_request::ApplyInstantDeathToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::apply_passive_skill_list_request::ApplyPassiveSkillListRequest;
use crate::game_field_unit::service::request::apply_status_effect_damage_iteratively_request::ApplyStatusEffectDamageIterativelyRequest;
use crate::game_field_unit::service::request::attach_single_energy_to_unit_index_request::AttachSingleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_multiple_energy_to_unit_index_request::AttachMultipleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_special_energy_to_unit_index_request::AttachSpecialEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attack_target_unit_with_extra_effect_request::AttackTargetUnitWithExtraEffectRequest;
use crate::game_field_unit::service::request::judge_death_of_unit_request::JudgeDeathOfUnitRequest;
use crate::game_field_unit::service::request::detach_multiple_energy_from_field_unit_request::DetachMultipleEnergyFromFieldUnitRequest;
use crate::game_field_unit::service::request::execute_turn_action_request::ExecuteTurnActionRequest;
use crate::game_field_unit::service::request::find_active_skill_usage_unit_id_by_index_request::FindActiveSkillUsageUnitIdByIndexRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::get_current_attached_energy_of_field_unit_by_index_request::GetCurrentAttachedEnergyOfFieldUnitByIndexRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_field_unit_by_index_request::GetCurrentHealthPointOfFieldUnitByIndexRequest;
use crate::game_field_unit::service::request::get_game_field_unit_card_of_account_uique_id_request::GetGameFieldUnitCardOfAccountUniqueIdRequest;
use crate::game_field_unit::service::request::reset_turn_action_of_all_field_unit_request::ResetTurnActionOfAllFieldUnitRequest;
use crate::game_field_unit::service::response::acquire_unit_attack_point_response::AcquireUnitAttackPointResponse;
use crate::game_field_unit::service::response::acquire_unit_extra_effect_response::AcquireUnitExtraEffectResponse;

use crate::game_field_unit::service::response::add_unit_to_game_field_response::AddUnitToGameFieldResponse;
use crate::game_field_unit::service::response::apply_catastrophic_damage_to_field_unit_response::ApplyCatastrophicDamageToFieldUnitResponse;
use crate::game_field_unit::service::response::apply_damage_to_target_unit_index_response::ApplyDamageToTargetUnitIndexResponse;
use crate::game_field_unit::service::response::apply_instant_death_to_target_unit_index_response::ApplyInstantDeathToTargetUnitIndexResponse;
use crate::game_field_unit::service::response::apply_passive_skill_list_response::ApplyPassiveSkillListResponse;
use crate::game_field_unit::service::response::apply_status_effect_damage_iteratively_response::ApplyStatusEffectDamageIterativelyResponse;
use crate::game_field_unit::service::response::attach_single_energy_to_unit_index_response::AttachSingleEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attach_multiple_energy_to_unit_index_response::AttachMultipleEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attach_special_energy_to_unit_index_response::AttachSpecialEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attack_target_unit_with_extra_effect_response::AttackTargetUnitWithExtraEffectResponse;
use crate::game_field_unit::service::response::judge_death_of_unit_response::JudgeDeathOfUnitResponse;
use crate::game_field_unit::service::response::detach_multiple_energy_from_field_unit_response::DetachMultipleEnergyFromFieldUnitResponse;
use crate::game_field_unit::service::response::execute_turn_action_response::ExecuteTurnActionResponse;
use crate::game_field_unit::service::response::find_active_skill_usage_unit_id_by_index_response::FindActiveSkillUsageUnitIdByIndexResponse;
use crate::game_field_unit::service::response::find_target_unit_id_by_index_response::FindTargetUnitIdByIndexResponse;
use crate::game_field_unit::service::response::get_current_attached_energy_of_field_unit_by_index_response::GetCurrentAttachedEnergyOfFieldUnitByIndexResponse;
use crate::game_field_unit::service::response::get_current_health_point_of_field_unit_by_index_response::GetCurrentHealthPointOfFieldUnitByIndexResponse;
use crate::game_field_unit::service::response::get_game_field_unit_card_of_account_uique_id_response::GetGameFieldUnitCardOfAccountUniqueIdResponse;
use crate::game_field_unit::service::response::reset_turn_action_of_all_field_unit_response::ResetTurnActionOfAllFieldUnitResponse;
use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;


pub struct GameFieldUnitServiceImpl {
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
}

impl GameFieldUnitServiceImpl {
    pub fn new(game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
               game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,) -> Self {

        GameFieldUnitServiceImpl {
            game_field_unit_repository,
            game_round_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameFieldUnitServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldUnitServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldUnitServiceImpl::new(
                            GameFieldUnitRepositoryImpl::get_instance(),
                            GameRoundRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn get_user_round_value(&self, account_unique_id: i32) -> Option<i32> {
        let mut game_round_repository_guard = self.game_round_repository.lock().await;
        game_round_repository_guard
            .get_game_round_map()
            .get(&account_unique_id)
            .map(|user_round| user_round.get_round())
    }
}

#[async_trait]
impl GameFieldUnitService for GameFieldUnitServiceImpl {

    async fn add_unit_to_game_field(&mut self, add_unit_to_game_field_request: AddUnitToGameFieldRequest) -> AddUnitToGameFieldResponse {
        println!("GameFieldUnitServiceImpl: add_unit_to_game_field()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;

        let account_unique_id = add_unit_to_game_field_request.get_account_unique_id();

        let maybe_added_unit_index = game_field_unit_repository_guard.add_unit_to_game_field(
            account_unique_id,
            add_unit_to_game_field_request.get_unit_card_id(),
            add_unit_to_game_field_request.get_unit_race(),
            add_unit_to_game_field_request.get_unit_grade(),
            add_unit_to_game_field_request.get_unit_attack_point(),
            add_unit_to_game_field_request.get_unit_health_point(),
            add_unit_to_game_field_request.get_unit_attack_required_energy(),
            add_unit_to_game_field_request.has_third_passive_skill(),
            add_unit_to_game_field_request.has_second_passive_skill(),
            add_unit_to_game_field_request.has_third_passive_skill());

        let current_round_value =
            self.get_user_round_value(account_unique_id).await.unwrap();

        let summoned_round_setting_result =
            game_field_unit_repository_guard.set_field_unit_deployed_round(
                account_unique_id, maybe_added_unit_index, current_round_value);

        if summoned_round_setting_result == false {
            return AddUnitToGameFieldResponse::new(-1)
        }

        AddUnitToGameFieldResponse::new(maybe_added_unit_index)
    }

    async fn attach_energy_to_field_unit_index(&mut self, attach_energy_to_unit_index_request: AttachSingleEnergyToUnitIndexRequest) -> AttachSingleEnergyToUnitIndexResponse {
        println!("GameFieldUnitServiceImpl: attach_energy_to_field_unit_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.attach_multiple_energy_to_indexed_unit(
            attach_energy_to_unit_index_request.get_account_unique_id(),
            attach_energy_to_unit_index_request.get_unit_card_index(),
            attach_energy_to_unit_index_request.get_race_enum(),
            attach_energy_to_unit_index_request.get_quantity());

        AttachSingleEnergyToUnitIndexResponse::new(true)
    }

    async fn attach_multiple_energy_to_field_unit_index(&mut self, attach_multiple_energy_to_unit_index_request: AttachMultipleEnergyToUnitIndexRequest) -> AttachMultipleEnergyToUnitIndexResponse {
        println!("GameFieldUnitServiceImpl: attach_multiple_energy_to_field_unit_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.attach_multiple_energy_to_indexed_unit(
            attach_multiple_energy_to_unit_index_request.get_account_unique_id(),
            attach_multiple_energy_to_unit_index_request.get_unit_card_index(),
            attach_multiple_energy_to_unit_index_request.get_race_enum(),
            attach_multiple_energy_to_unit_index_request.get_quantity());

        AttachMultipleEnergyToUnitIndexResponse::new(true)
    }

    async fn find_target_unit_id_by_index(&mut self, find_target_unit_id_by_index_request: FindTargetUnitIdByIndexRequest) -> FindTargetUnitIdByIndexResponse {
        println!("GameFieldUnitServiceImpl: find_target_unit_id_by_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let found_target_unit_id = game_field_unit_repository_guard.find_target_unit_id_by_index(
            find_target_unit_id_by_index_request.get_opponent_unique_id(),
            find_target_unit_id_by_index_request.get_opponent_target_unit_index());

        FindTargetUnitIdByIndexResponse::new(found_target_unit_id)
    }

    async fn apply_damage_to_target_unit_index(&mut self, apply_damage_to_target_unit_index_response: ApplyDamageToTargetUnitIndexRequest) -> ApplyDamageToTargetUnitIndexResponse {
        println!("GameFieldUnitServiceImpl: apply_damage_to_target_unit_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.apply_damage_to_target_unit_index(
            apply_damage_to_target_unit_index_response.get_opponent_unique_id(),
            apply_damage_to_target_unit_index_response.get_opponent_target_unit_index(),
            apply_damage_to_target_unit_index_response.get_damage());

        ApplyDamageToTargetUnitIndexResponse::new(response)
    }

    async fn apply_instant_death_to_target_unit_index(&mut self, apply_instant_death_to_target_unit_index_request: ApplyInstantDeathToTargetUnitIndexRequest) -> ApplyInstantDeathToTargetUnitIndexResponse {
        println!("GameFieldUnitServiceImpl: apply_instant_death_to_target_unit_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.apply_instant_death_to_target_unit_index(
            apply_instant_death_to_target_unit_index_request.get_opponent_unique_id(),
            apply_instant_death_to_target_unit_index_request.get_opponent_target_unit_index());

        ApplyInstantDeathToTargetUnitIndexResponse::new(response)
    }

    async fn judge_death_of_unit(&mut self, judge_death_of_unit_request: JudgeDeathOfUnitRequest) -> JudgeDeathOfUnitResponse {
        println!("GameFieldUnitServiceImpl: judge_death_of_unit()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.judge_death_of_unit(
            judge_death_of_unit_request.get_account_unique_id(),
            judge_death_of_unit_request.get_unit_card_index());

        JudgeDeathOfUnitResponse::new(response)
    }

    async fn execute_turn_action(&mut self, execute_turn_action_request: ExecuteTurnActionRequest) -> ExecuteTurnActionResponse {
        println!("GameFieldUnitServiceImpl: execute_turn_action_of_unit()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.execute_turn_action_of_unit(
            execute_turn_action_request.get_account_unique_id(),
            execute_turn_action_request.get_unit_card_index());

        ExecuteTurnActionResponse::new(response)
    }

    async fn reset_turn_action_of_all_field_unit(&mut self, reset_turn_action_of_all_field_unit_request: ResetTurnActionOfAllFieldUnitRequest) -> ResetTurnActionOfAllFieldUnitResponse {
        println!("GameFieldUnitServiceImpl: reset_turn_action_of_all_field_unit()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.reset_turn_action_of_all_unit(
            reset_turn_action_of_all_field_unit_request.get_account_unique_id());

        ResetTurnActionOfAllFieldUnitResponse::new(response)
    }

    async fn get_current_health_point_of_field_unit_by_index(&self, get_current_health_point_of_field_unit_by_index_request: GetCurrentHealthPointOfFieldUnitByIndexRequest) -> GetCurrentHealthPointOfFieldUnitByIndexResponse {
        println!("GameFieldUnitServiceImpl: get_current_health_point_of_field_unit_by_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let found_field_unit_response = game_field_unit_repository_guard.find_indexed_unit(
            get_current_health_point_of_field_unit_by_index_request.get_account_unique_id(),
            get_current_health_point_of_field_unit_by_index_request.get_field_unit_index());

        return if let Some(found_field_unit) = found_field_unit_response {
            GetCurrentHealthPointOfFieldUnitByIndexResponse::new(found_field_unit.get_unit_health_point().get_current_health_point())
        } else {
            GetCurrentHealthPointOfFieldUnitByIndexResponse::new(-1)
        }
    }

    async fn attach_special_energy_to_field_unit_index(&mut self, attach_special_energy_to_unit_index_request: AttachSpecialEnergyToUnitIndexRequest) -> AttachSpecialEnergyToUnitIndexResponse {
        println!("GameFieldUnitServiceImpl: attach_special_energy_to_field_unit_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.attach_special_energy_to_indexed_unit(
            attach_special_energy_to_unit_index_request.get_account_unique_id(),
            attach_special_energy_to_unit_index_request.get_unit_card_index(),
            attach_special_energy_to_unit_index_request.get_race_enum(),
            attach_special_energy_to_unit_index_request.get_quantity(),
            attach_special_energy_to_unit_index_request.get_status_effect_list().to_vec());

        AttachSpecialEnergyToUnitIndexResponse::new(response)
    }

    async fn find_active_skill_usage_unit_id_by_index(&mut self, find_active_skill_usage_unit_id_by_index_request: FindActiveSkillUsageUnitIdByIndexRequest) -> FindActiveSkillUsageUnitIdByIndexResponse {
        println!("GameFieldUnitServiceImpl: find_active_skill_usage_unit_id_by_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let found_target_unit_id = game_field_unit_repository_guard.find_target_unit_id_by_index(
            find_active_skill_usage_unit_id_by_index_request.get_account_unique_id(),
            find_active_skill_usage_unit_id_by_index_request.get_active_skill_usage_unit_index());

        FindActiveSkillUsageUnitIdByIndexResponse::new(found_target_unit_id)
    }

    async fn apply_status_effect_damage_iteratively(&mut self, apply_status_effect_damage_iteratively_request: ApplyStatusEffectDamageIterativelyRequest) -> ApplyStatusEffectDamageIterativelyResponse {
        println!("GameFieldUnitServiceImpl: apply_status_effect_damage_iteratively()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        // TODO: 여기서 어떤 정보를 다룰 것인지에 대한 고찰이 필요함 (사망한 유닛들 ? 기타 등등) <- 이거 여기서 하는 것은 안됨 (SRP 위배)
        game_field_unit_repository_guard.apply_harmful_status_effect_damage_iteratively(
            apply_status_effect_damage_iteratively_request.get_account_unique_id());

        ApplyStatusEffectDamageIterativelyResponse::new(true)
    }

    async fn acquire_unit_attack_point(&mut self, acquire_unit_attack_point_request: AcquireUnitAttackPointRequest) -> AcquireUnitAttackPointResponse {
        println!("GameFieldUnitServiceImpl: acquire_unit_attack_point()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let attack_point = game_field_unit_repository_guard.acquire_unit_attack_point(
            acquire_unit_attack_point_request.get_account_unique_id(),
            acquire_unit_attack_point_request.get_attacker_unit_index());

        AcquireUnitAttackPointResponse::new(attack_point)
    }

    async fn acquire_unit_extra_effect(&mut self, acquire_unit_extra_effect_request: AcquireUnitExtraEffectRequest) -> AcquireUnitExtraEffectResponse {
        println!("GameFieldUnitServiceImpl: acquire_unit_extra_effect()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let extra_effect_list = game_field_unit_repository_guard.acquire_unit_extra_effect_by_index(
            acquire_unit_extra_effect_request.get_account_unique_id(),
            acquire_unit_extra_effect_request.get_attacker_unit_index());

        AcquireUnitExtraEffectResponse::new(extra_effect_list.clone())
    }

    async fn attack_target_unit_with_extra_effect(&mut self, attack_target_unit_with_extra_effect_request: AttackTargetUnitWithExtraEffectRequest) -> AttackTargetUnitWithExtraEffectResponse {
        println!("GameFieldUnitServiceImpl: attack_target_unit_with_extra_effect()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let attack_target_unit_with_extra_effect_response = game_field_unit_repository_guard.attack_target_unit_with_extra_effect(
            attack_target_unit_with_extra_effect_request.get_opponent_unique_id(),
            attack_target_unit_with_extra_effect_request.get_target_unit_index(),
            attack_target_unit_with_extra_effect_request.get_damage(),
            attack_target_unit_with_extra_effect_request.get_extra_status_effect_list().clone());

        AttackTargetUnitWithExtraEffectResponse::new(attack_target_unit_with_extra_effect_response)
    }

    async fn apply_passive_skill_list(&mut self, apply_passive_skill_list_request: ApplyPassiveSkillListRequest) -> ApplyPassiveSkillListResponse {
        println!("GameFieldUnitServiceImpl: apply_passive_skill_list()");

        let passive_skill_list = apply_passive_skill_list_request.get_passive_skill_list();

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;

        // TODO: Need to Refactor
        for passive_skill in passive_skill_list.iter() {
            match passive_skill.get_passive_skill_type() {
                PassiveSkillType::PhysicalImmunity => {
                    println!("물리 공격 면역 효과를 적용합니다");

                    game_field_unit_repository_guard.impose_extra_effect_state_to_indexed_unit(
                        apply_passive_skill_list_request.get_account_unique_id(),
                        apply_passive_skill_list_request.get_unit_card_index(),
                        passive_skill.clone());
                },
                PassiveSkillType::BroadArea => {
                    println!("패시브 광역기!");

                    let damage = passive_skill.get_skill_damage();

                    game_field_unit_repository_guard.apply_damage_to_every_unit(
                        apply_passive_skill_list_request.get_opponent_unique_id(),
                        damage);
                },
                PassiveSkillType::SingleTarget => {
                    println!("패시브 단일기!");

                    let damage = passive_skill.get_skill_damage();

                    // TODO: 일단은 가장 가까운 상대 유닛을 가격하도록 설정, 추후 변경 가능
                    game_field_unit_repository_guard.apply_damage_to_nearest_target(
                        apply_passive_skill_list_request.get_opponent_unique_id(),
                        apply_passive_skill_list_request.get_unit_card_index(),
                        damage);
                },
                _ => (),
            }
        }

        ApplyPassiveSkillListResponse::new(true)
    }

    async fn apply_catastrophic_damage_to_field_unit(&mut self, apply_catastrophic_damage_to_field_unit_request: ApplyCatastrophicDamageToFieldUnitRequest) -> ApplyCatastrophicDamageToFieldUnitResponse {
        println!("GameFieldUnitServiceImpl: apply_catastrophic_damage_to_field_unit()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let apply_catastrophic_damage_result = game_field_unit_repository_guard
            .apply_damage_to_every_unit(
                apply_catastrophic_damage_to_field_unit_request.get_opponent_unique_id(),
                apply_catastrophic_damage_to_field_unit_request.get_catastrophic_damage());

        ApplyCatastrophicDamageToFieldUnitResponse::new(apply_catastrophic_damage_result)
    }

    async fn detach_multiple_energy_from_field_unit(&mut self, detach_multiple_energy_from_field_unit_request: DetachMultipleEnergyFromFieldUnitRequest) -> DetachMultipleEnergyFromFieldUnitResponse {
        println!("GameFieldUnitServiceImpl: detach_multiple_energy_from_field_unit()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let detach_multiple_energy_from_field_unit_result = game_field_unit_repository_guard
            .detach_multiple_energy_from_indexed_unit(
                detach_multiple_energy_from_field_unit_request.get_account_unique_id(),
                detach_multiple_energy_from_field_unit_request.get_unit_card_index(),
                detach_multiple_energy_from_field_unit_request.get_race_enum().to_owned(),
                detach_multiple_energy_from_field_unit_request.get_quantity());

        DetachMultipleEnergyFromFieldUnitResponse::new(detach_multiple_energy_from_field_unit_result)
    }

    async fn get_current_attached_energy_of_field_unit_by_index(&mut self, get_current_attached_energy_of_field_unit_by_index_request: GetCurrentAttachedEnergyOfFieldUnitByIndexRequest) -> GetCurrentAttachedEnergyOfFieldUnitByIndexResponse {
        println!("GameFieldUnitServiceImpl: detach_multiple_energy_from_field_unit()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let found_field_unit_response = game_field_unit_repository_guard.find_indexed_unit(
            get_current_attached_energy_of_field_unit_by_index_request.get_account_unique_id(),
            get_current_attached_energy_of_field_unit_by_index_request.get_field_unit_index());

        return if let Some(found_field_unit) = found_field_unit_response {
            GetCurrentAttachedEnergyOfFieldUnitByIndexResponse::new(
                found_field_unit
                    .get_attached_energy()
                        .get_energy_quantity(
                            &RaceEnumValue::from(
                                *get_current_attached_energy_of_field_unit_by_index_request.get_energy_race() as i32)).unwrap_or(&-1).clone())
        } else {
            GetCurrentAttachedEnergyOfFieldUnitByIndexResponse::new(-1)
        }
    }

    // TODO: Need Refactor
    async fn get_game_field_unit_card_of_account_unique_id(&mut self, get_game_field_unit_card_to_service_request: GetGameFieldUnitCardOfAccountUniqueIdRequest) -> GetGameFieldUnitCardOfAccountUniqueIdResponse {
        println!("GameFieldUnitServiceImpl: get_game_field_unit_card_of_account_unique_id()");

        let account_unique_id = get_game_field_unit_card_to_service_request.get_account_unique_id();

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let mut game_field_unit_account_unique_id_index = game_field_unit_repository_guard.get_game_field_unit_map().clone();
        let game_field_unit_account_unique_id = game_field_unit_account_unique_id_index.get_mut(&account_unique_id).unwrap();
        let game_field_unit_list_of_account_unique_id =game_field_unit_account_unique_id.get_all_unit_list_in_game_field().clone();

        return GetGameFieldUnitCardOfAccountUniqueIdResponse::new(game_field_unit_list_of_account_unique_id)
    }
}
