use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;

use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::card_grade::repository::card_grade_repository_impl::CardGradeRepositoryImpl;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::card_race::repository::card_race_repository::CardRaceRepository;
use crate::card_race::repository::card_race_repository_impl::CardRaceRepositoryImpl;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::common::converter::vector_string_to_vector_integer::VectorStringToVectorInteger;
use crate::game_deck::entity::game_deck_card::GameDeckCard;
use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_hand::repository::game_hand_repository::GameHandRepository;

use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::request::put_cards_on_deck_request::{PutCardsOnDeckRequest};

use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_hand::service::request::use_game_hand_energy_card_request::UseGameHandEnergyCardRequest;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_hand::service::request::use_game_hand_tool_card_request::UseGameHandToolCardRequest;
use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;
use crate::game_hand::service::response::put_cards_on_deck_response::PutCardsOnDeckResponse;
use crate::game_hand::service::response::use_game_hand_energy_card_response::UseGameHandEnergyCardResponse;
use crate::game_hand::service::response::use_game_hand_item_card_response::UseGameHandItemCardResponse;
use crate::game_hand::service::response::use_game_hand_support_card_response::UseGameHandSupportCardResponse;
use crate::game_hand::service::response::use_game_hand_tool_card_response::UseGameHandToolCardResponse;
use crate::game_hand::service::response::use_game_hand_unit_card_response::UseGameHandUnitCardResponse;
use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;
use crate::game_tomb::repository::game_tomb_repository::GameTombRepository;
use crate::game_tomb::repository::game_tomb_repository_impl::GameTombRepositoryImpl;
use crate::notify_player_action::repository::notify_player_action_repository::NotifyPlayerActionRepository;
use crate::notify_player_action::repository::notify_player_action_repository_impl::NotifyPlayerActionRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameHandServiceImpl {
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
    card_race_repository: Arc<AsyncMutex<CardRaceRepositoryImpl>>,
    battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
    notify_player_action_repository: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl GameHandServiceImpl {
    pub fn new(game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
               game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
               card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
               card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
               card_race_repository: Arc<AsyncMutex<CardRaceRepositoryImpl>>,
               battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
               notify_player_action_repository: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    ) -> Self {
        GameHandServiceImpl {
            game_round_repository,
            game_hand_repository,
            game_deck_repository,
            game_field_unit_repository,
            game_tomb_repository,
            card_kinds_repository,
            card_grade_repository,
            card_race_repository,
            battle_room_repository,
            notify_player_action_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameHandServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameHandServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameHandServiceImpl::new(
                            GameRoundRepositoryImpl::get_instance(),
                            GameHandRepositoryImpl::get_instance(),
                            GameDeckRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance(),
                            GameTombRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance(),
                            CardGradeRepositoryImpl::get_instance(),
                            CardRaceRepositoryImpl::get_instance(),
                            BattleRoomRepositoryImpl::get_instance(),
                            NotifyPlayerActionRepositoryImpl::get_instance(),
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

    // TODO: 해킹 감지 (3회 이상 계정 영구 정지, 1회, 2회 -> 1시간 접속 불가)
    async fn check_protocol_hacking(&mut self, account_unique_id: i32, unit_card_number: i32) -> bool {
        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let game_hand = game_hand_repository_guard.get_game_hand_map().get(&account_unique_id);

        if game_hand.is_none() {
            println!("핸드 자체가 없습니다!");
            return true
        }

        let mut result = true;

        for &unit_card in game_hand.unwrap().get_all_card_list_in_game_hand().iter() {
            if unit_card.get_card() == unit_card_number {
                result = false;
                break;
            }
        }

        result
    }



    async fn convert_race_string_to_enum(race_str: &str) -> RaceEnumValue {
        match race_str.to_lowercase().as_str() {
            "언데드" => RaceEnumValue::Undead,
            "휴먼" => RaceEnumValue::Human,
            "천사" => RaceEnumValue::Angel,
            "기계" => RaceEnumValue::Machine,
            "트랜트" => RaceEnumValue::Trent,
            _ => panic!("Invalid race string"),
        }
    }
}

#[async_trait]
impl GameHandService for GameHandServiceImpl {
    async fn put_hand_cards_to_deck(&mut self, request: PutCardsOnDeckRequest) -> PutCardsOnDeckResponse {
        // account_unique_id setting
        let session_id = request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        // Vec<i32> card list setting
        let card_string_list_to_be_changed = request.get_will_be_changed_card_list().clone();
        let card_i32_list_to_be_changed =
            VectorStringToVectorInteger::vector_string_to_vector_i32(card_string_list_to_be_changed);

        // removing hand cards
        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        if let Some(user_game_hand) =
            game_hand_repository_guard.get_game_hand_map().get_mut(&account_unique_id) {
            user_game_hand.remove_card_list_from_hand(card_i32_list_to_be_changed.clone());
        } else {
            println!("User game hand is not found.");
            return PutCardsOnDeckResponse::new(false)
        }

        // adding deck cards
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        if let Some(user_game_deck) =
            game_deck_repository_guard.get_game_deck_map().get_mut(&account_unique_id) {
            for card in card_i32_list_to_be_changed {
                let game_deck_card = GameDeckCard::new(card);
                user_game_deck.add_card_to_game_deck(game_deck_card);
            }
        } else {
            println!("User game deck is not found.");
            return PutCardsOnDeckResponse::new(false)
        }

        PutCardsOnDeckResponse::new(true)
    }

    async fn use_support_card(&mut self, use_game_hand_support_card_request: UseGameHandSupportCardRequest) -> UseGameHandSupportCardResponse {
        println!("GameHandServiceImpl: use_support_card()");

        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let specific_card_option = game_hand_repository_guard.use_specific_card(
            use_game_hand_support_card_request.get_account_unique_id(),
            use_game_hand_support_card_request.get_support_card_id());

        if specific_card_option.is_none() {
            return UseGameHandSupportCardResponse::new(-1)
        }
        let specific_card = specific_card_option.unwrap();

        UseGameHandSupportCardResponse::new(specific_card.get_card())
    }

    async fn use_unit_card(&mut self, use_game_hand_unit_card_request: UseGameHandUnitCardRequest) -> UseGameHandUnitCardResponse {
        println!("GameHandServiceImpl: use_unit_card()");

        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let maybe_unit_card = game_hand_repository_guard.use_specific_card(
            use_game_hand_unit_card_request.get_account_unique_id(),
            use_game_hand_unit_card_request.get_unit_card_id());

        if maybe_unit_card.is_none() {
            return UseGameHandUnitCardResponse::new(-1)
        }
        let unit_card = maybe_unit_card.unwrap();

        UseGameHandUnitCardResponse::new(unit_card.get_card())
    }

    async fn use_energy_card(&mut self, use_game_hand_energy_card_request: UseGameHandEnergyCardRequest) -> UseGameHandEnergyCardResponse {
        println!("GameHandServiceImpl: use_unit_card()");

        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let maybe_energy_card = game_hand_repository_guard.use_specific_card(
            use_game_hand_energy_card_request.get_account_unique_id(),
            use_game_hand_energy_card_request.get_energy_card_id());

        if maybe_energy_card.is_none() {
            return UseGameHandEnergyCardResponse::new(-1)
        }
        let energy_card = maybe_energy_card.unwrap();

        UseGameHandEnergyCardResponse::new(energy_card.get_card())
    }

    async fn use_item_card(&mut self, use_game_hand_item_card_request: UseGameHandItemCardRequest) -> UseGameHandItemCardResponse {
        println!("GameHandServiceImpl: use_unit_card()");

        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let maybe_item_card = game_hand_repository_guard.use_specific_card(
            use_game_hand_item_card_request.get_account_unique_id(),
            use_game_hand_item_card_request.get_item_card_id());

        if maybe_item_card.is_none() {
            return UseGameHandItemCardResponse::new(-1)
        }
        let item_card = maybe_item_card.unwrap();

        UseGameHandItemCardResponse::new(item_card.get_card())
    }

    async fn use_tool_card(&mut self, use_game_hand_tool_card_request: UseGameHandToolCardRequest) -> UseGameHandToolCardResponse {
        println!("GameHandServiceImpl: use_tool_card()");

        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let specific_card_option = game_hand_repository_guard.use_specific_card(
            use_game_hand_tool_card_request.get_account_unique_id(),
            use_game_hand_tool_card_request.get_tool_card_id());

        if specific_card_option.is_none() {
            return UseGameHandToolCardResponse::new(-1)
        }
        let specific_card = specific_card_option.unwrap();

        UseGameHandToolCardResponse::new(specific_card.get_card())
    }
}