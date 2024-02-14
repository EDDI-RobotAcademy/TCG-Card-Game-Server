use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::card_grade::service::card_grade_service_impl::CardGradeServiceImpl;
use crate::game_card_tool::controller::game_card_tool_controller::GameCardToolController;
use crate::game_card_tool::entity::game_card_tool_effect::GameCardToolEffect;
use crate::game_card_tool::service::game_card_tool_service::GameCardToolService;

use crate::game_card_tool::service::request::summarize_tool_card_effect_request::SummarizeToolCardEffectRequest;
use crate::game_card_tool::controller::request_form::enhance_attack_point_tool_request_form::EnhanceAttackPointToolRequestForm;
use crate::game_card_tool::controller::response_form::enhance_attack_point_tool_response_form::EnhanceAttackPointToolResponseForm;
use crate::game_card_tool::service::game_card_tool_service_impl::GameCardToolServiceImpl;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_hand::service::request::use_game_hand_tool_card_request::UseGameHandToolCardRequest;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_tool_card_request::IsItToolCardRequest;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardToolControllerImpl {
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_card_tool_service: Arc<AsyncMutex<GameCardToolServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
}

impl GameCardToolControllerImpl {
    pub fn new(battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_card_tool_service: Arc<AsyncMutex<GameCardToolServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>) -> Self {

        GameCardToolControllerImpl {
            battle_room_service,
            game_card_tool_service,
            notify_player_action_service,
            game_protocol_validation_service,
            game_hand_service,
            game_deck_service,
            game_tomb_service,
            game_field_unit_service,
            game_field_energy_service,
            redis_in_memory_service,
            card_grade_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardToolControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardToolControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardToolControllerImpl::new(
                            BattleRoomServiceImpl::get_instance(),
                            GameCardToolServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            GameHandServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            GameFieldEnergyServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            CardGradeServiceImpl::get_instance())));
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
    async fn is_it_tool_card(&self, is_it_tool_card_request: IsItToolCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let is_it_tool_card_response = game_protocol_validation_service_guard.is_it_tool_card(is_it_tool_card_request).await;
        drop(game_protocol_validation_service_guard);
        is_it_tool_card_response.is_success()
    }

    async fn is_able_to_use(&self, can_use_card_request: CanUseCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(can_use_card_request).await;
        drop(game_protocol_validation_service_guard);
        can_use_card_response.is_success()
    }

    async fn use_tool_card(&self, use_game_hand_tool_card_request: UseGameHandToolCardRequest) -> i32 {
        let mut game_hand_service_guard = self.game_hand_service.lock().await;
        let use_game_hand_tool_card_response = game_hand_service_guard.use_tool_card(use_game_hand_tool_card_request).await;
        drop(game_hand_service_guard);
        use_game_hand_tool_card_response.found_card_id()
    }

    async fn place_used_card_to_tomb(&self, place_to_tomb_request: PlaceToTombRequest) {
        let mut game_tomb_service_guard = self.game_tomb_service.lock().await;
        game_tomb_service_guard.add_used_card_to_tomb(place_to_tomb_request).await;
    }

    // TODO: hand에서도 use, summary_effect에서도 use라 혼동 -> summary_effect에서는 summary로 표현하도록 수정 필요
    async fn get_summary_of_tool_card(&self, summarize_tool_card_effect_request: SummarizeToolCardEffectRequest) -> GameCardToolEffect {
        let mut game_card_tool_service_guard = self.game_card_tool_service.lock().await;
        let game_card_tool_effect = game_card_tool_service_guard.summarize_tool_card_effect(summarize_tool_card_effect_request).await;
        drop(game_card_tool_service_guard);
        game_card_tool_effect
    }
    async fn get_opponent_unique_id(&self, find_opponent_by_account_id_request: FindOpponentByAccountIdRequest) -> i32 {
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(find_opponent_by_account_id_request).await;
        drop(battle_room_service_guard);
        find_opponent_by_account_id_response.get_opponent_unique_id()
    }
}

#[async_trait]
impl GameCardToolController for GameCardToolControllerImpl {
    async fn request_to_use_enhance_attack_point_tool(
        &self, enhance_attack_point_tool_request_form: EnhanceAttackPointToolRequestForm) -> EnhanceAttackPointToolResponseForm {
        println!("GameCardToolControllerImpl: request_to_use_enhance_attack_point_tool()");

        // 0. Redis에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(enhance_attack_point_tool_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return EnhanceAttackPointToolResponseForm::new(false)
        }

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let tool_card_number_string = enhance_attack_point_tool_request_form.get_tool_card_id();
        let tool_card_number = tool_card_number_string.parse::<i32>().unwrap();

        // 2. Hand 에 있는지 확인하여 해킹 여부 검증
        let check_protocol_hacking_response = self.is_valid_protocol(
            enhance_attack_point_tool_request_form.to_check_protocol_hacking_request(account_unique_id, tool_card_number)).await;
        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return EnhanceAttackPointToolResponseForm::new(false)
        }

        // 3. 실제 도구 카드가 맞는지 확인
        let is_it_tool_response = self.is_it_tool_card(
            enhance_attack_point_tool_request_form.to_is_it_tool_card_request(tool_card_number)).await;
        if !is_it_tool_response {
            println!("서포트 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return EnhanceAttackPointToolResponseForm::new(false)
        }

        // 4. GameProtocolValidation Service 호출하여 사용 가능한지 조건 검사 (신화 > 4라운드 제약)
        let can_use_card_response = self.is_able_to_use(
            enhance_attack_point_tool_request_form.to_can_use_card_request(account_unique_id, tool_card_number)).await;
        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return EnhanceAttackPointToolResponseForm::new(false)
        }

        // 5. Hand Service 호출하여 카드 사용
        let usage_hand_card = self.use_tool_card(
            enhance_attack_point_tool_request_form.to_use_game_hand_tool_card_request(account_unique_id, tool_card_number)).await;

        // 6. Tool 카드 사용이므로 Tomb Service 호출하여 무덤 배치
        // TODO: 유닛 카드에 도구 카드 번호를 부여 하여, 유닛 카드가 무덤으로 이동 시 도구 카드도 함께 무덤으로 이동하도록 변경이 필요함
        self.place_used_card_to_tomb(
            enhance_attack_point_tool_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        // 7. 효과를 적용하기 위해 Tool Card Service 호출하여 필요 효과 설정
        let calculated_effect_response = self.get_summary_of_tool_card(
            enhance_attack_point_tool_request_form.to_summarize_tool_card_effect_request(tool_card_number)).await;

        // // 8. 가져온 효과를 기반으로 Deck Service 호출하여 에너지 카드 수량만큼 가능한 검색하여 배치
        // let mut game_deck_service_guard = self.game_deck_service.lock().await;
        // let found_card_from_deck_response = game_deck_service_guard.find_by_card_id_with_count(
        //     enhance_attack_point_tool_request_form.to_found_card_from_deck_request(
        //         account_unique_id,
        //         calculated_effect_response.get_need_to_find_card_id(),
        //         calculated_effect_response.get_energy_from_deck().get_energy_count())).await;

        // TODO: enhance attack point 기능에 부합하게 내용 수정이 필요(필드 내 카드 속성 값에 강화 수치 적용)
        // // 9. Field Unit Service를 호출하여 배치한 에너지 부착
        // let energy_from_deck_info = calculated_effect_response.get_energy_from_deck();
        // let boost_race_reference = energy_from_deck_info.get_race();

        // // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let unit_card_index_string = enhance_attack_point_tool_request_form.get_unit_index_number();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();
        //
        // let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        // let attach_multiple_energy_to_unit_index_response = game_field_unit_service_guard.attach_multiple_energy_to_field_unit_index(
        //     enhance_attack_point_tool_request_form.to_attach_multiple_energy_to_unit_index_request(
        //         account_unique_id,
        //         unit_card_index,
        //         *boost_race_reference,
        //         energy_from_deck_info.get_energy_count())).await;

        // // 10. 상대방의 고유 id 값을 확보
        // let opponent_unique_id = self.get_opponent_unique_id(
        //     enhance_attack_point_tool_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;

        // 11. Notify Service를 호출하여 Opponent에게 무엇을 할 것인지 알려줌
        // TODO: enhance attack point 기능에 부합하게 내용 수정이 필요
        // let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
        // let notify_to_opponent_what_you_do_response = notify_player_action_service_guard.notify_to_opponent_you_use_energy_boost(
        //     enhance_attack_point_tool_request_form.to_notify_to_opponent_you_use_enhance_attack_point_card_request(
        //         opponent_unique_id,
        //         unit_card_index,
        //         usage_hand_card,
        //         calculated_effect_response.get_energy_from_deck().get_energy_count(),
        //         calculated_effect_response.get_need_to_find_card_id())).await;
        // if !notify_to_opponent_what_you_do_response.is_success() {
        //     println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
        //     return EnhanceAttackPointToolResponseForm::new(false)
        // }

        EnhanceAttackPointToolResponseForm::new(true)
    }
}