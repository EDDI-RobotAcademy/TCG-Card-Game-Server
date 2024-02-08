use std::num::ParseIntError;
use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::service::card_kinds_service::CardKindsService;
use crate::card_kinds::service::card_kinds_service_impl::CardKindsServiceImpl;
use crate::game_card_support::controller::game_card_support_controller::GameCardSupportController;
use crate::game_card_support::controller::request_form::draw_support_request_form::DrawSupportRequestForm;
use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;
use crate::game_card_support::controller::response_form::draw_support_response_form::DrawSupportResponseForm;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::service::game_card_support_service::GameCardSupportService;

use crate::game_card_support::service::game_card_support_service_impl::GameCardSupportServiceImpl;
use crate::game_card_support::service::request::calculate_effect_request::CalculateEffectRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_hand::service::response::use_game_hand_support_card_response::UseGameHandSupportCardResponse;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_protocol_validation::service::response::can_use_card_response::CanUseCardResponse;
use crate::game_protocol_validation::service::response::check_protocol_hacking_response::CheckProtocolHackingResponse;
use crate::game_protocol_validation::service::response::is_it_support_card_response::IsItSupportCardResponse;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardSupportControllerImpl {
    game_card_support_service: Arc<AsyncMutex<GameCardSupportServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
}

impl GameCardSupportControllerImpl {
    pub fn new(game_card_support_service: Arc<AsyncMutex<GameCardSupportServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,) -> Self {

        GameCardSupportControllerImpl {
            game_card_support_service,
            game_protocol_validation_service,
            game_hand_service,
            game_deck_service,
            game_tomb_service,
            game_field_unit_service,
            redis_in_memory_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportControllerImpl::new(
                            GameCardSupportServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            GameHandServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    // Redis Session Validation
    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;
        drop(redis_in_memory_service_guard);
        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    // Protocol Hacking Validation
    async fn is_valid_protocol(&self, check_protocol_hacking_request: CheckProtocolHackingRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let check_protocol_hacking_response = game_protocol_validation_service_guard.check_protocol_hacking(check_protocol_hacking_request).await;
        drop(game_protocol_validation_service_guard);
        check_protocol_hacking_response.is_success()
    }

    // Card Kind Validation
    async fn is_it_support_card(&self, is_it_support_card_request: IsItSupportCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let is_it_support_card_response = game_protocol_validation_service_guard.is_it_support_card(is_it_support_card_request).await;
        drop(game_protocol_validation_service_guard);
        is_it_support_card_response.is_success()
    }

    //
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

    async fn get_summary_of_support_card(&self, calculate_effect_request: CalculateEffectRequest) -> GameCardSupportEffect {
        let mut game_card_support_service_guard = self.game_card_support_service.lock().await;
        let game_card_support_effect = game_card_support_service_guard.use_support_card(calculate_effect_request).await;
        drop(game_card_support_service_guard);
        game_card_support_effect
    }
}

#[async_trait]
impl GameCardSupportController for GameCardSupportControllerImpl {
    async fn request_to_use_energy_boost_support(&self, energy_boost_support_request_form: EnergyBoostSupportRequestForm) -> EnergyBoostSupportResponseForm {
        println!("GameCardSupportControllerImpl: request_to_use_energy_boost_support()");

        // 1. Redis에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(energy_boost_support_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // TODO: 세션을 제외하고 애초에 UI 에서 숫자로 전송하면 더 좋다.
        let support_card_number_string = energy_boost_support_request_form.get_support_card_number();
        let support_card_number = support_card_number_string.parse::<i32>().unwrap();

        // 1.1. Hand 에 있는지 확인하여 해킹 여부 검증
        let check_protocol_hacking_response = self.is_valid_protocol(
            energy_boost_support_request_form.to_check_protocol_hacking_request(account_unique_id, support_card_number)).await;
        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 1.2. 실제 서포트 카드가 맞는지 확인
        let is_it_support_response = self.is_it_support_card(
            energy_boost_support_request_form.to_is_it_support_card_request(support_card_number)).await;
        if !is_it_support_response {
            println!("서포트 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 1.3. GameProtocolValidation Service 호출하여 사용 가능한지 조건 검사 (신화 > 4라운드 제약)
        let can_use_card_response = self.is_able_to_use(
            energy_boost_support_request_form.to_can_use_card_request(account_unique_id, support_card_number)).await;
        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 2. Hand Service 호출하여 카드 사용
        let usage_hand_card = self.use_support_card(
            energy_boost_support_request_form.to_use_game_hand_support_card_request(account_unique_id, support_card_number)).await;

        // 3. Support 카드 사용이므로 Tomb Service 호출하여 무덤 배치
        self.place_used_card_to_tomb(
            energy_boost_support_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        // 4. 효과를 적용하기 위해 Support Card Service 호출하여 필요 효과 설정
        let calculated_effect_response = self.get_summary_of_support_card(
            energy_boost_support_request_form.to_calculate_effect_request(support_card_number)).await;

        // 8. 가져온 효과를 기반으로 Deck Service 호출하여 에너지 카드 수량만큼 가능한 검색하여 배치
        let mut game_deck_service_guard = self.game_deck_service.lock().await;
        let found_card_from_deck_response = game_deck_service_guard.find_by_card_id_with_count(
            energy_boost_support_request_form.to_found_card_from_deck_request(
                account_unique_id,
                calculated_effect_response.get_need_to_find_card_id(),
                calculated_effect_response.get_energy_from_deck().get_energy_count())).await;

        // 9. Field Unit Service를 호출하여 배치한 에너지 부착
        let energy_from_deck_info = calculated_effect_response.get_energy_from_deck();
        let boost_race_reference = energy_from_deck_info.get_race();

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let unit_index_number_string = energy_boost_support_request_form.get_unit_index_number();
        let unit_index_number = unit_index_number_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let attach_multiple_energy_to_unit_index_response = game_field_unit_service_guard.attach_multiple_energy_to_field_unit_index(
            energy_boost_support_request_form.to_attach_multiple_energy_to_unit_index_request(
                account_unique_id,
                unit_index_number,
                *boost_race_reference,
                energy_from_deck_info.get_energy_count())).await;

        // 10. Notify Service를 호출하여 Opponent에게 무엇을 할 것인지 알려줌

        EnergyBoostSupportResponseForm::new(true)
    }

    // TODO: NotifyService 추가 필요
    async fn request_to_use_draw_support(&self, draw_support_request_form: DrawSupportRequestForm) -> DrawSupportResponseForm {
        println!("GameCardSupportControllerImpl: request_to_use_draw_support()");

        let account_unique_id = self.is_valid_session(
            draw_support_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session error");
            return DrawSupportResponseForm::new(Vec::new())
        }

        let support_card_number_string = draw_support_request_form.get_support_card_id().to_string();
        let support_card_number = support_card_number_string.parse::<i32>().unwrap();

        let check_hand_hacking_response = self.is_valid_protocol(
            draw_support_request_form.to_check_protocol_hacking_request(account_unique_id, support_card_number)).await;
        if !check_hand_hacking_response {
            println!("Hand hacking detected - account unique id : {}", account_unique_id);
            return DrawSupportResponseForm::new(Vec::new())
        }

        let is_it_support_response = self.is_it_support_card(
            draw_support_request_form.to_is_it_support_card_request(support_card_number)).await;
        if !is_it_support_response {
            println!("Support card hacking detected - account unique id : {}", account_unique_id);
            return DrawSupportResponseForm::new(Vec::new())
        }

        let can_use_card_response = self.is_able_to_use(
            draw_support_request_form.to_can_use_card_request(account_unique_id, support_card_number)).await;
        if !can_use_card_response {
            println!("A mythical grade card can be used after round 4.");
            return DrawSupportResponseForm::new(Vec::new())
        }

        let usage_hand_card = self.use_support_card(
            draw_support_request_form.to_use_game_hand_support_card_request(account_unique_id, support_card_number)).await;

        self.place_used_card_to_tomb(
            draw_support_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        let card_effect_summary = self.get_summary_of_support_card(
            draw_support_request_form.to_calculate_effect_request(support_card_number)).await;

        let game_deck_service_guard = self.game_deck_service.lock().await;
        let draw_deck_response = game_deck_service_guard
            .draw_deck(draw_support_request_form.to_draw_deck_request(card_effect_summary.get_need_to_draw_card_count())).await;
        let drawn_cards = draw_deck_response.get_drawn_card_list().clone();

        DrawSupportResponseForm::new(drawn_cards)
    }
}