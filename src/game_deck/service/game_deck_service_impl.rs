use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::account_deck_card::repository::account_deck_card_repository::AccountDeckCardRepository;
use crate::account_deck_card::repository::account_deck_card_repository_impl::AccountDeckCardRepositoryImpl;
use crate::account_card::repository::account_card_repository::AccountCardRepository;
use crate::account_card::repository::account_card_repository_impl::AccountCardRepositoryImpl;
use crate::common::converter::hash_to_vector_converter::HashToVectorConverter;
use crate::game_deck::entity::game_deck_card::GameDeckCard;
use crate::game_deck::repository::game_deck_repository::GameDeckRepository;

use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_deck::service::request::found_card_from_deck_request::FoundCardFromDeckRequest;
use crate::game_deck::service::request::game_deck_card_draw_request::GameDeckCardDrawRequest;
use crate::game_deck::service::request::game_deck_card_list_request::GameDeckCardListRequest;
use crate::game_deck::service::request::game_deck_start_card_list_request::{GameDeckStartCardListRequest};
use crate::game_deck::service::request::game_deck_card_shuffle_request::{GameDeckCardShuffleRequest};
use crate::game_deck::service::request::search_specific_deck_card_request::SearchSpecificDeckCardRequest;
use crate::game_deck::service::response::draw_cards_from_deck_response::DrawCardsFromDeckResponse;
use crate::game_deck::service::response::found_card_from_deck_response::FoundCardFromDeckResponse;
use crate::game_deck::service::response::game_deck_card_draw_list_response::GameDeckCardDrawListResponse;
use crate::game_deck::service::response::game_deck_card_list_response::GameDeckCardListResponse;
use crate::game_deck::service::response::game_deck_card_shuffle_response::{GameDeckCardShuffleResponse};
use crate::game_deck::service::response::game_deck_start_card_list_response::{GameDeckStartCardListResponse};
use crate::game_deck::service::response::search_specific_deck_card_response::SearchSpecificDeckCardResponse;
use crate::game_hand::repository::game_hand_repository::GameHandRepository;
use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::rock_paper_scissors_waiting_timer::repository::rock_paper_scissors_waiting_timer_repository::RockPaperScissorsWaitingTimerRepository;
use crate::rock_paper_scissors_waiting_timer::repository::rock_paper_scissors_waiting_timer_repository_impl::RockPaperScissorsWaitingTimerRepositoryImpl;

pub struct GameDeckServiceImpl {
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
    account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    rock_paper_scissors_waiting_timer_repository: Arc<AsyncMutex<RockPaperScissorsWaitingTimerRepositoryImpl>>
}

impl GameDeckServiceImpl {
    pub fn new(game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
               account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               rock_paper_scissors_waiting_timer_repository: Arc<AsyncMutex<RockPaperScissorsWaitingTimerRepositoryImpl>>) -> Self {

        GameDeckServiceImpl {
            game_deck_repository,
            game_hand_repository,
            account_card_repository,
            account_deck_card_repository,
            redis_in_memory_repository,
            rock_paper_scissors_waiting_timer_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameDeckServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameDeckServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameDeckServiceImpl::new(
                            GameDeckRepositoryImpl::get_instance(),
                            GameHandRepositoryImpl::get_instance(),
                            AccountCardRepositoryImpl::get_instance(),
                            AccountDeckCardRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
                            RockPaperScissorsWaitingTimerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn parse_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32")
    }

    async fn initialize_game_deck(&self, account_unique_id: i32, deck_id: i32) {
        let account_deck_card_repository_guard = self.account_deck_card_repository.lock().await;
        let account_deck_list = account_deck_card_repository_guard.get_card_list(deck_id).await;

        let account_deck_hash_vector = match account_deck_list {
            Ok(Some(card_list)) => card_list,
            _ => Vec::new(),
        };
        let account_deck_vector = HashToVectorConverter::hash_vector_to_vector(account_deck_hash_vector);
        drop(account_deck_card_repository_guard);

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        game_deck_repository_guard.set_game_deck_from_data(account_unique_id, account_deck_vector);
        drop(game_deck_repository_guard);
    }

    async fn validate_game_deck_card(&self, account_unique_id: i32) -> bool {

        // account_deck_card data 에서 AccountDeckCardList 호출
        let mut account_card_repository_guard = self.account_card_repository.lock().await;
        let mut account_card_list_validate = account_card_repository_guard.get_account_card_list(account_unique_id).await;
        let mutable_vector_deck_card = account_card_list_validate.get_all_card_list_mut();

        // initialize_game_deck 호출
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let initialize_game_deck_for_validate = game_deck_repository_guard.get_game_deck_map().get_mut(&account_unique_id).unwrap();
        let game_card_list_for_validate = initialize_game_deck_for_validate.get_all_cards_in_game_deck_mut();

        // 게임 덱 카드가 계정 덱 카드에 보유 여부 확인 (계정 카드 리스트에서 소거법으로 검증)
        for game_card_deck_card_index in game_card_list_for_validate {
            let game_card_deck_card_id = game_card_deck_card_index.get_card();

            for mut account_card_list_validate_index in &mut *mutable_vector_deck_card {
                let account_deck_card_id = account_card_list_validate_index.get_card();

                if game_card_deck_card_id == account_deck_card_id {
                    let mut card_count = account_card_list_validate_index.get_card_count_mut().clone();
                    let card_count = card_count - 1 ;

                    if card_count < 0 {
                        println!("validate_game_deck_card: false, card_id: {:?}", game_card_deck_card_id);
                        return false;
                    } else {
                        account_card_list_validate_index.set_card_count(card_count);
                    }
                }
            }
        }
        return true;
    }

    async fn shuffle_game_deck(&self, account_unique_id: i32) -> bool {
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        return game_deck_repository_guard.shuffle_game_deck(account_unique_id);
    }

    async fn draw_deck_cards(&self, account_unique_id: i32, num_cards: usize) -> Vec<i32> {
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let drawn_card_list = game_deck_repository_guard.draw_deck_card(account_unique_id, num_cards as i32);
        drop(game_deck_repository_guard);

        drawn_card_list
    }

    async fn add_drawn_cards_to_hand(&self, account_unique_id: i32, drawn_card_list: Vec<i32>) {
        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        game_hand_repository_guard.add_card_list_to_hand(account_unique_id, drawn_card_list);
        drop(game_hand_repository_guard);
    }

    async fn get_game_deck_card_ids(&self, account_unique_id: i32) -> Vec<i32> {
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let game_deck_card_vector = game_deck_repository_guard.get_game_deck_card_ids(account_unique_id);
        drop(game_deck_repository_guard);

        game_deck_card_vector
    }
}

#[async_trait]
impl GameDeckService for GameDeckServiceImpl {
    async fn create_and_shuffle_deck(&self, game_deck_card_list_request: GameDeckStartCardListRequest) -> GameDeckStartCardListResponse {
        println!("GameDeckServiceImpl: create_and_shuffle_deck()");

        let session_id = game_deck_card_list_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;
        let deck_id = game_deck_card_list_request.get_deck_id();

        self.initialize_game_deck(account_unique_id, deck_id).await;
        let result_validation = self.validate_game_deck_card(account_unique_id).await;
        if result_validation == false {
            GameDeckStartCardListResponse::default();
        }

        self.shuffle_game_deck(account_unique_id).await;

        let drawn_card_list = self.draw_deck_cards(account_unique_id, 5).await;
        let drawn_card_list_clone = drawn_card_list.clone();

        self.add_drawn_cards_to_hand(account_unique_id, drawn_card_list).await;
        let mut rock_paper_scissors_waiting_timer_repository = self.rock_paper_scissors_waiting_timer_repository.lock().await;
        rock_paper_scissors_waiting_timer_repository.set_rock_paper_scissors_waiting_timer(account_unique_id).await;
        GameDeckStartCardListResponse::new(true, drawn_card_list_clone)
    }

    async fn shuffle_deck(&self, game_deck_card_shuffle_request: GameDeckCardShuffleRequest) -> GameDeckCardShuffleResponse {
        println!("GameDeckServiceImpl: shuffle_deck()");

        let session_id = game_deck_card_shuffle_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let shuffle_result = self.shuffle_game_deck(account_unique_id).await;

        GameDeckCardShuffleResponse::new(shuffle_result)
    }

    // TODO: 여기서 핸드에 추가해버리는 문제가 있어 재사용성이 떨어짐
    async fn draw_deck(&self, game_deck_card_draw_request: GameDeckCardDrawRequest) -> GameDeckCardDrawListResponse {
        println!("GameDeckServiceImpl: draw_deck()");

        let session_id = game_deck_card_draw_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let draw_count: usize = game_deck_card_draw_request.get_draw_count() as usize;

        let draw_card_vector = self.draw_deck_cards(account_unique_id, draw_count).await;

        self.add_drawn_cards_to_hand(account_unique_id, draw_card_vector.clone()).await;

        GameDeckCardDrawListResponse::new(draw_card_vector.clone())
    }

    // TODO: 핸드에 추가하지 않는 버전. 다른 서비스들과의 호환성 향상을 위함.
    async fn draw_cards_from_deck(&self, draw_cards_from_deck_request: DrawCardsFromDeckRequest) -> DrawCardsFromDeckResponse {
        println!("GameDeckServiceImpl: draw_cards_from_deck()");

        let account_unique_id = draw_cards_from_deck_request.get_account_unique_id();
        let draw_count: usize = draw_cards_from_deck_request.get_draw_count() as usize;

        let draw_card_vector = self.draw_deck_cards(account_unique_id, draw_count).await;

        DrawCardsFromDeckResponse::new(draw_card_vector)
    }

    async fn get_deck(&self, game_deck_card_list_request: GameDeckCardListRequest) -> GameDeckCardListResponse {
        println!("GameDeckServiceImpl: get_deck()");

        let session_id = game_deck_card_list_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let deck_card_list = self.get_game_deck_card_ids(account_unique_id).await;

        GameDeckCardListResponse::new(deck_card_list)
    }

    async fn find_by_card_id_with_count(&self, found_card_from_deck_request: FoundCardFromDeckRequest) -> FoundCardFromDeckResponse {
        println!("GameDeckServiceImpl: find_by_card_id_with_count()");

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let found_card_list = game_deck_repository_guard.find_by_card_id_with_count(
            found_card_from_deck_request.get_account_unique_id(),
            found_card_from_deck_request.get_need_to_find_card_id(),
            found_card_from_deck_request.get_card_count());

        FoundCardFromDeckResponse::new(found_card_list)
    }

    async fn search_specific_deck_card(&self, search_specific_deck_card_request: SearchSpecificDeckCardRequest) -> SearchSpecificDeckCardResponse {
        println!("GameDeckServiceImpl: search_specific_deck_card()");

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;

        let account_unique_id = search_specific_deck_card_request.get_account_unique_id();
        let will_be_found_card_list = search_specific_deck_card_request.get_target_card_id_list();
        let mut searched_card_list = Vec::new();

        for will_be_found_card in will_be_found_card_list {
            let found_card = game_deck_repository_guard
                .find_by_card_id_with_count(account_unique_id, *will_be_found_card, 1);
            if found_card.is_empty() {
                return SearchSpecificDeckCardResponse::new(Vec::new());
            } else {
                searched_card_list.push(*will_be_found_card);
            }
        }

        SearchSpecificDeckCardResponse::new(searched_card_list)
    }
    async fn fake_create_and_shuffle_deck(&self, game_deck_card_list_request: GameDeckStartCardListRequest) -> GameDeckStartCardListResponse {
        println!("GameDeckServiceImpl: create_and_shuffle_deck()");

        let session_id = game_deck_card_list_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;
        let deck_id = game_deck_card_list_request.get_deck_id();

        self.initialize_game_deck(account_unique_id, deck_id).await;
        self.shuffle_game_deck(account_unique_id).await;

        let drawn_card_list = self.draw_deck_cards(account_unique_id, 20).await;
        let drawn_card_list_clone = drawn_card_list.clone();

        self.add_drawn_cards_to_hand(account_unique_id, drawn_card_list).await;

        GameDeckStartCardListResponse::new(true, drawn_card_list_clone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use crate::account_card::entity::card::Card;
    use crate::account_card::entity::account_card_list::AccountCardList;
    use crate::game_deck::entity::game_deck::GameDeck;

    #[tokio::test]
    async fn test_game_deck() {
        let deck_id = 1;
        let account_id = 1;

        let account_deck_card_repository_mutex = AccountDeckCardRepositoryImpl::get_instance();
        let account_deck_card_repository_guard = account_deck_card_repository_mutex.lock().await;
        let account_deck_list = account_deck_card_repository_guard.get_card_list(deck_id).await;

        let account_deck_hash_vector = match account_deck_list {
            Ok(Some(card_list)) => card_list,
            _ => Vec::new(),
        };
        let account_deck_vector = HashToVectorConverter::hash_vector_to_vector(account_deck_hash_vector.clone());
        println!("account_deck_list: {:?}", account_deck_hash_vector);

        let game_deck_repository_mutex = GameDeckRepositoryImpl::get_instance();
        let mut game_deck_repository_guard = game_deck_repository_mutex.lock().await;
        game_deck_repository_guard.set_game_deck_from_data(account_id, account_deck_vector);

        let game_deck_map = game_deck_repository_guard.get_game_deck_map();
        println!("Game Deck: {:?}", game_deck_map);

        game_deck_repository_guard.shuffle_game_deck(account_id);
        let game_deck_map = game_deck_repository_guard.get_game_deck_map();
        println!("After Shuffle -> Game Deck: {:?}", game_deck_map);

        let game_deck = game_deck_map.get(&account_id).unwrap();
        let game_deck_vector = game_deck.get_card_ids();

        println!("Game Deck as Vec<i32>: {:?}", game_deck_vector);
    }

    // #[test]
    // async fn test_search() {
    //     let game_deck_service_mutex = GameDeckServiceImpl::get_instance();
    //     let mut game_deck_service = game_deck_service_mutex.lock().await;
    //
    //     let game_deck_start_card_list_request
    //         = GameDeckStartCardListRequest::new("22".to_string(), "redis_token_str".to_string());
    //
    //     game_deck_service.create_and_shuffle_deck(game_deck_start_card_list_request).await;
    //
    //     let search_request1 = SearchSpecificDeckCardRequest::new(1, 25);
    //     let search_result1 = game_deck_service.search_specific_deck_card(search_request1).await;
    //
    //     assert_eq!(true, search_result1.is_success());
    //
    //     let search_request2 = SearchSpecificDeckCardRequest::new(1, 1000);
    //     let search_result2 = game_deck_service.search_specific_deck_card(search_request2).await;
    //
    //     assert_eq!(false, search_result2.is_success());
    // }

    #[tokio::test]
    async fn test_validate_game_deck_card() {

        let account_card_repository_mutex = AccountCardRepositoryImpl::get_instance();
        let account_card_repository_guard = account_card_repository_mutex.lock().await;


        // game_deck 초기화
        let mut game_deck = GameDeck::new();

        let card1 = GameDeckCard::new(1);
        let card2 = GameDeckCard::new(2);
        let card3 = GameDeckCard::new(3);
        let card4 = GameDeckCard::new(3);
        let card5 = GameDeckCard::new(4);
        let card6 = GameDeckCard::new(5);

        game_deck.add_card_to_game_deck(card1);
        game_deck.add_card_to_game_deck(card2);
        game_deck.add_card_to_game_deck(card3);
        game_deck.add_card_to_game_deck(card4);
        game_deck.add_card_to_game_deck(card5);
        game_deck.add_card_to_game_deck(card6);

        let mut_vec_game_deck_card = game_deck.get_all_cards_in_game_deck_mut();

        println!("mut_vec_game_deck_card: {:?}", mut_vec_game_deck_card);

        // account_card 초기화
        let mut account_card_hashmap = AccountCardList::new();

        let list1 = Card::new(1,2);
        let list2 = Card::new(2,2);
        let list3 = Card::new(3,1); // 3 번 카드를 1 장 부족하게 셋팅하여 테스트 진행 (기대값 false)
        let list4 = Card::new(4,1);
        let list5 = Card::new(5,1);

        account_card_hashmap.add_card(list1);
        account_card_hashmap.add_card(list2);
        account_card_hashmap.add_card(list3);
        account_card_hashmap.add_card(list4);
        account_card_hashmap.add_card(list5);

        let mut_account_card_hashmap = account_card_hashmap.get_all_card_list_mut();

        println!("account_card_hashmap: {:?} ", mut_account_card_hashmap);

        // game_deck_card 의 card_id 와 일치하는 account_card 를 찾아서 card_count 를 삭감하며 검증
        for game_card_deck_card_index in mut_vec_game_deck_card {
            let game_card_deck_card_id = game_card_deck_card_index.get_card();
            println!("game_card_deck_card_id: {:?}", game_card_deck_card_id);

            for mut account_card_list_validate_index in &mut *mut_account_card_hashmap {
                let account_card_id = account_card_list_validate_index.get_card();
                println!("account_deck_card_id: {:?}", account_card_id);

                if game_card_deck_card_id == account_card_id {
                    let mut card_count = account_card_list_validate_index.get_card_count_mut().clone();
                    println!("account_deck_card_list_validate_index: {:?}", account_card_list_validate_index);
                    let card_count = card_count - 1 ;
                    println!("decrease_card_count");
                    println!("game_card_deck_card_id: {:?}, card_count: {:?}", game_card_deck_card_id, card_count);

                    if card_count < 0 {
                        println!("false: {:?}", game_card_deck_card_id);
                    } else {
                        account_card_list_validate_index.set_card_count(card_count);
                    }
                }
            }
        }
    }
}