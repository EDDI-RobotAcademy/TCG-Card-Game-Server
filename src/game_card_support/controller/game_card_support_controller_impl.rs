use std::num::ParseIntError;
use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::service::card_kinds_service::CardKindsService;
use crate::card_kinds::service::card_kinds_service_impl::CardKindsServiceImpl;
use crate::game_card_support::controller::game_card_support_controller::GameCardSupportController;
use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_support::service::game_card_support_service::GameCardSupportService;

use crate::game_card_support::service::game_card_support_service_impl::GameCardSupportServiceImpl;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
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

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }
}

#[async_trait]
impl GameCardSupportController for GameCardSupportControllerImpl {
    async fn request_to_use_energy_boost_support(&self, energy_boost_support_request_form: EnergyBoostSupportRequestForm) -> EnergyBoostSupportResponseForm {
        println!("GameCardSupportControllerImpl: request_to_use_energy_boost_support()");

        // 0. Redis에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(energy_boost_support_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            return EnergyBoostSupportResponseForm::new(false)
        }

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let support_card_number_string = energy_boost_support_request_form.get_support_card_number();
        let support_card_number = support_card_number_string.parse::<i32>().unwrap();

        // TODO: 쪼갤 것인지 고민이 필요하다 생각했지만 무조건 쪼개자 (만들다 보니 이거 했는지 안 했는지 확인이 어려움)
        // 1.1. GameProtocolValidation Service 호출하여 Hand에 있는지 확인하여 해킹 여부 검증
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let check_protocol_hacking_response = game_protocol_validation_service_guard.check_protocol_hacking(
            energy_boost_support_request_form.to_check_protocol_hacking_request(account_unique_id, support_card_number)).await;
        if !check_protocol_hacking_response.is_success() {
            println!("해킹범을 검거합니다!");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 1.2. CardKinds Service를 호출하여 실제 서포트 카드가 맞는지 확인
        let is_it_support_response = game_protocol_validation_service_guard.is_it_support_card(
            energy_boost_support_request_form.to_is_it_support_card_request(support_card_number)).await;
        if !is_it_support_response.is_success() {
            println!("서포트 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 1.3. GameProtocolValidation Service 호출하여 사용 가능한지 조건 검사 (신화 > 4라운드 제약)
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(
            energy_boost_support_request_form.to_can_use_card_request(account_unique_id, support_card_number)).await;
        if !can_use_card_response.is_success() {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return EnergyBoostSupportResponseForm::new(false)
        }

        // 2. Hand Service 호출하여 카드 사용
        let mut game_hand_service_guard = self.game_hand_service.lock().await;
        let use_game_hand_support_card_response = game_hand_service_guard.use_support_card(
            energy_boost_support_request_form.to_use_game_hand_support_card_request(account_unique_id, support_card_number)).await;
        let usage_hand_card = use_game_hand_support_card_response.found_card_id();

        // 3. Support 카드 사용이므로 Tomb Service 호출하여 무덤 배치
        let mut game_tomb_service_guard = self.game_tomb_service.lock().await;
        let place_to_tomb_response = game_tomb_service_guard.add_used_card_to_tomb(
            energy_boost_support_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        // 4. 효과를 적용하기 위해 Support Card Service 호출하여 필요 효과 설정
        let mut game_card_support_service_guard = self.game_card_support_service.lock().await;
        let calculated_effect_response = game_card_support_service_guard.use_support_card(
            energy_boost_support_request_form.to_calculate_effect_request(support_card_number)).await;

        // 5. 가져온 효과를 기반으로 Deck Service 호출하여 에너지 카드 수량만큼 가능한 검색하여 배치
        let mut game_deck_service_guard = self.game_deck_service.lock().await;
        let found_card_from_deck_response = game_deck_service_guard.find_by_card_id_with_count(
            energy_boost_support_request_form.to_found_card_from_deck_request(
                account_unique_id,
                calculated_effect_response.get_need_to_find_card_id(),
                calculated_effect_response.get_energy_from_deck().get_energy_count())).await;

        // 6. Field Unit Service를 호출하여 배치한 에너지 부착
        let energy_from_deck_info = calculated_effect_response.get_energy_from_deck();
        let boost_race_reference = energy_from_deck_info.get_race();
        let boost_race = *boost_race_reference;

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let unit_number_string = energy_boost_support_request_form.get_unit_number();
        let unit_number = unit_number_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let attach_multiple_energy_to_unit_response = game_field_unit_service_guard.attach_multiple_energy_to_game_field_unit(
            energy_boost_support_request_form.to_attach_multiple_energy_to_field_unit_request(
                account_unique_id,
                unit_number,
                boost_race as i32,
                energy_from_deck_info.get_energy_count())).await;

        // 7. Notify Service를 호출하여 Opponent에게 무엇을 할 것인지 알려줌

        EnergyBoostSupportResponseForm::new(true)
    }
}