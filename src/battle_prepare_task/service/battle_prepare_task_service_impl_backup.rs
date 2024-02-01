use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use futures::future::try_join_all;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_prepare_task::service::battle_prepare_task_service::BattlePrepareTaskService;

use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository::BattleReadyAccountHashRepository;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository_impl::BattleReadyAccountHashRepositoryImpl;
use crate::game_deck::repository::game_deck_repository::GameDeckRepository;
use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::game_field_energy::repository::game_field_energy_repository::GameFieldEnergyRepository;
use crate::game_field_energy::repository::game_field_energy_repository_impl::GameFieldEnergyRepositoryImpl;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_hand::repository::game_hand_repository::GameHandRepository;
use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::game_lost_zone::repository::game_lost_zone_repository::GameLostZoneRepository;
use crate::game_lost_zone::repository::game_lost_zone_repository_impl::GameLostZoneRepositoryImpl;
use crate::game_main_character::repository::game_main_character_repository::GameMainCharacterRepository;
use crate::game_main_character::repository::game_main_character_repository_impl::GameMainCharacterRepositoryImpl;
use crate::game_tomb::repository::game_tomb_repository::GameTombRepository;
use crate::game_tomb::repository::game_tomb_repository_impl::GameTombRepositoryImpl;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;


pub struct BattlePrepareTaskServiceImpl {
    battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    game_lost_zone_repository: Arc<AsyncMutex<GameLostZoneRepositoryImpl>>,
    game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
    game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
    game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
}

impl BattlePrepareTaskServiceImpl {
    pub fn new(battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
               game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
               game_lost_zone_repository: Arc<AsyncMutex<GameLostZoneRepositoryImpl>>,
               game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
               game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
               game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,) -> Self {
        BattlePrepareTaskServiceImpl {
            battle_ready_account_hash_repository,
            game_deck_repository,
            game_hand_repository,
            game_field_energy_repository,
            game_field_unit_repository,
            game_lost_zone_repository,
            game_main_character_repository,
            game_tomb_repository,
            game_turn_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattlePrepareTaskServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattlePrepareTaskServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattlePrepareTaskServiceImpl::new(
                            BattleReadyAccountHashRepositoryImpl::get_instance(),
                            GameDeckRepositoryImpl::get_instance(),
                            GameHandRepositoryImpl::get_instance(),
                            GameFieldEnergyRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance(),
                            GameLostZoneRepositoryImpl::get_instance(),
                            GameMainCharacterRepositoryImpl::get_instance(),
                            GameTombRepositoryImpl::get_instance(),
                            GameTurnRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    pub async fn spawn_async_task_for_prepare_battle(&self, user_id: i32) {
        let game_deck_repository_clone = Arc::clone(&self.game_deck_repository);
        let game_hand_repository_clone = Arc::clone(&self.game_hand_repository);
        let game_field_energy_repository_clone = Arc::clone(&self.game_field_energy_repository);
        let game_field_unit_repository_clone = Arc::clone(&self.game_field_unit_repository);
        let game_lost_zone_repository_clone  = Arc::clone(&self.game_lost_zone_repository);
        let game_main_character_repository_clone  = Arc::clone(&self.game_main_character_repository);
        let game_tomb_repository_clone  = Arc::clone(&self.game_tomb_repository);
        let game_turn_repository_clone  = Arc::clone(&self.game_turn_repository);
        let battle_ready_account_hash_repository_clone = Arc::clone(&self.battle_ready_account_hash_repository);


        tokio::spawn(async move {
            let mut game_deck_repository_guard = game_deck_repository_clone.lock().await;
            game_deck_repository_guard.create_game_deck_object(user_id);
            drop(game_deck_repository_guard);

            let mut game_hand_repository_guard = game_hand_repository_clone.lock().await;
            game_hand_repository_guard.create_game_hand_object(user_id);
            drop(game_hand_repository_guard);

            let mut game_field_energy_repository_guard = game_field_energy_repository_clone.lock().await;
            game_field_energy_repository_guard.create_field_energy_object(user_id);
            drop(game_field_energy_repository_guard);

            let mut game_field_unit_repository_guard = game_field_unit_repository_clone.lock().await;
            game_field_unit_repository_guard.create_game_field_unit_object(user_id);
            drop(game_field_unit_repository_guard);

            let mut game_lost_zone_repository_guard = game_lost_zone_repository_clone.lock().await;
            game_lost_zone_repository_guard.create_game_lost_zone_object(user_id);
            drop(game_lost_zone_repository_guard);

            let mut game_main_character_repository_guard = game_main_character_repository_clone.lock().await;
            game_main_character_repository_guard.create_game_main_character_object(user_id);
            drop(game_main_character_repository_guard);

            let mut game_tomb_repository_guard = game_tomb_repository_clone.lock().await;
            game_tomb_repository_guard.create_game_tomb_object(user_id);
            drop(game_tomb_repository_guard);

            let mut game_turn_repository_guard = game_turn_repository_clone.lock().await;
            game_turn_repository_guard.create_game_turn_object(user_id);
            drop(game_turn_repository_guard);

            let mut battle_ready_account_hash_repository_guard = battle_ready_account_hash_repository_clone.lock().await;
            battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(user_id, BattleReadyAccountHashStatus::SUCCESS).await;
            drop(battle_ready_account_hash_repository_guard);

            println!("Thread for User {} completed.", user_id);
        });
    }
}

#[async_trait]
impl BattlePrepareTaskService for BattlePrepareTaskServiceImpl {
    // async fn prepare_for_player_battle(&self) {
    //     loop {
    //         let mut battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;
    //         let battle_ready_account_hash = battle_ready_account_hash_repository_guard.get_battle_ready_account_hash();
    //         drop(battle_ready_account_hash_repository_guard);
    //
    //         // println!("Preparing for battle. Hash: {:?}", battle_ready_account_hash);
    //
    //         for (user_id, status) in battle_ready_account_hash.get_status_map().iter() {
    //             if status == &BattleReadyAccountHashStatus::PREPARE {
    //                 let mut battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;
    //                 battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(*user_id, BattleReadyAccountHashStatus::PREPARE_PROCESS).await;
    //                 drop(battle_ready_account_hash_repository_guard);
    //
    //                 self.spawn_async_task_for_prepare_battle(*user_id).await;
    //             }
    //         }
    //
    //         tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    //     }
    // }

    async fn prepare_for_player_battle(&self) {
        loop {
            let battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;
            let battle_ready_account_hash = battle_ready_account_hash_repository_guard.get_battle_ready_account_hash();

            // Clone the user IDs that need further processing
            let users_to_process: Vec<i32> = battle_ready_account_hash
                .get_status_map()
                .iter()
                .filter(|(_, status)| *status == &BattleReadyAccountHashStatus::PREPARE)
                .map(|(&user_id, _)| user_id)
                .collect();

            // Drop the lock guard before further processing
            drop(battle_ready_account_hash_repository_guard);

            for user_id in users_to_process {
                let mut battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;
                battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(user_id, BattleReadyAccountHashStatus::PREPARE_PROCESS).await;
                // Additional processing with the cloned user_id
                drop(battle_ready_account_hash_repository_guard);

                self.spawn_async_task_for_prepare_battle(user_id).await;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }

    // async fn prepare_for_player_battle(&self) {
    //     loop {
    //         let battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;
    //         let battle_ready_account_hash = battle_ready_account_hash_repository_guard.get_battle_ready_account_hash();
    //
    //         // Clone the user IDs that need further processing
    //         let users_to_process: Vec<i32> = battle_ready_account_hash
    //             .get_status_map()
    //             .iter()
    //             .filter(|(_, status)| *status == &BattleReadyAccountHashStatus::PREPARE)
    //             .map(|(&user_id, _)| user_id)
    //             .collect();
    //
    //         // Drop the lock guard before further processing
    //         drop(battle_ready_account_hash_repository_guard);
    //
    //         // Use tokio::spawn for each user ID without awaiting immediately
    //         let mut spawned_tasks = Vec::new();
    //
    //         for user_id in users_to_process {
    //             let self_clone = self.clone(); // Clone the entire struct to be moved into the spawned task
    //
    //             let task = tokio::spawn(async move {
    //                 let mut battle_ready_account_hash_repository_guard = self_clone.battle_ready_account_hash_repository.lock().await;
    //                 battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(user_id, BattleReadyAccountHashStatus::PREPARE_PROCESS).await;
    //                 // Additional processing with the cloned user_id
    //                 drop(battle_ready_account_hash_repository_guard);
    //
    //                 self_clone.spawn_async_task_for_prepare_battle(user_id).await;
    //             });
    //
    //             spawned_tasks.push(task);
    //         }
    //
    //         // Await all spawned tasks to complete in parallel
    //         if let Err(e) = try_join_all(spawned_tasks).await {
    //             eprintln!("Failed to join tasks: {:?}", e);
    //         }
    //
    //         tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    //     }
    // }
}

#[tokio::test]
async fn test_prepare_for_player_battle() {
    let battle_ready_repository = BattleReadyAccountHashRepositoryImpl::get_instance();

    let battle_ready_repository_clone = Arc::clone(&battle_ready_repository);

    let mut guard = battle_ready_repository.lock().await;
    guard.save_battle_ready_account_hash(1, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(2, BattleReadyAccountHashStatus::PREPARE).await;

    drop(guard);

    // Spawn the asynchronous task
    let task = tokio::spawn(async move {
        let battle_prepare_service = BattlePrepareTaskServiceImpl::get_instance();
        let battle_prepare_service_guard = battle_prepare_service.lock().await;

        battle_prepare_service_guard
            .prepare_for_player_battle()
            .await;
    });

    // Sleep for a sufficient duration to allow the task to run
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Cancel the task after the delay
    task.abort();
}