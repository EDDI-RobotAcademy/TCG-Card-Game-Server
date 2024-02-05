use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::card_grade::repository::card_grade_repository_impl::CardGradeRepositoryImpl;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_hand::repository::game_hand_repository::GameHandRepository;

use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::request::use_game_hand_energy_card_request::UseGameHandEnergyCardRequest;
use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;
use crate::game_hand::service::response::use_game_hand_energy_card_response::UseGameHandEnergyCardResponse;
use crate::game_hand::service::response::use_game_hand_unit_card_response::UseGameHandUnitCardResponse;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameHandServiceImpl {
    game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl GameHandServiceImpl {
    pub fn new(game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
               card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
               card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    ) -> Self {
        GameHandServiceImpl {
            game_turn_repository,
            game_hand_repository,
            game_field_unit_repository,
            card_kinds_repository,
            card_grade_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameHandServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameHandServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameHandServiceImpl::new(
                            GameTurnRepositoryImpl::get_instance(),
                            GameHandRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance(),
                            CardGradeRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn get_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
        account_unique_id
    }

    async fn check_protocol_hacking(&mut self, account_unique_id: i32, unit_card_number: i32) -> bool {
        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let maybe_exist_hand_unit = game_field_unit_repository_guard.find_unit_by_id(account_unique_id, unit_card_number);

        if maybe_exist_hand_unit.is_none() {
            // TODO: 해킹 감지 (3회 이상 계정 영구 정지, 1회, 2회 -> 1시간 접속 불가)
            return true;
        }

        return false
    }
}

#[async_trait]
impl GameHandService for GameHandServiceImpl {
    async fn use_specific_card(&mut self, use_game_hand_unit_card_request: UseGameHandUnitCardRequest) -> UseGameHandUnitCardResponse {
        println!("GameHandServiceImpl: use_specific_card()");

        let session_id = use_game_hand_unit_card_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let unit_card_number_string = use_game_hand_unit_card_request.get_unit_number();
        let unit_card_number = unit_card_number_string.parse::<i32>().unwrap();

        if self.check_protocol_hacking(account_unique_id, unit_card_number).await {
            println!("프로토콜 조작 감지: 해킹범을 검거합시다!");
            return UseGameHandUnitCardResponse::new(false)
        }

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        let maybe_unit_card = card_kinds_repository_guard.get_card_kind(&unit_card_number).await;
        if maybe_unit_card.unwrap() != "유닛" {
            return UseGameHandUnitCardResponse::new(false)
        }

        let card_grade_repository_guard = self.card_grade_repository.lock().await;
        if card_grade_repository_guard.get_card_grade(&unit_card_number).await.unwrap() == "신화".to_string() {
            let mut game_turn_repository_guard = self.game_turn_repository.lock().await;
            let user_turn = game_turn_repository_guard.get_game_turn_map().get(&account_unique_id).unwrap();
            if user_turn.get_turn() <= 4 {
                println!("신화 유닛은 현재 사용 불가");
                return UseGameHandUnitCardResponse::new(false)
            }
        }

        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let specific_card_option = game_hand_repository_guard.use_specific_card(account_unique_id, unit_card_number);
        let specific_card = specific_card_option.unwrap();

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        game_field_unit_repository_guard.add_unit_to_game_field(account_unique_id, specific_card.get_card());

        UseGameHandUnitCardResponse::new(true)
    }

    async fn attach_energy_card_to_field_unit(&mut self, use_game_hand_energy_card_request: UseGameHandEnergyCardRequest) -> UseGameHandEnergyCardResponse {
        println!("GameHandServiceImpl: attach_energy_card_to_field_unit()");

        let session_id = use_game_hand_energy_card_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let unit_card_number_string = use_game_hand_energy_card_request.get_unit_number();
        let unit_card_number = unit_card_number_string.parse::<i32>().unwrap();

        let energy_card_id_string = use_game_hand_energy_card_request.get_energy_card_id();
        let energy_card_id = energy_card_id_string.parse::<i32>().unwrap();

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        let maybe_energy_card = card_kinds_repository_guard.get_card_kind(&energy_card_id).await;
        if maybe_energy_card.unwrap() != "에너지" {
            return UseGameHandEnergyCardResponse::new(false)
        }

        let maybe_unit_card = card_kinds_repository_guard.get_card_kind(&unit_card_number).await;
        if maybe_unit_card.unwrap() != "유닛" {
            return UseGameHandEnergyCardResponse::new(false)
        }

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        game_field_unit_repository_guard.attach_energy_to_game_field_unit(account_unique_id, unit_card_number);

        UseGameHandEnergyCardResponse::new(true)
    }
}