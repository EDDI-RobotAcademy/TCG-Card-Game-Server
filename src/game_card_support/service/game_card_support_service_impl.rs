use std::ops::Deref;
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
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::repository::game_card_support_repository::GameCardSupportRepository;
use crate::game_card_support::repository::game_card_support_repository_impl::GameCardSupportRepositoryImpl;
use crate::game_card_support::service::game_card_support_service::GameCardSupportService;
use crate::game_card_support::service::request::calculate_effect_request::CalculateEffectRequest;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::calculate_effect_response::CalculateEffectResponse;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;
use crate::game_deck::repository::game_deck_repository::GameDeckRepository;
use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_hand::entity::game_hand_card::GameHandCard;
use crate::game_hand::repository::game_hand_repository::GameHandRepository;
use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;
use crate::game_tomb::repository::game_tomb_repository::GameTombRepository;
use crate::game_tomb::repository::game_tomb_repository_impl::GameTombRepositoryImpl;
use crate::notify_player_action::repository::notify_player_action_repository::NotifyPlayerActionRepository;
use crate::notify_player_action::repository::notify_player_action_repository_impl::NotifyPlayerActionRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameCardSupportServiceImpl {
    game_card_support_repository: Arc<AsyncMutex<GameCardSupportRepositoryImpl>>,
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
    battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
    notify_player_action_repository: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl GameCardSupportServiceImpl {
    pub fn new(game_card_support_repository: Arc<AsyncMutex<GameCardSupportRepositoryImpl>>,
               game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
               card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
               card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
               battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
               notify_player_action_repository: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    ) -> Self {
        GameCardSupportServiceImpl {
            game_card_support_repository,
            game_round_repository,
            game_hand_repository,
            game_deck_repository,
            game_tomb_repository,
            game_field_unit_repository,
            card_kinds_repository,
            card_grade_repository,
            battle_room_repository,
            notify_player_action_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportServiceImpl::new(
                            GameCardSupportRepositoryImpl::get_instance(),
                            GameRoundRepositoryImpl::get_instance(),
                            GameHandRepositoryImpl::get_instance(),
                            GameDeckRepositoryImpl::get_instance(),
                            GameTombRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance(),
                            CardGradeRepositoryImpl::get_instance(),
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

    async fn is_it_support_card(&self, use_support_card_request: &UseSupportCardRequest) -> bool {
        let support_card_number = match use_support_card_request.get_support_card_number().parse::<i32>() {
            Ok(number) => number,
            Err(_) => return false,
        };

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        if let maybe_support_card = card_kinds_repository_guard.get_card_kind(&support_card_number).await {
            return maybe_support_card == KindsEnum::Support
        }

        false
    }

    async fn get_user_round_value(&self, account_unique_id: i32) -> Option<i32> {
        let mut game_round_repository_guard = self.game_round_repository.lock().await;
        game_round_repository_guard
            .get_game_round_map()
            .get(&account_unique_id)
            .map(|user_round| user_round.get_round())
    }

    async fn can_use_mythical_card(&self, use_support_card_request: &UseSupportCardRequest, round: i32) -> bool {
        if let Ok(support_card_number) = use_support_card_request.get_unit_number().parse::<i32>() {
            let card_grade_repository_guard = self.card_grade_repository.lock().await;
            if let Some(card_grade) = card_grade_repository_guard.get_card_grade(&support_card_number).await {
                return card_grade == GradeEnum::Mythical as i32 && round == 4;
            }
        }

        false
    }

    async fn use_specific_support_card_from_hand(&self, account_unique_id: i32, use_support_card_request: &UseSupportCardRequest) -> Option<GameHandCard> {
        let support_card_number_string = use_support_card_request.get_unit_number();
        let support_card_number = match support_card_number_string.parse::<i32>() {
            Ok(number) => number,
            Err(_) => return None,
        };

        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        game_hand_repository_guard.use_specific_card(account_unique_id, support_card_number)
    }

    async fn get_opponent_unique_id(&self, account_unique_id: i32) -> i32 {
        let battle_room_repository_guard = self.battle_room_repository.lock().await;
        let opponent_unique_id_option = battle_room_repository_guard.find_opponent_unique_id(account_unique_id).await;
        opponent_unique_id_option.unwrap()
    }
}

#[async_trait]
impl GameCardSupportService for GameCardSupportServiceImpl {
    // TODO: Before Refactor (need to chore)
    async fn use_specific_support_card(&mut self, use_support_card_request: UseSupportCardRequest) -> UseSupportCardResponse {
        println!("GameCardSupportServiceImpl: use_specific_support_card()");

        let use_support_card_request_arc = Arc::new(use_support_card_request);
        let use_support_card_request_clone = use_support_card_request_arc.clone();

        let session_id = use_support_card_request_clone.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        if !self.is_it_support_card(&use_support_card_request_clone).await {
            return UseSupportCardResponse::new(false)
        }

        let round_option = self.get_user_round_value(account_unique_id).await;
        let round = round_option.unwrap();

        if !self.can_use_mythical_card(&use_support_card_request_clone, round).await {
            return UseSupportCardResponse::new(false)
        }

        let specific_card_option = self.use_specific_support_card_from_hand(account_unique_id, &use_support_card_request_clone).await;
        let specific_card = specific_card_option.unwrap();

        // TODO: 이 구간 때문에 Controller 구성이 필요함 (일반화가 불가능함)
        let use_support_card_request_value = use_support_card_request_clone.deref().clone();
        let game_card_support_repository_guard = self.game_card_support_repository.lock().await;
        let game_card_support_effect = unsafe { game_card_support_repository_guard.call_support_card_repository_table(use_support_card_request_value) };

        // 덱에서 effect의 수량만큼 찾는다.
        // 만약 2장을 찾는데 1장만 남아 있다면 1장만 가져오고 없다면 못가져온다.
        let need_to_find_card_id = game_card_support_effect.get_need_to_find_card_id();
        let energy_from_deck = game_card_support_effect.get_energy_from_deck();
        let energy_count = energy_from_deck.get_energy_count();

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let found_energy_card_list = game_deck_repository_guard.find_by_card_id_with_count(account_unique_id, need_to_find_card_id, energy_count);
        let real_energy_count = found_energy_card_list.len();

        let unit_card_number_string = use_support_card_request_clone.get_unit_number();
        let unit_card_number = unit_card_number_string.parse::<i32>().unwrap();

        let maybe_energy_from_deck = game_card_support_effect.get_energy_from_deck();
        let race_enum_reference = maybe_energy_from_deck.get_race();
        let race_enum = *race_enum_reference;

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        game_field_unit_repository_guard.attach_multiple_energy_to_game_field_unit(
            account_unique_id, unit_card_number, race_enum as i32, real_energy_count as i32);

        // TODO: UI에서 서포트를 통한 에너지 부스트를 어떻게 표현 할 것인가 ? (붙인 에너지와 수량 관련 사항이 필요한가 ?)
        let opponent_unique_id = self.get_opponent_unique_id(account_unique_id).await;
        let support_card_id = specific_card.get_card();
        // let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        // notify_player_action_repository_guard.notify_to_opponent_what_support_you_use(opponent_unique_id.unwrap(), unit_card_number, ).await;

        UseSupportCardResponse::new(true)
    }

    // TODO: After Refactor (keep it)
    async fn use_support_card(&mut self, calculate_effect_request: CalculateEffectRequest) -> GameCardSupportEffect {
        println!("GameCardSupportServiceImpl: use_specific_support_card()");

        let game_card_support_repository_guard = self.game_card_support_repository.lock().await;
        let game_card_support_effect = unsafe {
            game_card_support_repository_guard.call_support_card_repository_handler(calculate_effect_request.get_support_card_number())
        };

        return game_card_support_effect
    }
}
