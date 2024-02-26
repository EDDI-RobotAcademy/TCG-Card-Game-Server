use std::sync::Arc;
use async_trait::async_trait;
use diesel::IntoSql;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use tokio::time;
use tokio::time::sleep;
use uuid::Uuid;

use crate::account::repository::account_repository::AccountRepository;
use crate::account::repository::account_repository_impl::AccountRepositoryImpl;
use crate::account::service::account_service::AccountService;
use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::account_point::repository::account_point_repository::AccountPointRepository;
use crate::account_point::repository::account_point_repository_impl::AccountPointRepositoryImpl;

use crate::battle_finish::service::battle_finish_service::BattleFinishService;
use crate::battle_finish::service::request::battle_finish_request::BattleFinishRequest;
use crate::battle_finish::service::response::battle_finish_response::BattleFinishResponse;

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
use crate::game_winner_check::entity::finish_position_enum::FinishPositionEnum::{Draw, Loser, Winner};
use crate::game_winner_check::repository::game_winner_check_repository::GameWinnerCheckRepository;
use crate::game_winner_check::repository::game_winner_check_repository_impl::GameWinnerCheckRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

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
    account_point_repository: Arc<AsyncMutex<AccountPointRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    game_winner_check_repository: Arc<AsyncMutex<GameWinnerCheckRepositoryImpl>>,
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
               game_card_support_usage_counter_repository: Arc<AsyncMutex<GameCardSupportUsageCounterRepositoryImpl>>,
               account_point_repository: Arc<AsyncMutex<AccountPointRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               game_winner_check_repository: Arc<AsyncMutex<GameWinnerCheckRepositoryImpl>>, ) -> Self {
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
            game_card_support_usage_counter_repository,
            account_point_repository,
            redis_in_memory_repository,
            game_winner_check_repository,
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
                            GameCardSupportUsageCounterRepositoryImpl::get_instance(),
                            AccountPointRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
                            GameWinnerCheckRepositoryImpl::get_instance())));
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

// TODO :: battle_ready_account_hash 삭제 기능 구현 필요
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
    // TODO :: battle_ready_account_hash 삭제 기능 구현 필요
    // let task_battle_ready_account_hash_status_remove = tokio::spawn(player_battle_ready_account_hash_config_thread(user_id));

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
        // TODO :: battle_ready_account_hash 삭제 기능 구현 필요
        // task_battle_ready_account_hash_status_remove,
    );
}

#[async_trait]
impl BattleFinishService for BattleFinishServiceImpl {
    async fn battle_finish_for_player_battle(&self, battle_finish_request: BattleFinishRequest) -> BattleFinishResponse {
        println!("BattleFinishServiceImpl: battle_finish_for_player_battle()");

        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_unique_id_sting = redis_repository_guard.get(battle_finish_request.get_session_id()).await.unwrap();
        let user_id: i32 = account_unique_id_sting.parse().expect("Failed to parse account_unique_id_string as i32");
        println!("battle_finish_for_player_battle_user_id: {:?}", user_id);
        drop(redis_repository_guard);

        let mut reward_gold: i32 = 0;
        // let mut finish_position_user_id = battle_finish_request.get_finish_position().clone();
        let mut game_winner_check_repository_guard = self.game_winner_check_repository.lock().await;
        let mut user_finish_position = game_winner_check_repository_guard.get_finish_position_enum(user_id).unwrap();
        if *user_finish_position == Winner {
            println!("Winner 보상 100 gold");
            reward_gold = 100;
        } else if *user_finish_position == Loser {
            println!("Loser 보상 50 gold");
            reward_gold = 50;
        } else if *user_finish_position == Draw {
            println!("Draw 보상 70 gold");
            reward_gold = 70;
        } else {
            println!("finish_position_enum_error");
        }

        let mut account_point_repository_guard = self.account_point_repository.lock().await;

        let found_account_point = account_point_repository_guard.find_by_account_id(user_id).await.unwrap().unwrap();
        let current_gold = found_account_point.gold;
        let result_gold = current_gold + reward_gold;
        let result_reward = account_point_repository_guard.update_gold(found_account_point, result_gold).await;

        println!("BattleFinishServiceImpl: spawn_async_task_for_finish_battle()");
        let result_task_for_finish_battle = tokio::spawn(spawn_async_task_for_finish_battle(user_id)).await;

        if result_task_for_finish_battle.is_ok() && result_reward.is_ok() {
            return BattleFinishResponse::new(true)
        }
        return BattleFinishResponse::new(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::account::service::request::account_register_request::AccountRegisterRequest;
    use super::*;

    #[tokio::test]
    async fn test_battle_finish_for_player_battle() {
        // test 계정 생성
        let account_service = AccountServiceImpl::get_instance();
        let account_service_guard = account_service.lock().await;

        let test_account_id = "reward_test9";
        let test_account_pw = "reward_test9";
        let string_test_account_pw = test_account_pw.to_string();

        let test_account = AccountRegisterRequest::new(test_account_id, string_test_account_pw);
        account_service_guard.account_register(test_account).await;
        drop(account_service_guard);

        // redis_token 생성(로그인 과정) - Account, redis_token 필요
        let mut account_repository_mutex = AccountRepositoryImpl::get_instance();
        let mut account_repository_guard = account_repository_mutex.lock().await;

        let found_account = account_repository_guard.find_by_user_id(test_account_id).await.unwrap().unwrap();
        drop(account_repository_guard);

        let mut redis_repository_mutex = RedisInMemoryRepositoryImpl::get_instance();
        let mut redis_repository_guard = redis_repository_mutex.lock().await;

        let redis_token_account_id = Uuid::new_v4();
        redis_repository_guard.set_with_expired_time(&*redis_token_account_id.to_string(), &found_account.id.to_string(), Some(60)).await;
        drop(redis_repository_guard);

        // redis key 값으로 user_id 획득
        let mut redis_repository_mutex = RedisInMemoryRepositoryImpl::get_instance();
        let mut redis_repository_guard = redis_repository_mutex.lock().await;
        let account_unique_id_sting = redis_repository_guard.get(&redis_token_account_id.to_string()).await.unwrap();
        let user_id: i32 = account_unique_id_sting.parse().expect("Failed to parse account_unique_id_string as i32");
        drop(redis_repository_guard);

        // test 를 위해 각종 정보 생성
        // game_deck 정보 생성
        let game_deck_repository_mutex = GameDeckRepositoryImpl::get_instance();
        let mut game_deck_repository_guard = game_deck_repository_mutex.lock().await;
        game_deck_repository_guard.create_game_deck_object(user_id);
        println!("game_deck_map: {:?}", game_deck_repository_guard.get_game_deck_map());
        drop(game_deck_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_hand 정보 생성
        let game_hand_repository_mutex = GameHandRepositoryImpl::get_instance();
        let mut game_hand_repository_guard = game_hand_repository_mutex.lock().await;
        game_hand_repository_guard.create_game_hand_object(user_id);
        println!("game_hand_map: {:?}", game_hand_repository_guard.get_game_hand_map());
        drop(game_hand_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_field_energy 정보 생성
        let game_field_energy_repository_mutex = GameFieldEnergyRepositoryImpl::get_instance();
        let mut game_field_energy_repository_guard = game_field_energy_repository_mutex.lock().await;
        game_field_energy_repository_guard.create_field_energy_object(user_id);
        println!("game_field_energy_map: {:?}", game_field_energy_repository_guard.get_game_field_energy_map());
        drop(game_field_energy_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_field_unit 정보 생성
        let game_field_unit_repository_mutex = GameFieldUnitRepositoryImpl::get_instance();
        let mut game_field_unit_repository_guard = game_field_unit_repository_mutex.lock().await;
        game_field_unit_repository_guard.create_game_field_unit_object(user_id);
        println!("game_field_unit_map: {:?}", game_field_unit_repository_guard.get_game_field_unit_map());
        drop(game_field_unit_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_lost_zone 정보 생성
        let game_lost_zone_repository_mutex = GameLostZoneRepositoryImpl::get_instance();
        let mut game_lost_zone_repository_guard = game_lost_zone_repository_mutex.lock().await;
        game_lost_zone_repository_guard.create_game_lost_zone_object(user_id);
        println!("game_lost_zone_map: {:?}", game_lost_zone_repository_guard.get_game_lost_zone_map());
        drop(game_lost_zone_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_main_character 정보 생성
        let game_main_character_repository_mutex = GameMainCharacterRepositoryImpl::get_instance();
        let mut game_main_character_repository_guard = game_main_character_repository_mutex.lock().await;
        game_main_character_repository_guard.create_game_main_character_object(user_id);
        println!("game_main_character_map: {:?}", game_main_character_repository_guard.get_game_main_character_map());
        drop(game_main_character_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_tomb 정보 생성
        let game_tomb_repository_mutex = GameTombRepositoryImpl::get_instance();
        let mut game_tomb_repository_guard = game_tomb_repository_mutex.lock().await;
        game_tomb_repository_guard.create_game_tomb_object(user_id);
        println!("game_tomb_map: {:?}", game_tomb_repository_guard.get_game_tomb_map());
        drop(game_tomb_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_round 정보 생성
        let game_round_repository_mutex = GameRoundRepositoryImpl::get_instance();
        let mut game_round_repository_guard = game_round_repository_mutex.lock().await;
        game_round_repository_guard.create_game_round_object(user_id);
        println!("game_round_map: {:?}", game_round_repository_guard.get_game_round_map());
        drop(game_round_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // battle_ready_account_hash를 BATTLE_FINISH 로 설정
        let battle_ready_account_hash_repository_mutex = BattleReadyAccountHashRepositoryImpl::get_instance();
        let mut battle_ready_account_hash_repository_guard = battle_ready_account_hash_repository_mutex.lock().await;
        battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(user_id, BattleReadyAccountHashStatus::BATTLE_FINISH).await;
        println!("battle_ready_account_hash: {:?}", battle_ready_account_hash_repository_guard.get_battle_ready_account_hash());
        drop(battle_ready_account_hash_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_turn 정보 생성
        let game_turn_repository_mutex = GameTurnRepositoryImpl::get_instance();
        let mut game_turn_repository_guard = game_turn_repository_mutex.lock().await;
        game_turn_repository_guard.create_game_turn_object(user_id);
        println!("game_turn_map: {:?}", game_turn_repository_guard.get_game_turn_map());
        drop(game_turn_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // game_card_support_usage_counter 정보 생성
        let game_card_support_usage_counter_repository_mutex = GameCardSupportUsageCounterRepositoryImpl::get_instance();
        let mut game_card_support_usage_counter_repository_guard = game_card_support_usage_counter_repository_mutex.lock().await;
        game_card_support_usage_counter_repository_guard.create_support_card_usage_counter_object(user_id);
        println!("game_card_support_usage_counter_map: {:?}", game_card_support_usage_counter_repository_guard.check_support_card_usage_counter(user_id));
        drop(game_card_support_usage_counter_repository_guard);

        sleep(time::Duration::from_millis(300)).await;

        // 보상 전 골드 확인
        let account_point_repository = AccountPointRepositoryImpl::get_instance();
        let account_point_repository_guard = account_point_repository.lock().await;
        let account_point_test = account_point_repository_guard.find_by_account_id(user_id).await.unwrap();
        println!("before_user_id: {:?}", user_id);
        let account_gold_test = account_point_test.unwrap().gold;

        println!("account_gold_before_reward: {:?}", account_gold_test);
        drop(account_point_repository_guard);

        // battle_finish_for_player_battle 메서드 test 구동
        // Spawn the asynchronous task
        let task = tokio::spawn(async move {
            let battle_finish_service = BattleFinishServiceImpl::get_instance();
            let battle_finish_service_guard = battle_finish_service.lock().await;

            let test_request = BattleFinishRequest::new(redis_token_account_id.to_string(), Winner);
            println!("test_request: {:?}", test_request);
            battle_finish_service_guard
                .battle_finish_for_player_battle(test_request)
                .await;
        });

        sleep(time::Duration::from_secs(5)).await;
        task.abort();

        // 보상 후 gold 결과 확인
        let account_point_repository = AccountPointRepositoryImpl::get_instance();
        let account_point_repository_guard = account_point_repository.lock().await;
        let account_point_test = account_point_repository_guard.find_by_account_id(user_id).await.unwrap();
        println!("after_user_id: {:?}", user_id);
        let account_gold_test_after = account_point_test.unwrap().gold;

        println!("account_gold_after_reward: {:?}", account_gold_test_after);
        drop(account_point_repository_guard);



    }
}