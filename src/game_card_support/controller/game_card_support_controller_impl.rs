use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::card_grade::service::card_grade_service::CardGradeService;
use crate::card_grade::service::card_grade_service_impl::CardGradeServiceImpl;
use crate::common::converter::vector_string_to_vector_integer::VectorStringToVectorInteger;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::game_card_support::controller::game_card_support_controller::GameCardSupportController;
use crate::game_card_support::controller::request_form::draw_support_request_form::DrawSupportRequestForm;
use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;
use crate::game_card_support::controller::request_form::remove_opponent_field_energy_support_request_form::RemoveOpponentFieldEnergySupportRequestForm;
use crate::game_card_support::controller::request_form::search_unit_support_request_form::SearchUnitSupportRequestForm;
use crate::game_card_support::controller::response_form::draw_support_response_form::DrawSupportResponseForm;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_support::controller::response_form::remove_opponent_field_energy_support_response_form::RemoveOpponentFieldEnergySupportResponseForm;
use crate::game_card_support::controller::response_form::search_unit_support_response_form::SearchUnitSupportResponseForm;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::service::game_card_support_service::GameCardSupportService;

use crate::game_card_support::service::game_card_support_service_impl::GameCardSupportServiceImpl;
use crate::game_card_support::service::request::summarize_support_card_effect_request::SummarizeSupportCardEffectRequest;
use crate::game_card_support_usage_counter::service::game_card_support_usage_counter_service::GameCardSupportUsageCounterService;
use crate::game_card_support_usage_counter::service::game_card_support_usage_counter_service_impl::GameCardSupportUsageCounterServiceImpl;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_round::service::game_round_service::GameRoundService;
use crate::game_round::service::game_round_service_impl::GameRoundServiceImpl;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::notify_player_action_info_service_impl::NotifyPlayerActionInfoServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardSupportControllerImpl {
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_card_support_service: Arc<AsyncMutex<GameCardSupportServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
    game_card_support_usage_counter_service: Arc<AsyncMutex<GameCardSupportUsageCounterServiceImpl>>,
    notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
}

impl GameCardSupportControllerImpl {
    pub fn new(battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_card_support_service: Arc<AsyncMutex<GameCardSupportServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
               game_card_support_usage_counter_service: Arc<AsyncMutex<GameCardSupportUsageCounterServiceImpl>>,
               notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,) -> Self {

        GameCardSupportControllerImpl {
            battle_room_service,
            game_card_support_service,
            notify_player_action_service,
            game_protocol_validation_service,
            game_hand_service,
            game_deck_service,
            game_tomb_service,
            game_field_unit_service,
            game_field_energy_service,
            redis_in_memory_service,
            card_grade_service,
            game_card_support_usage_counter_service,
            notify_player_action_info_service
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportControllerImpl::new(
                            BattleRoomServiceImpl::get_instance(),
                            GameCardSupportServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            GameHandServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            GameFieldEnergyServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            CardGradeServiceImpl::get_instance(),
                            GameCardSupportUsageCounterServiceImpl::get_instance(),
                            NotifyPlayerActionInfoServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;
        drop(redis_in_memory_service_guard);
        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    async fn is_valid_protocol(&self, check_protocol_hacking_request: CheckProtocolHackingRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let check_protocol_hacking_response = game_protocol_validation_service_guard.check_protocol_hacking(check_protocol_hacking_request).await;
        drop(game_protocol_validation_service_guard);
        check_protocol_hacking_response.is_success()
    }

    async fn is_it_support_card(&self, is_it_support_card_request: IsItSupportCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let is_it_support_card_response = game_protocol_validation_service_guard.is_it_support_card(is_it_support_card_request).await;
        drop(game_protocol_validation_service_guard);
        is_it_support_card_response.is_success()
    }

    async fn is_able_to_use(&self, can_use_card_request: CanUseCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(can_use_card_request).await;
        drop(game_protocol_validation_service_guard);
        can_use_card_response.is_success()
    }

    async fn use_support_card(&self, use_game_hand_support_card_request: UseGameHandSupportCardRequest) -> i32 {
        let mut game_hand_service_guard = self.game_hand_service.lock().await;
        let use_game_hand_support_card_response = game_hand_service_guard.use_support_card(use_game_hand_support_card_request).await;
        drop(game_hand_service_guard);
        use_game_hand_support_card_response.found_card_id()
    }

    async fn place_used_card_to_tomb(&self, place_to_tomb_request: PlaceToTombRequest) {
        let mut game_tomb_service_guard = self.game_tomb_service.lock().await;
        game_tomb_service_guard.add_used_card_to_tomb(place_to_tomb_request).await;
    }

    async fn get_summary_of_support_card(&self, summarize_support_card_effect_request: SummarizeSupportCardEffectRequest) -> GameCardSupportEffect {
        let mut game_card_support_service_guard = self.game_card_support_service.lock().await;
        let game_card_support_effect = game_card_support_service_guard.summarize_support_card_effect(summarize_support_card_effect_request).await;
        drop(game_card_support_service_guard);
        game_card_support_effect
    }

    async fn get_opponent_unique_id(&self, find_opponent_by_account_id_request: FindOpponentByAccountIdRequest) -> i32 {
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(find_opponent_by_account_id_request).await;
        drop(battle_room_service_guard);
        find_opponent_by_account_id_response.get_opponent_unique_id()
    }
}

#[async_trait]
impl GameCardSupportController for GameCardSupportControllerImpl {
    async fn request_to_use_energy_boost_support(&self, energy_boost_support_request_form: EnergyBoostSupportRequestForm) -> EnergyBoostSupportResponseForm {
        println!("GameCardSupportControllerImpl: request_to_use_energy_boost_support()");

        // Redis 에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(
            energy_boost_support_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 사용자의 턴 확인
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                energy_boost_support_request_form.to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return EnergyBoostSupportResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 세션을 제외하고 애초에 UI 에서 숫자로 전송하면 더 좋다.
        let support_card_number_string = energy_boost_support_request_form.get_support_card_id();
        let support_card_number = support_card_number_string.parse::<i32>().unwrap();

        // Hand 에 있는지 확인하여 해킹 여부 검증
        let check_protocol_hacking_response = self.is_valid_protocol(
            energy_boost_support_request_form
                .to_check_protocol_hacking_request(account_unique_id, support_card_number)).await;

        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 실제 서포트 카드가 맞는지 확인
        let is_it_support_response = self.is_it_support_card(
            energy_boost_support_request_form.to_is_it_support_card_request(support_card_number)).await;
        if !is_it_support_response {
            println!("서포트 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // GameProtocolValidation Service 호출하여 사용 가능한지 조건 검사 (신화 > 4라운드 제약)
        let can_use_card_response = self.is_able_to_use(
            energy_boost_support_request_form
                .to_can_use_card_request(account_unique_id, support_card_number)).await;

        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 서포트 카드 사용 횟수 카운트
        let mut game_card_support_usage_counter_service =
            self.game_card_support_usage_counter_service.lock().await;

        let check_support_card_usage_count_response =
            game_card_support_usage_counter_service.check_support_card_usage_count(
                energy_boost_support_request_form.to_check_support_card_usage_count_request(account_unique_id)).await;

        if check_support_card_usage_count_response.get_used_count() > 0 {
            println!("Support card usage limit over");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // Hand Service 호출하여 카드 사용
        let usage_hand_card = self.use_support_card(
            energy_boost_support_request_form
                .to_use_game_hand_support_card_request(account_unique_id, support_card_number)).await;

        // Support 카드 사용이므로 Tomb Service 호출하여 무덤 배치
        self.place_used_card_to_tomb(
            energy_boost_support_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        // 효과를 적용하기 위해 Support Card Service 호출하여 필요 효과 설정
        let calculated_effect_response = self.get_summary_of_support_card(
            energy_boost_support_request_form
                .to_summarize_support_card_effect_request(support_card_number)).await;

        // 가져온 효과를 기반으로 Deck Service 호출하여 에너지 카드 수량만큼 가능한 검색하여 배치
        let mut game_deck_service_guard =
            self.game_deck_service.lock().await;

        let found_card_from_deck_response =
            game_deck_service_guard.find_by_card_id_with_count(
                energy_boost_support_request_form.to_found_card_from_deck_request(
                    account_unique_id,
                    calculated_effect_response.get_need_to_find_card_id(),
                    calculated_effect_response.get_energy_from_deck().get_energy_count())).await;

        // Field Unit Service 를 호출하여 배치한 에너지 부착
        let energy_from_deck_info = calculated_effect_response.get_energy_from_deck();
        let boost_race_reference = energy_from_deck_info.get_race();

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let unit_card_index_string = energy_boost_support_request_form.get_unit_index_number();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        game_field_unit_service_guard.attach_multiple_energy_to_field_unit_index(
            energy_boost_support_request_form.to_attach_multiple_energy_to_unit_index_request(
                account_unique_id,
                unit_card_index,
                *boost_race_reference,
                found_card_from_deck_response.found_card_list().len() as i32)).await;

        drop(game_field_unit_service_guard);

        game_card_support_usage_counter_service.update_support_card_usage_count(
            energy_boost_support_request_form
                .to_update_support_card_usage_count_request(account_unique_id)).await;

        drop(game_card_support_usage_counter_service);

        // 서포트 카드 사용을 통해 덱에서 카드를 가져왔으므로 셔플
        let mut game_deck_service_guard = self.game_deck_service.lock().await;
        game_deck_service_guard.shuffle_deck(
            energy_boost_support_request_form.to_shuffle_deck_request()).await;

        drop(game_deck_service_guard);

        // 상대방의 고유 id 값을 확보
        let opponent_unique_id = self.get_opponent_unique_id(
            energy_boost_support_request_form
                .to_find_opponent_by_account_id_request(account_unique_id)).await;

        // 변화된 사항들 전체 공지
        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_boost_energy_to_specific_unit_by_using_hand_card(
            energy_boost_support_request_form
                .to_notice_boost_energy_to_specific_unit_by_using_hand_card_request(
                    account_unique_id,
                    opponent_unique_id,
                    usage_hand_card,
                    found_card_from_deck_response.found_card_list().clone(),
                    unit_card_index)).await;

        drop(notify_player_action_info_service_guard);

        EnergyBoostSupportResponseForm::new(true)
    }

    async fn request_to_use_draw_support(&self, draw_support_request_form: DrawSupportRequestForm) -> DrawSupportResponseForm {
        println!("GameCardSupportControllerImpl: request_to_use_draw_support()");

        let account_unique_id =
            self.is_valid_session(draw_support_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session error");
            return DrawSupportResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                draw_support_request_form.to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return DrawSupportResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        let support_card_number_string = draw_support_request_form.get_support_card_id().to_string();
        let support_card_number = support_card_number_string.parse::<i32>().unwrap();

        let check_hand_hacking_response = self.is_valid_protocol(
            draw_support_request_form.to_check_protocol_hacking_request(account_unique_id, support_card_number)).await;
        if !check_hand_hacking_response {
            println!("Hand hacking detected - account unique id : {}", account_unique_id);
            return DrawSupportResponseForm::new(false)
        }

        let is_it_support_response = self.is_it_support_card(
            draw_support_request_form.to_is_it_support_card_request(support_card_number)).await;
        if !is_it_support_response {
            println!("Support card hacking detected - account unique id : {}", account_unique_id);
            return DrawSupportResponseForm::new(false)
        }

        let can_use_card_response = self.is_able_to_use(
            draw_support_request_form.to_can_use_card_request(account_unique_id, support_card_number)).await;
        if !can_use_card_response {
            println!("A mythical grade card can be used after round 4.");
            return DrawSupportResponseForm::new(false)
        }

        let mut game_card_support_usage_counter_service =
            self.game_card_support_usage_counter_service.lock().await;

        let check_support_card_usage_count_response =
            game_card_support_usage_counter_service.check_support_card_usage_count(
                draw_support_request_form.to_check_support_card_usage_count_request(account_unique_id)).await;

        if check_support_card_usage_count_response.get_used_count() > 0 {
            println!("Support card usage limit over");
            return DrawSupportResponseForm::new(false)
        }

        let support_card_effect_summary = self.get_summary_of_support_card(
            draw_support_request_form.to_summarize_support_card_effect_request(support_card_number)).await;

        let mut game_deck_service_guard = self.game_deck_service.lock().await;
        let draw_deck_response =
            game_deck_service_guard.draw_cards_from_deck(
                draw_support_request_form
                    .to_draw_cards_from_deck_request(
                        account_unique_id,
                        support_card_effect_summary.get_need_to_draw_card_count())).await;

        game_deck_service_guard.shuffle_deck(
            draw_support_request_form.to_shuffle_deck_request()).await;

        let drawn_cards = draw_deck_response.get_drawn_card_list().clone();

        drop(game_deck_service_guard);

        let usage_hand_card = self.use_support_card(
            draw_support_request_form.to_use_game_hand_support_card_request(account_unique_id, support_card_number)).await;

        let mut game_hand_service_guard = self.game_hand_service.lock().await;
        game_hand_service_guard.add_card_list_to_hand(
            draw_support_request_form
                .to_add_card_list_to_hand_request(account_unique_id, drawn_cards.clone())).await;

        drop(game_hand_service_guard);

        self.place_used_card_to_tomb(
            draw_support_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        game_card_support_usage_counter_service.update_support_card_usage_count(
            draw_support_request_form.to_update_support_card_usage_count_request(account_unique_id)).await;

        drop(game_card_support_usage_counter_service);

        let opponent_unique_id = self.get_opponent_unique_id(
            draw_support_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_draw_card_by_using_hand_card(
            draw_support_request_form
                .to_notice_draw_card_by_using_hand_card_request(
                    account_unique_id,
                    opponent_unique_id,
                    support_card_number,
                    drawn_cards.clone())).await;

        drop(notify_player_action_info_service_guard);

        DrawSupportResponseForm::new(true)
    }

    // 여러 장의 유닛 카드 동시에 검색해서 핸드에 추가하는 형태
    async fn request_to_use_search_unit_support(&self, search_unit_support_request_form: SearchUnitSupportRequestForm) -> SearchUnitSupportResponseForm {
        println!("GameCardSupportControllerImpl: request_to_use_search_unit_support()");

        let account_unique_id = self.is_valid_session(
            search_unit_support_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session error");
            return SearchUnitSupportResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                search_unit_support_request_form.to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return SearchUnitSupportResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        let support_card_number_string = search_unit_support_request_form.get_support_card_number().to_string();
        let support_card_number = support_card_number_string.parse::<i32>().unwrap();

        let check_hand_hacking_response = self.is_valid_protocol(
            search_unit_support_request_form.to_check_protocol_hacking_request(account_unique_id, support_card_number)).await;
        if !check_hand_hacking_response {
            println!("Hand hacking detected - account unique id : {}", account_unique_id);
            return SearchUnitSupportResponseForm::new(false)
        }

        let is_it_support_response = self.is_it_support_card(
            search_unit_support_request_form.to_is_it_support_card_request(support_card_number)).await;
        if !is_it_support_response {
            println!("Support card hacking detected - account unique id : {}", account_unique_id);
            return SearchUnitSupportResponseForm::new(false)
        }

        let can_use_card_response = self.is_able_to_use(
            search_unit_support_request_form.to_can_use_card_request(account_unique_id, support_card_number)).await;
        if !can_use_card_response {
            println!("A mythical grade card can be used after round 4.");
            return SearchUnitSupportResponseForm::new(false)
        }

        let mut game_card_support_usage_counter_service = self.game_card_support_usage_counter_service.lock().await;
        let check_support_card_usage_count_response =
            game_card_support_usage_counter_service.check_support_card_usage_count(
                search_unit_support_request_form.to_check_support_card_usage_count_request(account_unique_id)).await;

        if check_support_card_usage_count_response.get_used_count() > 0 {
            println!("Support card usage limit over");
            return SearchUnitSupportResponseForm::new(false)
        }

        let card_effect_summary = self.get_summary_of_support_card(
            search_unit_support_request_form.to_summarize_support_card_effect_request(support_card_number)).await;

        let searching_grade_limit = card_effect_summary.get_unit_from_deck().get_grade_limit();
        let searching_card_count = card_effect_summary.get_unit_from_deck().get_unit_count();

        let target_unit_card_number_list_string = search_unit_support_request_form.get_target_unit_card_list().clone();
        let target_unit_card_number_list =
            VectorStringToVectorInteger::vector_string_to_vector_i32(target_unit_card_number_list_string);

        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let mut card_grade_service_guard = self.card_grade_service.lock().await;

        // card kind && grade check
        for unit_card_number in target_unit_card_number_list.clone() {
            let is_it_unit_request = search_unit_support_request_form.to_is_it_unit_card_request(unit_card_number);
            let is_it_unit_response = game_protocol_validation_service_guard.is_it_unit_card(is_it_unit_request).await;
            if !is_it_unit_response.is_success() {
                println!("Target is not unit.");
                return SearchUnitSupportResponseForm::new(false)
            }
            let grade_of_unit = card_grade_service_guard.get_card_grade(&unit_card_number).await;
            if grade_of_unit as i32 > searching_grade_limit as i32 {
                println!("Player chose too high grade unit card to search.");
                return SearchUnitSupportResponseForm::new(false)
            }
        }

        // target number check
        if search_unit_support_request_form.get_target_unit_card_list().len() != searching_card_count as usize {
            println!("Player should choose {} unit(s) from deck.", searching_card_count);
            return SearchUnitSupportResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);
        drop(card_grade_service_guard);

        // remove found card from deck && add it to hand && shuffle
        let mut game_deck_service_guard = self.game_deck_service.lock().await;
        for unit_card_number in target_unit_card_number_list.clone() {
            let search_response = game_deck_service_guard
                .search_specific_deck_card(search_unit_support_request_form
                    .to_search_specific_deck_card_request(account_unique_id, unit_card_number)).await;
            if !search_response.is_success() {
                println!("Target unit - {} is not found in player's deck.", unit_card_number);
                return SearchUnitSupportResponseForm::new(false)
            }
        }

        game_deck_service_guard.shuffle_deck(
            search_unit_support_request_form.to_shuffle_deck_request()).await;

        drop(game_deck_service_guard);

        let usage_hand_card = self.use_support_card(
            search_unit_support_request_form.to_use_game_hand_support_card_request(account_unique_id, support_card_number)).await;

        self.place_used_card_to_tomb(
            search_unit_support_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        game_card_support_usage_counter_service.update_support_card_usage_count(
            search_unit_support_request_form.to_update_support_card_usage_count_request(account_unique_id)).await;

        let opponent_unique_id = self.get_opponent_unique_id(
            search_unit_support_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        notify_player_action_info_service_guard.notice_search_card_by_using_hand_card(
            search_unit_support_request_form
                .to_notice_search_card_by_using_hand_card_request(
                    account_unique_id,
                    opponent_unique_id,
                    usage_hand_card,
                    target_unit_card_number_list.clone())).await;

        drop(notify_player_action_info_service_guard);

        SearchUnitSupportResponseForm::new(true)
    }

    async fn request_to_use_remove_opponent_field_energy_support(&self, remove_opponent_field_energy_support_request_form: RemoveOpponentFieldEnergySupportRequestForm) -> RemoveOpponentFieldEnergySupportResponseForm {
        println!("GameCardSupportControllerImpl: request_to_use_remove_opponent_field_energy_support()");

        let account_unique_id = self.is_valid_session(
            remove_opponent_field_energy_support_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session error");
            return RemoveOpponentFieldEnergySupportResponseForm::new(false)
        }

        // let mut game_protocol_validation_service_guard =
        //     self.game_protocol_validation_service.lock().await;
        //
        // let is_this_your_turn_response =
        //     game_protocol_validation_service_guard.is_this_your_turn(
        //         remove_opponent_field_energy_support_request_form.to_is_this_your_turn_request(account_unique_id)).await;
        //
        // if !is_this_your_turn_response.is_success() {
        //     println!("당신의 턴이 아닙니다.");
        //     return RemoveOpponentFieldEnergySupportResponseForm::new(false)
        // }
        //
        // drop(game_protocol_validation_service_guard);

        let support_card_number_string = remove_opponent_field_energy_support_request_form.get_support_card_id().to_string();
        let support_card_number = support_card_number_string.parse::<i32>().unwrap();

        let check_hand_hacking_response = self.is_valid_protocol(
            remove_opponent_field_energy_support_request_form.to_check_protocol_hacking_request(account_unique_id, support_card_number)).await;
        if !check_hand_hacking_response {
            println!("Hand hacking detected - account unique id : {}", account_unique_id);
            return RemoveOpponentFieldEnergySupportResponseForm::new(false)
        }

        let is_it_support_response = self.is_it_support_card(
            remove_opponent_field_energy_support_request_form.to_is_it_support_card_request(support_card_number)).await;
        if !is_it_support_response {
            println!("Support card hacking detected - account unique id : {}", account_unique_id);
            return RemoveOpponentFieldEnergySupportResponseForm::new(false)
        }

        let can_use_card_response = self.is_able_to_use(
            remove_opponent_field_energy_support_request_form.to_can_use_card_request(account_unique_id, support_card_number)).await;
        if !can_use_card_response {
            println!("A mythical grade card can be used after round 4.");
            return RemoveOpponentFieldEnergySupportResponseForm::new(false)
        }

        let mut game_card_support_usage_counter_service = self.game_card_support_usage_counter_service.lock().await;
        let check_support_card_usage_count_response =
            game_card_support_usage_counter_service.check_support_card_usage_count(
                remove_opponent_field_energy_support_request_form.to_check_support_card_usage_count_request(account_unique_id)).await;

        if check_support_card_usage_count_response.get_used_count() > 0 {
            println!("Support card usage limit over");
            return RemoveOpponentFieldEnergySupportResponseForm::new(false)
        }

        let card_effect_summary = self.get_summary_of_support_card(
            remove_opponent_field_energy_support_request_form.to_summarize_support_card_effect_request(support_card_number)).await;

        let opponent_unique_id = self.get_opponent_unique_id(
            remove_opponent_field_energy_support_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;

        let mut game_field_energy_service_guard = self.game_field_energy_service.lock().await;
        let remove_field_energy_with_amount_response = game_field_energy_service_guard.remove_field_energy_with_amount(
            remove_opponent_field_energy_support_request_form.to_remove_field_energy_with_amount_request(
                opponent_unique_id,
                card_effect_summary.get_removal_amount_of_opponent_field_energy())).await;
        if !remove_field_energy_with_amount_response.get_is_success() {
            println!("Failed to remove opponent's field energy.");
            return RemoveOpponentFieldEnergySupportResponseForm::new(false)
        }

        drop(game_field_energy_service_guard);

        let usage_hand_card = self.use_support_card(
            remove_opponent_field_energy_support_request_form
                .to_use_game_hand_support_card_request(account_unique_id, support_card_number)).await;

        self.place_used_card_to_tomb(
            remove_opponent_field_energy_support_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        game_card_support_usage_counter_service.update_support_card_usage_count(
            remove_opponent_field_energy_support_request_form
                .to_update_support_card_usage_count_request(account_unique_id)).await;

        // let mut notify_player_action_service_guard =
        //     self.notify_player_action_service.lock().await;
        //
        // let notify_opponent_you_use_support_card_response =
        //     notify_player_action_service_guard
        //         .notify_opponent_you_use_field_energy_remove_support_card(
        //             remove_opponent_field_energy_support_request_form
        //                 .to_notify_to_opponent_you_use_field_energy_remove_support_card_request(
        //                     opponent_unique_id,
        //                     support_card_number,
        //                     card_effect_summary.get_removal_amount_of_opponent_field_energy())).await;
        //
        // // TODO: 만약 공지가 제대로 되지 않았다고 하면 어떻게 진행할 것인가; 여기에서 return false 해버리면 지난 logic 은 그대로 수행된 채로 남게 됨
        // if !notify_opponent_you_use_support_card_response.is_success() {
        //     println!("Notification Error - Failed to notice field energy remove support card usage to opponent.");
        // }
        //
        // drop(notify_player_action_service_guard);

        RemoveOpponentFieldEnergySupportResponseForm::new(true)
    }
}