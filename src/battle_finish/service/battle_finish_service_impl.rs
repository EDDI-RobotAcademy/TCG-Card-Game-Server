use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use tokio::{task, time};
use tokio::time::sleep;

use crate::battle_finish::service::battle_finish_service::BattleFinishService;
use crate::battle_prepare_task::service::battle_prepare_task_service_impl::BattlePrepareTaskServiceImpl;

use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository::BattleReadyAccountHashRepository;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository_impl::BattleReadyAccountHashRepositoryImpl;
use crate::game_card_support_usage_counter::repository::game_card_support_usage_counter_repository::GameCardSupportUsageCounterRepository;
use crate::game_card_support_usage_counter::repository::game_card_support_usage_counter_repository_impl::GameCardSupportUsageCounterRepositoryImpl;
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
pub struct BattleFinishServiceImpl {
    battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
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

impl BattleFinishServiceImpl {
    pub fn new(battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
               game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
               game_lost_zone_repository: Arc<AsyncMutex<GameLostZoneRepositoryImpl>>,
               game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
               game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
               game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
               game_card_support_usage_counter_repository: Arc<AsyncMutex<GameCardSupportUsageCounterRepositoryImpl>>) -> Self {
        BattleFinishServiceImpl {
            battle_ready_account_hash_repository,
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

    pub fn get_instance() -> Arc<AsyncMutex<BattleFinishServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleFinishServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleFinishServiceImpl::new(
                            BattleReadyAccountHashRepositoryImpl::get_instance(),
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

pub async fn remove_player_deck(user_id: i32) {
    let game_deck_repository_mutex = GameDeckRepositoryImpl::get_instance();
    let mut game_deck_repository_guard = game_deck_repository_mutex.lock().await;
    game_deck_repository_guard.remove_game_deck_hash_by_account_unique_id(user_id);
    println!("game_deck_map: {:?}", game_deck_repository_guard.get_game_deck_map());
    drop(game_deck_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_hand(user_id: i32) {
    let game_hand_repository_mutex = GameHandRepositoryImpl::get_instance();
    let mut game_hand_repository_guard = game_hand_repository_mutex.lock().await;
    game_hand_repository_guard.remove_game_hand_hash_by_account_unique_id(user_id);
    println!("game_hand_map: {:?}", game_hand_repository_guard.get_game_hand_map());
    drop(game_hand_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_field_energy(user_id: i32) {
    let game_field_energy_repository_mutex = GameFieldEnergyRepositoryImpl::get_instance();
    let mut game_field_energy_repository_guard = game_field_energy_repository_mutex.lock().await;
    game_field_energy_repository_guard.remove_game_field_energy_hash_by_account_unique_id(user_id);
    println!("game_field_energy_map: {:?}", game_field_energy_repository_guard.get_game_field_energy_map());
    drop(game_field_energy_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_field_unit(user_id: i32) {
    let game_field_unit_repository_mutex = GameFieldUnitRepositoryImpl::get_instance();
    let mut game_field_unit_repository_guard = game_field_unit_repository_mutex.lock().await;
    game_field_unit_repository_guard.remove_game_field_unit_hash_by_account_unique_id(user_id);
    println!("game_field_unit_map: {:?}", game_field_unit_repository_guard.get_game_field_unit_map());
    drop(game_field_unit_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_lost_zone(user_id: i32) {
    let game_lost_zone_repository_mutex = GameLostZoneRepositoryImpl::get_instance();
    let mut game_lost_zone_repository_guard = game_lost_zone_repository_mutex.lock().await;
    game_lost_zone_repository_guard.remove_game_lost_zone_hash_by_account_unique_id(user_id);
    println!("game_lost_zone_map: {:?}", game_lost_zone_repository_guard.get_game_lost_zone_map());
    drop(game_lost_zone_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_main_character(user_id: i32) {
    let game_main_character_repository_mutex = GameMainCharacterRepositoryImpl::get_instance();
    let mut game_main_character_repository_guard = game_main_character_repository_mutex.lock().await;
    game_main_character_repository_guard.remove_game_main_character_hash_by_account_unique_id(user_id);
    println!("game_main_character_map: {:?}", game_main_character_repository_guard.get_game_main_character_map());
    drop(game_main_character_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_tomb(user_id: i32) {
    let game_tomb_repository_mutex = GameTombRepositoryImpl::get_instance();
    let mut game_tomb_repository_guard = game_tomb_repository_mutex.lock().await;
    game_tomb_repository_guard.remove_game_tomb_hash_by_account_unique_id(user_id);
    println!("game_tomb_map: {:?}", game_tomb_repository_guard.get_game_tomb_map());
    drop(game_tomb_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_round(user_id: i32) {
    let game_round_repository_mutex = GameRoundRepositoryImpl::get_instance();
    let mut game_round_repository_guard = game_round_repository_mutex.lock().await;
    game_round_repository_guard.remove_game_round_hash_by_account_unique_id(user_id);
    println!("game_round_map: {:?}", game_round_repository_guard.get_game_round_map());
    drop(game_round_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_turn(user_id: i32) {
    let game_turn_repository_mutex = GameTurnRepositoryImpl::get_instance();
    let mut game_turn_repository_guard = game_turn_repository_mutex.lock().await;
    game_turn_repository_guard.remove_game_turn_hash_by_account_unique_id(user_id);
    println!("game_turn_map: {:?}", game_turn_repository_guard.get_game_turn_map());
    drop(game_turn_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn remove_player_support_card_usage_counter(user_id: i32) {
    let game_card_support_usage_counter_repository_mutex = GameCardSupportUsageCounterRepositoryImpl::get_instance();
    let mut game_card_support_usage_counter_repository_guard = game_card_support_usage_counter_repository_mutex.lock().await;
    game_card_support_usage_counter_repository_guard.remove_game_support_card_usage_counter_hash_by_account_unique_id(user_id);
    println!("game_card_support_usage_counter_map: {:?}", game_card_support_usage_counter_repository_guard.check_support_card_usage_counter(user_id));
    drop(game_card_support_usage_counter_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

pub async fn player_battle_ready_account_hash_config_thread(user_id: i32) {
    let battle_ready_account_hash_repository_mutex = BattleReadyAccountHashRepositoryImpl::get_instance();
    let mut battle_ready_account_hash_repository_guard = battle_ready_account_hash_repository_mutex.lock().await;
    battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(user_id, BattleReadyAccountHashStatus::SUCCESS).await;
    println!("battle_ready_account_hash: {:?}", battle_ready_account_hash_repository_guard.get_battle_ready_account_hash());
    drop(battle_ready_account_hash_repository_guard);

    sleep(time::Duration::from_millis(300)).await;
}

// TODO :: battle_ready_acount_hash 삭제 기능 구현
// pub async fn remove_player_battle_ready_account_hash_status(user_id: i32) {
//     let battle_ready_account_hash_repository_mutex = BattleReadyAccountHashRepositoryImpl::get_instance();
//     let mut battle_ready_account_hash_repository_guard = battle_ready_account_hash_repository_mutex.lock().await;
//     battle_ready_account_hash_repository_guard.remove_battle_ready_account_hash_status_hash_by_account_unique_id(user_id);
//     println!("battle_ready_account_hash: {:?}", battle_ready_account_hash_repository_guard.get_battle_ready_account_hash());
//     drop(battle_ready_account_hash_repository_guard);
//
//     sleep(time::Duration::from_millis(300)).await;
// }

pub async fn spawn_async_task_for_finish_battle(user_id: i32) {
    let task_deck_remove = tokio::spawn(remove_player_deck(user_id));
    let task_hand_remove = tokio::spawn(remove_player_hand(user_id));
    let task_field_energy_remove = tokio::spawn(remove_player_field_energy(user_id));
    let task_field_unit_remove = tokio::spawn(remove_player_field_unit(user_id));
    let task_lost_zone_remove = tokio::spawn(remove_player_lost_zone(user_id));
    let task_main_character_remove = tokio::spawn(remove_player_main_character(user_id));
    let task_tomb_remove = tokio::spawn(remove_player_tomb(user_id));
    let task_round_remove = tokio::spawn(remove_player_round(user_id));
    let task_turn_remove = tokio::spawn(remove_player_turn(user_id));
    let task_support_usage_counter_remove = tokio::spawn(remove_player_support_card_usage_counter(user_id));
    // TODO :: 삭제 기능 구
    let task_battle_ready_account_hash_status_remove = tokio::spawn(player_battle_ready_account_hash_config_thread(user_id));

    let _ = tokio::try_join!(
        task_deck_remove,
        task_hand_remove,
        task_field_energy_remove,
        task_field_unit_remove,
        task_lost_zone_remove,
        task_main_character_remove,
        task_tomb_remove,
        task_round_remove,
        task_turn_remove,
        task_support_usage_counter_remove,
        task_battle_ready_account_hash_status_remove,
    );
}

#[async_trait]
impl BattleFinishService for BattleFinishServiceImpl {

    async fn battle_finish_for_player_battle(&self) {
        loop {
            let battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;
            let battle_ready_account_hash = battle_ready_account_hash_repository_guard.get_battle_ready_account_hash();

            // 대전 준비상태(BATTLE_FINISH)인 사용자의 ID를 수집
            let users_to_process: Vec<i32> = battle_ready_account_hash
                .get_status_map()
                .iter()
                .filter(|(_, status)| *status == &BattleReadyAccountHashStatus::BATTLE_FINISH)
                .map(|(&user_id, _)| user_id)
                .collect();

            drop(battle_ready_account_hash_repository_guard);

            // 수집된 ID 마다 battle finish 작업 실행 (spawn_async_task_for_prepare_battle)
            let handles = users_to_process.into_iter().map(|user_id| {
                tokio::spawn(spawn_async_task_for_finish_battle(user_id))
            }).collect::<Vec<_>>();

            // 각 작업이 완료될 때가지 대기. 작업이 실패하면 예외를 발생시킴
            for handle in handles {
                handle.await.expect("Failed to await spawned task");
            }

            // 1 초 동안 대기
            tokio::time::sleep(time::Duration::from_millis(1000)).await;
        }
    }
}

#[tokio::test]
async fn test_battle_finish_for_player_battle() {
    let user_id = 1;
    let game_deck_repository_mutex = GameDeckRepositoryImpl::get_instance();
    let mut game_deck_repository_guard = game_deck_repository_mutex.lock().await;
    game_deck_repository_guard.create_game_deck_object(user_id);
    println!("game_deck_map: {:?}", game_deck_repository_guard.get_game_deck_map());
    drop(game_deck_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_hand_repository_mutex = GameHandRepositoryImpl::get_instance();
    let mut game_hand_repository_guard = game_hand_repository_mutex.lock().await;
    game_hand_repository_guard.create_game_hand_object(user_id);
    println!("game_hand_map: {:?}", game_hand_repository_guard.get_game_hand_map());
    drop(game_hand_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_field_energy_repository_mutex = GameFieldEnergyRepositoryImpl::get_instance();
    let mut game_field_energy_repository_guard = game_field_energy_repository_mutex.lock().await;
    game_field_energy_repository_guard.create_field_energy_object(user_id);
    println!("game_field_energy_map: {:?}", game_field_energy_repository_guard.get_game_field_energy_map());
    drop(game_field_energy_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_field_unit_repository_mutex = GameFieldUnitRepositoryImpl::get_instance();
    let mut game_field_unit_repository_guard = game_field_unit_repository_mutex.lock().await;
    game_field_unit_repository_guard.create_game_field_unit_object(user_id);
    println!("game_field_unit_map: {:?}", game_field_unit_repository_guard.get_game_field_unit_map());
    drop(game_field_unit_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_lost_zone_repository_mutex = GameLostZoneRepositoryImpl::get_instance();
    let mut game_lost_zone_repository_guard = game_lost_zone_repository_mutex.lock().await;
    game_lost_zone_repository_guard.create_game_lost_zone_object(user_id);
    println!("game_lost_zone_map: {:?}", game_lost_zone_repository_guard.get_game_lost_zone_map());
    drop(game_lost_zone_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_main_character_repository_mutex = GameMainCharacterRepositoryImpl::get_instance();
    let mut game_main_character_repository_guard = game_main_character_repository_mutex.lock().await;
    game_main_character_repository_guard.create_game_main_character_object(user_id);
    println!("game_main_character_map: {:?}", game_main_character_repository_guard.get_game_main_character_map());
    drop(game_main_character_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_tomb_repository_mutex = GameTombRepositoryImpl::get_instance();
    let mut game_tomb_repository_guard = game_tomb_repository_mutex.lock().await;
    game_tomb_repository_guard.create_game_tomb_object(user_id);
    println!("game_tomb_map: {:?}", game_tomb_repository_guard.get_game_tomb_map());
    drop(game_tomb_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_round_repository_mutex = GameRoundRepositoryImpl::get_instance();
    let mut game_round_repository_guard = game_round_repository_mutex.lock().await;
    game_round_repository_guard.create_game_round_object(user_id);
    println!("game_round_map: {:?}", game_round_repository_guard.get_game_round_map());
    drop(game_round_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let battle_ready_account_hash_repository_mutex = BattleReadyAccountHashRepositoryImpl::get_instance();
    let mut battle_ready_account_hash_repository_guard = battle_ready_account_hash_repository_mutex.lock().await;
    battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(user_id, BattleReadyAccountHashStatus::BATTLE_FINISH).await;
    drop(battle_ready_account_hash_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_turn_repository_mutex = GameTurnRepositoryImpl::get_instance();
    let mut game_turn_repository_guard = game_turn_repository_mutex.lock().await;
    game_turn_repository_guard.create_game_turn_object(user_id);
    println!("game_turn_map: {:?}", game_turn_repository_guard.get_game_turn_map());
    drop(game_turn_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let game_card_support_usage_counter_repository_mutex = GameCardSupportUsageCounterRepositoryImpl::get_instance();
    let mut game_card_support_usage_counter_repository_guard = game_card_support_usage_counter_repository_mutex.lock().await;
    game_card_support_usage_counter_repository_guard.create_support_card_usage_counter_object(user_id);
    println!("game_card_support_usage_counter_map: {:?}", game_card_support_usage_counter_repository_guard.check_support_card_usage_counter(user_id));
    drop(game_card_support_usage_counter_repository_guard);

    sleep(time::Duration::from_millis(300)).await;

    let battle_ready_account_hash_repository_mutex = BattleReadyAccountHashRepositoryImpl::get_instance();
    let mut battle_ready_account_hash_repository_guard = battle_ready_account_hash_repository_mutex.lock().await;
    battle_ready_account_hash_repository_guard.remove_battle_ready_account_hash_status_hash_by_account_unique_id(user_id);
    println!("battle_ready_account_hash: {:?}", battle_ready_account_hash_repository_guard.get_battle_ready_account_hash());
    drop(battle_ready_account_hash_repository_guard);

    sleep(time::Duration::from_millis(300)).await;


    // Spawn the asynchronous task
    let task = tokio::spawn(async move {
        let battle_finish_service = BattleFinishServiceImpl::get_instance();
        let battle_finish_service_guard = battle_finish_service.lock().await;

        battle_finish_service_guard
            .battle_finish_for_player_battle()
            .await;
    });

    tokio::time::sleep(time::Duration::from_secs(5)).await;

    task.abort();
}