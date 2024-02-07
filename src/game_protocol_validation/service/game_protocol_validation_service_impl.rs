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
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_protocol_validation::service::request::support_card_protocol_validation_request::SupportCardProtocolValidationRequest;
use crate::game_protocol_validation::service::response::can_use_card_response::CanUseCardResponse;
use crate::game_protocol_validation::service::response::check_protocol_hacking_response::CheckProtocolHackingResponse;
use crate::game_protocol_validation::service::response::is_it_support_card_response::IsItSupportCardResponse;
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

    async fn get_user_round_value(&self, can_use_card_request: &CanUseCardRequest) -> Option<i32> {
        let mut game_round_repository_guard = self.game_round_repository.lock().await;
        game_round_repository_guard
            .get_game_round_map()
            .get(&can_use_card_request.get_account_unique_id())
            .map(|user_round| user_round.get_round())
    }

    async fn can_use_mythical_card(&self, can_use_card_request: &CanUseCardRequest, round: i32) -> bool {
        if let target_card_number = can_use_card_request.get_support_card_number() {
            let card_grade_repository_guard = self.card_grade_repository.lock().await;
            if let Some(card_grade) = card_grade_repository_guard.get_card_grade(&target_card_number).await {
                return card_grade == GradeEnum::Mythical as i32 && round == 4;
            }
        }

        false
    }
}

#[async_trait]
impl GameProtocolValidationService for GameProtocolValidationServiceImpl {

    async fn check_protocol_hacking(&mut self, support_card_protocol_validation_request: CheckProtocolHackingRequest) -> CheckProtocolHackingResponse {
        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let game_hand = game_hand_repository_guard.get_game_hand_map().get(&support_card_protocol_validation_request.get_account_unique_id());

        if game_hand.is_none() {
            println!("핸드 자체가 없습니다!");
            return CheckProtocolHackingResponse::new(false)
        }

        let mut result = true;
        let target_card_number = support_card_protocol_validation_request.get_support_card_number();

        for &target_card in game_hand.unwrap().get_all_card_list_in_game_hand().iter() {
            if target_card.get_card() == target_card_number {
                result = false;
                break;
            }
        }

        CheckProtocolHackingResponse::new(result)
    }

    async fn can_use_card(&mut self, can_use_card_request: CanUseCardRequest) -> CanUseCardResponse {
        let round_option = self.get_user_round_value(&can_use_card_request).await;
        let round = round_option.unwrap();

        if !self.can_use_mythical_card(&can_use_card_request, round).await {
            return CanUseCardResponse::new(false)
        }

        CanUseCardResponse::new(true)
    }

    async fn is_it_support_card(&self, is_it_support_card_request: IsItSupportCardRequest) -> IsItSupportCardResponse {
        let support_card_number = is_it_support_card_request.get_support_card_number();

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        if let maybe_support_card = card_kinds_repository_guard.get_card_kind(&support_card_number).await {
            return IsItSupportCardResponse::new(maybe_support_card == KindsEnum::Support)
        }

        IsItSupportCardResponse::new(false)
    }
}