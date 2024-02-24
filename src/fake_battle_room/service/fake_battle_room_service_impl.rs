use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_prepare_task::service::battle_prepare_task_service_impl::{player_deck_init_thread, player_field_energy_init_thread, player_field_unit_init_thread, player_hand_init_thread, player_lost_zone_init_thread, player_main_character_init_thread, player_round_init_thread, player_support_card_usage_counter_init_thread, player_tomb_init_thread, player_turn_init_thread, spawn_async_task_for_prepare_battle};

use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository::BattleReadyAccountHashRepository;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository_impl::BattleReadyAccountHashRepositoryImpl;
use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;
use crate::fake_battle_room::service::fake_battle_room_service::FakeBattleRoomService;
use crate::game_card_support_usage_counter::repository::game_card_support_usage_counter_repository_impl::GameCardSupportUsageCounterRepositoryImpl;
use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::game_field_energy::repository::game_field_energy_repository_impl::GameFieldEnergyRepositoryImpl;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::game_lost_zone::repository::game_lost_zone_repository_impl::GameLostZoneRepositoryImpl;
use crate::game_main_character::repository::game_main_character_repository_impl::GameMainCharacterRepositoryImpl;
use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;
use crate::game_tomb::repository::game_tomb_repository_impl::GameTombRepositoryImpl;

pub struct FakeBattleRoomServiceImpl {
    battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    game_lost_zone_repository: Arc<AsyncMutex<GameLostZoneRepositoryImpl>>,
    game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
    game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
    game_card_support_usage_counter_repository: Arc<AsyncMutex<GameCardSupportUsageCounterRepositoryImpl>>,
}

impl FakeBattleRoomServiceImpl {
    pub fn new(battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
               game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
               game_lost_zone_repository: Arc<AsyncMutex<GameLostZoneRepositoryImpl>>,
               game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
               game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
               game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
               game_card_support_usage_counter_repository: Arc<AsyncMutex<GameCardSupportUsageCounterRepositoryImpl>>) -> Self {

        FakeBattleRoomServiceImpl {
            battle_room_repository,
            game_deck_repository,
            game_hand_repository,
            game_field_energy_repository,
            game_field_unit_repository,
            game_lost_zone_repository,
            game_main_character_repository,
            game_tomb_repository,
            game_round_repository,
            game_card_support_usage_counter_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<FakeBattleRoomServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FakeBattleRoomServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        FakeBattleRoomServiceImpl::new(
                            BattleRoomRepositoryImpl::get_instance(),
                            GameDeckRepositoryImpl::get_instance(),
                            GameHandRepositoryImpl::get_instance(),
                            GameFieldEnergyRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance(),
                            GameLostZoneRepositoryImpl::get_instance(),
                            GameMainCharacterRepositoryImpl::get_instance(),
                            GameTombRepositoryImpl::get_instance(),
                            GameRoundRepositoryImpl::get_instance(),
                            GameCardSupportUsageCounterRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

pub async fn spawn_async_task_for_fake_battle_room(user_id: i32) {
    let task_deck_init = tokio::spawn(player_deck_init_thread(user_id));
    let task_hand_init = tokio::spawn(player_hand_init_thread(user_id));
    let task_field_energy_init = tokio::spawn(player_field_energy_init_thread(user_id));
    let task_field_unit_init = tokio::spawn(player_field_unit_init_thread(user_id));
    let task_lost_zone_init = tokio::spawn(player_lost_zone_init_thread(user_id));
    let task_main_character_init = tokio::spawn(player_main_character_init_thread(user_id));
    let task_tomb_init = tokio::spawn(player_tomb_init_thread(user_id));
    let task_round_init = tokio::spawn(player_round_init_thread(user_id));
    let task_turn_init = tokio::spawn(player_turn_init_thread(user_id));
    let task_support_usage_counter_init = tokio::spawn(player_support_card_usage_counter_init_thread(user_id));

    let _ = tokio::try_join!(
        task_deck_init,
        task_hand_init,
        task_field_energy_init,
        task_field_unit_init,
        task_lost_zone_init,
        task_main_character_init,
        task_tomb_init,
        task_round_init,
        task_turn_init,
        task_support_usage_counter_init,
    );
}

#[async_trait]
impl FakeBattleRoomService for FakeBattleRoomServiceImpl {
    async fn create_fake_battle_room(&self, fake_your_id: i32, fake_opponent_id: i32) {
        let users_to_process: Vec<i32> = vec![fake_your_id, fake_opponent_id];

        let battle_room_repository_guard = self.battle_room_repository.lock().await;

        battle_room_repository_guard.set_players_to_battle_room(users_to_process.clone()).await.expect("전투 배치 실패");
        let battle_room_count = battle_room_repository_guard.get_battle_room_count().await;
        drop(battle_room_repository_guard);

        let handles = users_to_process.into_iter().map(|user_id| {
            tokio::spawn(spawn_async_task_for_prepare_battle(user_id))
        }).collect::<Vec<_>>();

        for handle in handles {
            handle.await.expect("Failed to await spawned task");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_create_fake_battle_room() {


        let fake_battle_room_service_mutex = FakeBattleRoomServiceImpl::get_instance();
        let fake_battle_room_service_guard = fake_battle_room_service_mutex.lock().await;

        fake_battle_room_service_guard.create_fake_battle_room(1, 2).await;
    }
}
