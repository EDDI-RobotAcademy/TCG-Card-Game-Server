use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_passive_skill::entity::passive_skill_casting_condition::PassiveSkillCastingCondition;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::entity::harmful_status_effect::HarmfulStatusEffect;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service::GameFieldUnitActionPossibilityValidatorService;
use crate::game_field_unit_action_possibility_validator::service::request::is_unit_basic_attack_possible_request::{IsUnitBasicAttackPossibleRequest};
use crate::game_field_unit_action_possibility_validator::service::request::is_using_active_skill_possible_request::IsUsingActiveSkillPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_using_deploy_passive_skill_possible_request::IsUsingDeployPassiveSkillPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_using_turn_start_passive_skill_request::IsUsingTurnStartPassiveSkillPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::response::is_unit_basic_attack_possible_response::{IsUnitBasicAttackPossibleResponse};
use crate::game_field_unit_action_possibility_validator::service::response::is_using_active_skill_possible_response::IsUsingActiveSkillPossibleResponse;
use crate::game_field_unit_action_possibility_validator::service::response::is_using_deploy_passive_skill_possible_response::IsUsingDeployPassiveSkillPossibleResponse;
use crate::game_field_unit_action_possibility_validator::service::response::is_using_turn_start_passive_skill_response::IsUsingTurnStartPassiveSkillPossibleResponse;
use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;

pub struct GameFieldUnitActionPossibilityValidatorServiceImpl {
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
}

impl GameFieldUnitActionPossibilityValidatorServiceImpl {
    pub fn new(game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>) -> Self {

        GameFieldUnitActionPossibilityValidatorServiceImpl {
            game_round_repository,
            game_field_unit_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldUnitActionPossibilityValidatorServiceImpl::new(
                            GameRoundRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn get_field_unit_turn_action(&self,
                                        account_unique_id: i32,
                                        field_unit_index: i32) -> Option<bool> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .check_turn_action_of_unit(field_unit_index as usize))
    }

    async fn get_field_unit_deployed_round(&self,
                                           account_unique_id: i32,
                                           field_unit_index: i32) -> Option<i32> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .get_unit_deployed_round(field_unit_index as usize))
    }

    async fn get_total_energy_count_of_field_unit(&self,
                                                  account_unique_id: i32,
                                                  field_unit_index: i32) -> Option<i32> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .get_total_energy_count_of_unit(field_unit_index as usize))
    }

    async fn get_field_unit_race_energy(&self,
                                        account_unique_id: i32,
                                        field_unit_index: i32,
                                        race_enum: RaceEnum) -> Option<i32> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .get_attached_energy_count_of_field_unit_with_race(field_unit_index as usize,
                                                                       race_enum))
    }

    async fn get_field_unit_harmful_effect(&self,
                                           account_unique_id: i32,
                                           field_unit_index: i32) -> Option<Vec<HarmfulStatusEffect>> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .get_harmful_status_list_of_indexed_unit(field_unit_index as usize))
    }

    async fn get_player_round(&self,
                              account_unique_id: i32) -> Option<i32> {
        let mut game_round_repository_guard =
            self.game_round_repository.lock().await;

        game_round_repository_guard
            .get_game_round_map()
            .get(&account_unique_id)
            .map(|user_round| user_round.get_round())
    }
    async fn get_field_unit_index_passive_possible(&self,
                                        account_unique_id: i32,
                                        field_unit_index: i32,
                                        passive_index: i32) -> Option<bool> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .get_index_passive_of_unit(field_unit_index as usize, passive_index as usize))
    }
}

#[async_trait]
impl GameFieldUnitActionPossibilityValidatorService for GameFieldUnitActionPossibilityValidatorServiceImpl {
    async fn is_unit_basic_attack_possible(
        &self, is_unit_basic_attack_possible_request: IsUnitBasicAttackPossibleRequest)
        -> IsUnitBasicAttackPossibleResponse {

        println!("GameFieldUnitActionValidatorServiceImpl: is_unit_basic_attack_possible()");

        // 1. check unit turn action
        let turn_action = self.get_field_unit_turn_action(
            is_unit_basic_attack_possible_request.get_account_unique_id(),
            is_unit_basic_attack_possible_request.get_field_unit_index()).await.unwrap_or(true);

        if turn_action == true {
            println!("이번 턴에 더 이상 액션이 불가능합니다.");
            return IsUnitBasicAttackPossibleResponse::new(false)
        }

        // 2. check round
        let field_unit_deployed_round = self.get_field_unit_deployed_round(
            is_unit_basic_attack_possible_request.get_account_unique_id(),
            is_unit_basic_attack_possible_request.get_field_unit_index()).await.unwrap_or(-1);

        let player_current_round = self.get_player_round(
            is_unit_basic_attack_possible_request.get_account_unique_id()).await.unwrap_or(-1);

        if player_current_round == field_unit_deployed_round {
            println!("소환된 턴에는 액션이 불가합니다.");
            return IsUnitBasicAttackPossibleResponse::new(false)
        }

        // 3. check energy enough
        let total_energy_count_of_field_unit = self.get_total_energy_count_of_field_unit(
            is_unit_basic_attack_possible_request.get_account_unique_id(),
            is_unit_basic_attack_possible_request.get_field_unit_index()).await.unwrap_or(-1);

        if is_unit_basic_attack_possible_request
            .get_basic_attack_required_energy_count() > total_energy_count_of_field_unit {
            println!("기본 공격에 필요한 에너지가 충분하지 않습니다.");
            return IsUnitBasicAttackPossibleResponse::new(false)
        }

        let harmful_status_effect_list = self.get_field_unit_harmful_effect(
            is_unit_basic_attack_possible_request.get_account_unique_id(),
            is_unit_basic_attack_possible_request.get_field_unit_index()).await.unwrap_or(Vec::new());

        for harmful_status in harmful_status_effect_list {
            if harmful_status.get_harmful_effect() == &ExtraEffect::Freeze {
                println!("빙결 상태의 유닛은 공격이 불가합니다.");
                return IsUnitBasicAttackPossibleResponse::new(false)
            }
        }

        IsUnitBasicAttackPossibleResponse::new(true)
    }

    async fn is_using_active_skill_possible(
        &self, is_using_active_skill_possible_request: IsUsingActiveSkillPossibleRequest)
        -> IsUsingActiveSkillPossibleResponse {

        println!("GameFieldUnitActionValidatorServiceImpl: is_using_active_skill_possible()");

        // 1. check unit turn action
        let turn_action = self.get_field_unit_turn_action(
            is_using_active_skill_possible_request.get_account_unique_id(),
            is_using_active_skill_possible_request.get_field_unit_index()).await.unwrap_or(true);

        if turn_action == true {
            println!("이번 턴에 더 이상 액션이 불가능합니다.");
            return IsUsingActiveSkillPossibleResponse::new(false)
        }

        // 2. check round
        let field_unit_deployed_round = self.get_field_unit_deployed_round(
            is_using_active_skill_possible_request.get_account_unique_id(),
            is_using_active_skill_possible_request.get_field_unit_index()).await.unwrap_or(-1);

        let player_current_round = self.get_player_round(
            is_using_active_skill_possible_request.get_account_unique_id()).await.unwrap_or(-1);

        if player_current_round == field_unit_deployed_round {
            println!("소환된 턴에는 액션이 불가합니다.");
            return IsUsingActiveSkillPossibleResponse::new(false)
        }

        // 3. energy enough
        let required_energy_map_to_use_active_skill =
            is_using_active_skill_possible_request.get_skill_required_energy_map().clone();

        for (race, required_energy_count) in required_energy_map_to_use_active_skill {
            let attached_race_energy_count = self.get_field_unit_race_energy(
                is_using_active_skill_possible_request.get_account_unique_id(),
                is_using_active_skill_possible_request.get_field_unit_index(),
                race).await.unwrap_or(-1);

            if required_energy_count > attached_race_energy_count {
                println!("액티브 스킬 사용에 필요한 에너지가 충분하지 않습니다.");
                return IsUsingActiveSkillPossibleResponse::new(false)
            }
        }

        let harmful_status_effect_list = self.get_field_unit_harmful_effect(
            is_using_active_skill_possible_request.get_account_unique_id(),
            is_using_active_skill_possible_request.get_field_unit_index()).await.unwrap_or(Vec::new());

        for harmful_status in harmful_status_effect_list {
            if harmful_status.get_harmful_effect() == &ExtraEffect::Freeze {
                println!("빙결 상태의 유닛은 공격이 불가합니다.");
                return IsUsingActiveSkillPossibleResponse::new(false)
            }
        }

        IsUsingActiveSkillPossibleResponse::new(true)
    }

    async fn is_using_deploy_passive_skill_possible(
        &self, is_using_deploy_passive_skill_possible_request: IsUsingDeployPassiveSkillPossibleRequest)
        -> IsUsingDeployPassiveSkillPossibleResponse {

        println!("GameFieldUnitActionValidatorServiceImpl: is_using_deploy_passive_skill_possible()");

        // 1. check round
        let field_unit_deployed_round = self.get_field_unit_deployed_round(
            is_using_deploy_passive_skill_possible_request.get_account_unique_id(),
            is_using_deploy_passive_skill_possible_request.get_field_unit_index()).await.unwrap_or(-1);

        let player_current_round = self.get_player_round(
            is_using_deploy_passive_skill_possible_request.get_account_unique_id()).await.unwrap_or(-1);

        if !player_current_round == field_unit_deployed_round {
            println!("소환된 턴에만 사용할 수 있습니다.");
            return IsUsingDeployPassiveSkillPossibleResponse::new(false)
        }

        // 2. check passive index is true

        let index_passive_possible = self.get_field_unit_index_passive_possible(
            is_using_deploy_passive_skill_possible_request.get_account_unique_id(),
            is_using_deploy_passive_skill_possible_request.get_field_unit_index(),
            is_using_deploy_passive_skill_possible_request.get_passive_skill_index()).await.unwrap();

        if !index_passive_possible {
            println!("해당 패시브는 사용할 수 없습니다.");
            return IsUsingDeployPassiveSkillPossibleResponse::new(false)
        }

        // 3. is casting condition deploy
        let casting_condition = is_using_deploy_passive_skill_possible_request.get_passive_skill_casting_condition().
            iter().find(|x| x == &&PassiveSkillCastingCondition::Deploy).is_some();

        if !casting_condition {
            println!("소환시 사용되는 패시브가 아닙니다.");
            return IsUsingDeployPassiveSkillPossibleResponse::new(false)
        }

        IsUsingDeployPassiveSkillPossibleResponse::new(true)
    }

    async fn is_using_turn_start_passive_skill_possible(
        &self, is_using_turn_start_passive_skill_possible_request: IsUsingTurnStartPassiveSkillPossibleRequest)
        -> IsUsingTurnStartPassiveSkillPossibleResponse {
        println!("GameFieldUnitActionValidatorServiceImpl: is_using_deploy_passive_skill_possible()");

        // 1. check round
        let field_unit_deployed_round = self.get_field_unit_deployed_round(
            is_using_turn_start_passive_skill_possible_request.get_account_unique_id(),
            is_using_turn_start_passive_skill_possible_request.get_field_unit_index()).await.unwrap_or(-1);

        let player_current_round = self.get_player_round(
            is_using_turn_start_passive_skill_possible_request.get_account_unique_id()).await.unwrap_or(-1);

        if player_current_round == field_unit_deployed_round {
            println!("소환된 턴에는 사용할 수 없습니다.");
            return IsUsingTurnStartPassiveSkillPossibleResponse::new(false)
        }

        // 2. check passive index is true

        let index_passive_possible = self.get_field_unit_index_passive_possible(
            is_using_turn_start_passive_skill_possible_request.get_account_unique_id(),
            is_using_turn_start_passive_skill_possible_request.get_field_unit_index(),
            is_using_turn_start_passive_skill_possible_request.get_passive_skill_index()).await.unwrap();

        if !index_passive_possible {
            println!("해당 패시브는 사용할 수 없습니다.");
            return IsUsingTurnStartPassiveSkillPossibleResponse::new(false)
        }

        // 3. is casting condition deploy
        let casting_condition = is_using_turn_start_passive_skill_possible_request.get_passive_skill_casting_condition().
            iter().find(|x| x == &&PassiveSkillCastingCondition::TurnStart).is_some();

        if !casting_condition {
            println!("턴 시작 시 사용되는 패시브가 아닙니다.");
            return IsUsingTurnStartPassiveSkillPossibleResponse::new(false)
        }

        IsUsingTurnStartPassiveSkillPossibleResponse::new(true)
    }
}