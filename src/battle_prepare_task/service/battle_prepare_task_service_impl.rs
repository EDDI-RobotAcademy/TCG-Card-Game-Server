use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use tokio::{task, time};
use tokio::time::sleep;
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
use crate::game_round::repository::game_round_repository::GameRoundRepository;
use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;

// TODO: 사실상 지금 필요 없어졌으나 우선은 유지시킴 (비즈니스 로직에 지대한 영향 없음)
pub struct BattlePrepareTaskServiceImpl {
    battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    game_lost_zone_repository: Arc<AsyncMutex<GameLostZoneRepositoryImpl>>,
    game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
    game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
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
               game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,) -> Self {
        BattlePrepareTaskServiceImpl {
            battle_ready_account_hash_repository,
            game_deck_repository,
            game_hand_repository,
            game_field_energy_repository,
            game_field_unit_repository,
            game_lost_zone_repository,
            game_main_character_repository,
            game_tomb_repository,
            game_round_repository,
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
                            GameRoundRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

pub async fn player_deck_init_thread(user_id: i32) {
    let game_deck_repository_mutex = GameDeckRepositoryImpl::get_instance();
    let mut game_deck_repository_guard = game_deck_repository_mutex.lock().await;
    game_deck_repository_guard.create_game_deck_object(user_id);
    drop(game_deck_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_hand_init_thread(user_id: i32) {
    let game_hand_repository_mutex = GameHandRepositoryImpl::get_instance();
    let mut game_hand_repository_guard = game_hand_repository_mutex.lock().await;
    game_hand_repository_guard.create_game_hand_object(user_id);
    drop(game_hand_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_field_energy_init_thread(user_id: i32) {
    let game_field_energy_repository_mutex = GameFieldEnergyRepositoryImpl::get_instance();
    let mut game_field_energy_repository_guard = game_field_energy_repository_mutex.lock().await;
    game_field_energy_repository_guard.create_field_energy_object(user_id);
    drop(game_field_energy_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_field_unit_init_thread(user_id: i32) {
    let game_field_unit_repository_mutex = GameFieldUnitRepositoryImpl::get_instance();
    let mut game_field_unit_repository_guard = game_field_unit_repository_mutex.lock().await;
    game_field_unit_repository_guard.create_game_field_unit_object(user_id);
    drop(game_field_unit_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_lost_zone_init_thread(user_id: i32) {
    let game_lost_zone_repository_mutex = GameLostZoneRepositoryImpl::get_instance();
    let mut game_lost_zone_repository_guard = game_lost_zone_repository_mutex.lock().await;
    game_lost_zone_repository_guard.create_game_lost_zone_object(user_id);
    drop(game_lost_zone_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_main_character_init_thread(user_id: i32) {
    let game_main_character_repository_mutex = GameMainCharacterRepositoryImpl::get_instance();
    let mut game_main_character_repository_guard = game_main_character_repository_mutex.lock().await;
    game_main_character_repository_guard.create_game_main_character_object(user_id);
    drop(game_main_character_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_tomb_init_thread(user_id: i32) {
    let game_tomb_repository_mutex = GameTombRepositoryImpl::get_instance();
    let mut game_tomb_repository_guard = game_tomb_repository_mutex.lock().await;
    game_tomb_repository_guard.create_game_tomb_object(user_id);
    drop(game_tomb_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_round_init_thread(user_id: i32) {
    let game_round_repository_mutex = GameRoundRepositoryImpl::get_instance();
    let mut game_round_repository_guard = game_round_repository_mutex.lock().await;
    game_round_repository_guard.create_game_round_object(user_id);
    drop(game_round_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_battle_ready_account_hash_config_thread(user_id: i32) {
    let battle_ready_account_hash_repository_mutex = BattleReadyAccountHashRepositoryImpl::get_instance();
    let mut battle_ready_account_hash_repository_guard = battle_ready_account_hash_repository_mutex.lock().await;
    battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(user_id, BattleReadyAccountHashStatus::SUCCESS).await;
    drop(battle_ready_account_hash_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_turn_init_thread(user_id: i32) {
    let game_turn_repository_mutex = GameTurnRepositoryImpl::get_instance();
    let mut game_turn_repository_guard = game_turn_repository_mutex.lock().await;
    game_turn_repository_guard.create_game_turn_object(user_id);
    drop(game_turn_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn spawn_async_task_for_prepare_battle(user_id: i32) {
    let task_deck_init = tokio::spawn(player_deck_init_thread(user_id));
    let task_hand_init = tokio::spawn(player_hand_init_thread(user_id));
    let task_field_energy_init = tokio::spawn(player_field_energy_init_thread(user_id));
    let task_field_unit_init = tokio::spawn(player_field_unit_init_thread(user_id));
    let task_lost_zone_init = tokio::spawn(player_lost_zone_init_thread(user_id));
    let task_main_character_init = tokio::spawn(player_main_character_init_thread(user_id));
    let task_tomb_init = tokio::spawn(player_tomb_init_thread(user_id));
    let task_round_init = tokio::spawn(player_round_init_thread(user_id));
    let task_turn_init = tokio::spawn(player_turn_init_thread(user_id));

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
    );

    let task_battle_ready_account_hash_config =
        tokio::spawn(player_battle_ready_account_hash_config_thread(user_id));

    if let Ok(_) = task_battle_ready_account_hash_config.await {
        println!("Thread for User {} completed.", user_id);
    } else {
        eprintln!("Error in task_battle_ready_account_hash_config for User {}.", user_id);
    }
}

#[async_trait]
impl BattlePrepareTaskService for BattlePrepareTaskServiceImpl {

    async fn prepare_for_player_battle(&self) {
        loop {
            let battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;
            let battle_ready_account_hash = battle_ready_account_hash_repository_guard.get_battle_ready_account_hash();

            let users_to_process: Vec<i32> = battle_ready_account_hash
                .get_status_map()
                .iter()
                .filter(|(_, status)| *status == &BattleReadyAccountHashStatus::PREPARE)
                .map(|(&user_id, _)| user_id)
                .collect();

            drop(battle_ready_account_hash_repository_guard);

            let handles = users_to_process.into_iter().map(|user_id| {
                tokio::spawn(spawn_async_task_for_prepare_battle(user_id))
            }).collect::<Vec<_>>();

            for handle in handles {
                handle.await.expect("Failed to await spawned task");
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }
}

#[tokio::test]
async fn test_prepare_for_player_battle() {
    let battle_ready_repository = BattleReadyAccountHashRepositoryImpl::get_instance();

    let battle_ready_repository_clone = Arc::clone(&battle_ready_repository);

    let mut guard = battle_ready_repository.lock().await;
    guard.save_battle_ready_account_hash(1, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(2, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(3, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(4, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(5, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(6, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(7, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(8, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(9, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(10, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(11, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(12, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(13, BattleReadyAccountHashStatus::PREPARE).await;
    guard.save_battle_ready_account_hash(14, BattleReadyAccountHashStatus::PREPARE).await;

    drop(guard);

    // Spawn the asynchronous task
    let task = tokio::spawn(async move {
        let battle_prepare_service = BattlePrepareTaskServiceImpl::get_instance();
        let battle_prepare_service_guard = battle_prepare_service.lock().await;

        battle_prepare_service_guard
            .prepare_for_player_battle()
            .await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    task.abort();
}