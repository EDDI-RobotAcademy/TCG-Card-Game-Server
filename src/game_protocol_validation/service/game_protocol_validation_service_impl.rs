use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::card_grade::repository::card_grade_repository_impl::CardGradeRepositoryImpl;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;
use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::game_hand::service::response::use_game_hand_unit_card_response::UseGameHandUnitCardResponse;
use crate::game_protocol_validation::repository::game_protocol_validation_repository_impl::GameProtocolValidationRepositoryImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::request::support_card_protocol_validation_request::SupportCardProtocolValidationRequest;
use crate::game_protocol_validation::service::response::support_card_protocol_validation_response::SupportCardProtocolValidationResponse;
use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameProtocolValidationServiceImpl {
    game_protocol_validation_repository: Arc<AsyncMutex<GameProtocolValidationRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    // account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl GameProtocolValidationServiceImpl {
    pub fn new(game_protocol_validation_repository: Arc<AsyncMutex<GameProtocolValidationRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               // account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
               game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
               card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
               card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {

        GameProtocolValidationServiceImpl {
            game_protocol_validation_repository,
            game_hand_repository,
            // account_deck_card_repository,
            game_round_repository,
            card_kinds_repository,
            card_grade_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameProtocolValidationServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameProtocolValidationServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameProtocolValidationServiceImpl::new(
                            GameProtocolValidationRepositoryImpl::get_instance(),
                            GameHandRepositoryImpl::get_instance(),
                            // AccountDeckCardRepositoryImpl::get_instance(),
                            GameRoundRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance(),
                            CardGradeRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_it_support_card(&self, use_support_card_request: &SupportCardProtocolValidationRequest) -> bool {
        let support_card_number = match use_support_card_request.get_support_card_number().parse::<i32>() {
            Ok(number) => number,
            Err(_) => return false,
        };

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        if let Some(maybe_support_card) = card_kinds_repository_guard.get_card_kind(&support_card_number).await {
            maybe_support_card == KindsEnum::Support as i32
        } else {
            false
        }
    }

    async fn get_user_round_value(&self, support_card_protocol_validation_request: &SupportCardProtocolValidationRequest) -> Option<i32> {
        let mut game_round_repository_guard = self.game_round_repository.lock().await;
        game_round_repository_guard
            .get_game_round_map()
            .get(&support_card_protocol_validation_request.get_account_unique_id())
            .map(|user_round| user_round.get_round())
    }

    async fn can_use_mythical_card(&self, support_card_protocol_validation_request: &SupportCardProtocolValidationRequest, round: i32) -> bool {
        if let Ok(support_card_number) = support_card_protocol_validation_request.get_support_card_number().parse::<i32>() {
            let card_grade_repository_guard = self.card_grade_repository.lock().await;
            if let Some(card_grade) = card_grade_repository_guard.get_card_grade(&support_card_number).await {
                return card_grade == GradeEnum::Mythical as i32 && round == 4;
            }
        }

        false
    }

    async fn check_protocol_hacking(&mut self, support_card_protocol_validation_request: &SupportCardProtocolValidationRequest) -> bool {
        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let game_hand = game_hand_repository_guard.get_game_hand_map().get(&support_card_protocol_validation_request.get_account_unique_id());

        if game_hand.is_none() {
            println!("핸드 자체가 없습니다!");
            return true
        }

        let mut result = true;

        let target_card_number = match support_card_protocol_validation_request.get_support_card_number().parse::<i32>() {
            Ok(number) => number,
            Err(_) => return false,
        };

        for &target_card in game_hand.unwrap().get_all_card_list_in_game_hand().iter() {
            if target_card.get_card() == target_card_number {
                result = false;
                break;
            }
        }

        result
    }
}

#[async_trait]
impl GameProtocolValidationService for GameProtocolValidationServiceImpl {
    async fn support_card_protocol_validation(&mut self, support_card_validation_request: SupportCardProtocolValidationRequest) -> SupportCardProtocolValidationResponse {
        println!("GameProtocolValidationServiceImpl: support_card_protocol_validation()");

        if !self.is_it_support_card(&support_card_validation_request).await {
            return SupportCardProtocolValidationResponse::new(false)
        }

        let round_option = self.get_user_round_value(&support_card_validation_request).await;
        let round = round_option.unwrap();

        if !self.can_use_mythical_card(&support_card_validation_request, round).await {
            return SupportCardProtocolValidationResponse::new(false)
        }

        if self.check_protocol_hacking(&support_card_validation_request).await {
            println!("프로토콜 조작 감지: 해킹범을 검거합시다!");
            return SupportCardProtocolValidationResponse::new(false)
        }

        SupportCardProtocolValidationResponse::new(true)
    }
}