use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;

use crate::card_grade::service::card_grade_service::CardGradeService;
use crate::card_grade::service::card_grade_service_impl::CardGradeServiceImpl;
use crate::card_race::service::card_race_service::CardRaceService;
use crate::card_race::service::card_race_service_impl::CardRaceServiceImpl;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::converter::vector_string_to_vector_integer::VectorStringToVectorInteger;

use crate::game_card_item::controller::game_card_item_controller::GameCardItemController;
use crate::game_card_item::controller::request_form::add_field_energy_with_field_unit_health_point_item_request_form::AddFieldEnergyWithFieldUnitHealthPointRequestForm;
use crate::game_card_item::controller::request_form::multiple_target_damage_by_field_unit_death_item_request_form::MultipleTargetDamageByFieldUnitDeathItemRequestForm;
use crate::game_card_item::controller::request_form::catastrophic_damage_item_request_form::CatastrophicDamageItemRequestForm;
use crate::game_card_item::controller::request_form::remove_opponent_field_unit_energy_item_request_form::RemoveOpponentFieldUnitEnergyItemRequestForm;
use crate::game_card_item::controller::request_form::target_death_item_request_form::TargetDeathItemRequestForm;
use crate::game_card_item::controller::response_form::add_field_energy_with_field_unit_health_point_item_response_form::AddFieldEnergyWithFieldUnitHealthPointResponseForm;
use crate::game_card_item::controller::response_form::multiple_target_damage_by_field_unit_death_item_response_form::MultipleTargetDamageByFieldUnitDeathItemResponseForm;
use crate::game_card_item::controller::response_form::catastrophic_damage_item_response_form::CatastrophicDamageItemResponseForm;
use crate::game_card_item::controller::response_form::remove_opponent_field_unit_energy_item_response_form::RemoveOpponentFieldUnitEnergyItemResponseForm;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::game_card_item::service::game_card_item_service::GameCardItemService;

use crate::game_card_item::service::game_card_item_service_impl::GameCardItemServiceImpl;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_card_item::service::response::summary_item_card_effect_response::SummaryItemCardEffectResponse;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;

use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_lost_zone::service::game_lost_zone_service::GameLostZoneService;
use crate::game_lost_zone::service::game_lost_zone_service_impl::GameLostZoneServiceImpl;
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_main_character::service::game_main_character_service_impl::GameMainCharacterServiceImpl;

use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_item_card_request::IsItItemCardRequest;

use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardItemControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_card_item_service: Arc<AsyncMutex<GameCardItemServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_lost_zone_service: Arc<AsyncMutex<GameLostZoneServiceImpl>>,
    card_race_service: Arc<AsyncMutex<CardRaceServiceImpl>>,

}

impl GameCardItemControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_card_item_service: Arc<AsyncMutex<GameCardItemServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_lost_zone_service: Arc<AsyncMutex<GameLostZoneServiceImpl>>,
               card_race_service: Arc<AsyncMutex<CardRaceServiceImpl>>,) -> Self {

        GameCardItemControllerImpl {
            game_hand_service,
            game_tomb_service,
            card_grade_service,
            battle_room_service,
            game_card_item_service,
            game_field_unit_service,
            game_protocol_validation_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_field_energy_service,
            game_main_character_service,
            game_deck_service,
            game_lost_zone_service,
            card_race_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardItemControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardItemControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardItemControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            CardGradeServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameCardItemServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameFieldEnergyServiceImpl::get_instance(),
                            GameMainCharacterServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameLostZoneServiceImpl::get_instance(),
                            CardRaceServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    // TODO: 모든 Controller는 검증 로직 때문에 반복을 줄이기 위해 추후 Aspect 처리 필요함
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

    async fn is_it_item_card(&self, is_it_item_card_request: IsItItemCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let is_it_item_card_response = game_protocol_validation_service_guard.is_it_item_card(is_it_item_card_request).await;
        drop(game_protocol_validation_service_guard);
        is_it_item_card_response.is_success()
    }

    async fn is_able_to_use(&self, can_use_card_request: CanUseCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(can_use_card_request).await;
        drop(game_protocol_validation_service_guard);
        can_use_card_response.is_success()
    }

    async fn use_item_card(&self, use_game_hand_item_card_request: UseGameHandItemCardRequest) -> i32 {
        let mut game_hand_service_guard = self.game_hand_service.lock().await;
        let use_game_hand_item_card_response = game_hand_service_guard.use_item_card(use_game_hand_item_card_request).await;
        drop(game_hand_service_guard);
        use_game_hand_item_card_response.get_found_item_card_id()
    }

    async fn place_used_card_to_tomb(&self, place_to_tomb_request: PlaceToTombRequest) {
        let mut game_tomb_service_guard = self.game_tomb_service.lock().await;
        game_tomb_service_guard.add_used_card_to_tomb(place_to_tomb_request).await;
    }

    async fn get_summary_of_item_card(&self, summary_item_card_effect_request: SummaryItemCardEffectRequest) -> SummaryItemCardEffectResponse {
        let mut game_card_item_service_guard = self.game_card_item_service.lock().await;
        let summary_item_card_effect_response = game_card_item_service_guard.summary_item_card(summary_item_card_effect_request).await;
        drop(game_card_item_service_guard);
        summary_item_card_effect_response
    }
}

#[async_trait]
impl GameCardItemController for GameCardItemControllerImpl {
    async fn request_to_use_target_death_item(&self, target_death_item_request_form: TargetDeathItemRequestForm) -> TargetDeathItemResponseForm {
        println!("GameCardItemControllerImpl: request_to_use_target_death_item()");

        // 1. Redis에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(target_death_item_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return TargetDeathItemResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                target_death_item_request_form.to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return TargetDeathItemResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 세션을 제외하고 애초에 UI 에서 숫자로 전송하면 더 좋다.
        let item_card_id_string = target_death_item_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        // 2. Hand 에 있는지 확인하여 해킹 여부 검증
        let check_protocol_hacking_response = self.is_valid_protocol(
            target_death_item_request_form.to_check_protocol_hacking_request(account_unique_id, item_card_id)).await;
        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return TargetDeathItemResponseForm::new(false)
        }

        // 3. 실제 아이템 카드가 맞는지 확인
        let is_it_item_response = self.is_it_item_card(
            target_death_item_request_form.to_is_it_item_card_request(item_card_id)).await;
        if !is_it_item_response {
            println!("아이템 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return TargetDeathItemResponseForm::new(false)
        }

        // 4. GameProtocolValidation Service 호출하여 사용 가능한지 조건 검사 (신화 > 4라운드 제약)
        let can_use_card_response = self.is_able_to_use(
            target_death_item_request_form.to_can_use_card_request(account_unique_id, item_card_id)).await;
        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return TargetDeathItemResponseForm::new(false)
        }

        // 5. 효과를 적용하기 위해 Game Card Item Service 호출하여 필요 효과 설정
        let summarized_item_effect_response = self.get_summary_of_item_card(
            target_death_item_request_form.to_summary_item_effect_request(item_card_id)).await;

        // TODO: 사용 방식에 대한 논의 필요 <- UI에서 어떻게 보여줄 것인가
        // 6. 필드 에너지 혹은 손패의 에너지가 충분한지 확인
        // 여기서 summarized_item_effect_response 의 required_energy 처리
        let required_energy = summarized_item_effect_response.get_required_energy_count();

        // 7. Hand Service 호출하여 카드 사용
        let usage_hand_card = self.use_item_card(
            target_death_item_request_form.to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        // 8. Item 카드 사용이므로 Tomb Service 호출하여 무덤 배치
        self.place_used_card_to_tomb(
            target_death_item_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        // 9. 즉사 스킬 적용을 위해 상대방의 고유 id 값을 확보
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(
            target_death_item_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let opponent_target_unit_index_string = target_death_item_request_form.get_opponent_target_unit_index();
        let opponent_target_unit_index = opponent_target_unit_index_string.parse::<i32>().unwrap();

        // TODO: 추후 즉사 면역인 언데드 등등에 대한 조건도 필요함
        // 10. 타겟 인덱스 유닛이 신화 미만인지 확인
        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let find_target_unit_id_by_index_response = game_field_unit_service_guard.find_target_unit_id_by_index(
            target_death_item_request_form.to_find_target_unit_id_by_index_request(
                find_opponent_by_account_id_response.get_opponent_unique_id(),
                opponent_target_unit_index)).await;

        let card_grade_service_guard = self.card_grade_service.lock().await;
        let opponent_target_unit_grade = card_grade_service_guard.get_card_grade(
            &find_target_unit_id_by_index_response.get_found_opponent_unit_id()).await;

        // 11. Field Unit Service를 호출하여 상대 유닛에 Alternatives 적용
        if opponent_target_unit_grade == GradeEnum::Mythical {
            game_field_unit_service_guard.apply_damage_to_target_unit_index(
                target_death_item_request_form.to_apply_damage_to_target_unit_index(
                    find_opponent_by_account_id_response.get_opponent_unique_id(),
                    opponent_target_unit_index,
                    summarized_item_effect_response.get_alternatives_damage())).await;

            // 즉사기에 면역되어 alternatives로 작용하였음을 알림
            let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
            let notify_to_opponent_you_use_item_card_response = notify_player_action_service_guard
                .notify_to_opponent_you_use_instant_death_alternatives_item_card(
                    target_death_item_request_form.to_notify_to_opponent_you_use_item_instant_death_alternatives_request(
                        find_opponent_by_account_id_response.get_opponent_unique_id(),
                        opponent_target_unit_index,
                        usage_hand_card,
                        summarized_item_effect_response.get_alternatives_damage())).await;

            if !notify_to_opponent_you_use_item_card_response.is_success() {
                println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
                return TargetDeathItemResponseForm::new(false)
            }

            return TargetDeathItemResponseForm::new(true)
        }

        // 12. Field Unit Service를 호출하여 상대 유닛에 즉사 적용
        let apply_instant_death_to_target_unit_index_response = game_field_unit_service_guard.apply_instant_death_to_target_unit_index(
            target_death_item_request_form.to_apply_instant_death_to_target_unit_index_request(
                find_opponent_by_account_id_response.get_opponent_unique_id(),
                opponent_target_unit_index)).await;

        // 즉사기가 적용되어 실제 사망 처리 되었음을 알림
        let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
        let notify_to_opponent_you_use_item_card_response = notify_player_action_service_guard
            .notify_to_opponent_you_use_instant_death_item_card(
                target_death_item_request_form.to_notify_to_opponent_you_use_item_instant_death_request(
                    find_opponent_by_account_id_response.get_opponent_unique_id(),
                    opponent_target_unit_index,
                    usage_hand_card)).await;

        if !notify_to_opponent_you_use_item_card_response.is_success() {
            println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
            return TargetDeathItemResponseForm::new(false)
        }

        TargetDeathItemResponseForm::new(true)
    }

    async fn request_to_use_add_field_energy_with_field_unit_health_point(&self, add_field_energy_with_field_unit_health_point_request_form: AddFieldEnergyWithFieldUnitHealthPointRequestForm) -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {
        println!("GameCardItemControllerImpl: request_to_use_add_field_energy_with_field_unit_health_point()");

        let account_unique_id = self.is_valid_session(add_field_energy_with_field_unit_health_point_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        let item_card_id_string = add_field_energy_with_field_unit_health_point_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        let check_protocol_hacking_response = self.is_valid_protocol(
            add_field_energy_with_field_unit_health_point_request_form.to_check_protocol_hacking_request(account_unique_id, item_card_id)).await;
        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        let is_it_item_response = self.is_it_item_card(
            add_field_energy_with_field_unit_health_point_request_form.to_is_it_item_card_request(item_card_id)).await;
        if !is_it_item_response {
            println!("아이템 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        let can_use_card_response = self.is_able_to_use(
            add_field_energy_with_field_unit_health_point_request_form.to_can_use_card_request(account_unique_id, item_card_id)).await;
        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        let field_unit_index_string = add_field_energy_with_field_unit_health_point_request_form.get_field_unit_index();
        let field_unit_index = field_unit_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let get_current_health_point_of_field_unit_by_index_response = game_field_unit_service_guard
            .get_current_health_point_of_field_unit_by_index(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_get_field_unit_health_point_request(account_unique_id, field_unit_index)).await;

        let current_health_point_of_field_unit = get_current_health_point_of_field_unit_by_index_response.get_current_unit_health_point();

        if current_health_point_of_field_unit == -1 {
            println!("필드에 존재하지 않는 유닛을 지정하여 보냈으므로 당신도 해킹범입니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        drop(game_field_unit_service_guard);

        let mut summarized_item_effect_response = self.get_summary_of_item_card(
            add_field_energy_with_field_unit_health_point_request_form.to_summary_item_effect_request(item_card_id)).await;

        let field_energy_amount_to_increase = summarized_item_effect_response
            .get_field_energy_addition_calculator()
                .calculation_of_field_energy_increment(current_health_point_of_field_unit);

        let game_field_energy_service_guard = self.game_field_energy_service.lock().await;
        let add_field_energy_with_amount_response = game_field_energy_service_guard.add_field_energy_with_amount(
            add_field_energy_with_field_unit_health_point_request_form
                .to_add_field_energy_with_amount_request(account_unique_id, field_energy_amount_to_increase)).await;

        if !add_field_energy_with_amount_response.is_success() {
            println!("필드에 에너지를 추가하는 과정에서 문제가 발생했습니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        drop(game_field_energy_service_guard);

        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
        let notify_opponent_you_use_item_card_response = notify_player_action_service_guard
            .notify_opponent_you_use_item_field_energy_increase_item_card(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_notify_opponent_you_use_item_field_energy_increase_request(opponent_unique_id, item_card_id, field_energy_amount_to_increase)).await;

        if !notify_opponent_you_use_item_card_response.is_success() {
            println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        drop(notify_player_action_service_guard);

        let usage_hand_card = self.use_item_card(
            add_field_energy_with_field_unit_health_point_request_form.to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        self.place_used_card_to_tomb(
            add_field_energy_with_field_unit_health_point_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(true)
    }

    async fn request_to_use_catastrophic_damage_item(&self, catastrophic_damage_item_request_form: CatastrophicDamageItemRequestForm) -> CatastrophicDamageItemResponseForm {
        println!("GameCardItemControllerImpl: request_to_use_catastrophic_damage_item()");

        let account_unique_id = self.is_valid_session(catastrophic_damage_item_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return CatastrophicDamageItemResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                catastrophic_damage_item_request_form.to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return CatastrophicDamageItemResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);
        // TODO: 프로토콜 검증은 추후 추가

        let item_card_id_string = catastrophic_damage_item_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        let can_use_card_response = self.is_able_to_use(
            catastrophic_damage_item_request_form.to_can_use_card_request(account_unique_id, item_card_id)).await;
        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return CatastrophicDamageItemResponseForm::new(false)
        }

        let mut summarized_item_effect_response = self.get_summary_of_item_card(
            catastrophic_damage_item_request_form.to_summary_item_effect_request(item_card_id)).await;

        let damage_for_field_unit = summarized_item_effect_response.get_catastrophic_damage_for_field_unit();
        let damage_for_main_character = summarized_item_effect_response.get_catastrophic_damage_for_main_character();

        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                catastrophic_damage_item_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let apply_catastrophic_damage_to_opponent_field_unit_response = game_field_unit_service_guard
            .apply_catastrophic_damage_to_field_unit(
                catastrophic_damage_item_request_form
                    .to_apply_catastrophic_damage_to_field_unit_request(opponent_unique_id, damage_for_field_unit)).await;

        if !apply_catastrophic_damage_to_opponent_field_unit_response.is_success() {
            println!("상대 유닛 전체에 피해를 주는 데에 실패하였습니다.");
            return CatastrophicDamageItemResponseForm::new(false)
        }

        drop(game_field_unit_service_guard);

        // 상대 본체에는 피해를 가하지 않는 경우가 있을 수 있으므로 다음과 같이 처리
        if damage_for_main_character != -1 {
            let mut game_main_character_service_guard = self.game_main_character_service.lock().await;
            let apply_damage_to_main_character_response = game_main_character_service_guard
                .apply_damage_to_main_character(
                    catastrophic_damage_item_request_form
                        .to_apply_damage_to_main_character(opponent_unique_id, damage_for_main_character)).await;

            if !apply_damage_to_main_character_response.is_success() {
                println!("상대 본체에 피해를 주는 데에 실패하였습니다.");
                return CatastrophicDamageItemResponseForm::new(false)
            }
            let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
            let notify_item_card_response = notify_player_action_service_guard
                .notify_to_opponent_you_use_damage_main_character_item_card(
                    catastrophic_damage_item_request_form
                        .to_notify_opponent_you_use_damage_main_character_item_card_request(
                            opponent_unique_id,
                            item_card_id,
                            damage_for_main_character)).await;

            if !notify_item_card_response.is_success() {
                println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
                return CatastrophicDamageItemResponseForm::new(false)
            }

            drop(game_main_character_service_guard);
        }

        let will_be_lost_deck_card_count = summarized_item_effect_response.get_will_be_lost_deck_card_count();

        // 다른 광역기의 경우 로스트 존으로 이동시키는 기능이 없을 수 있으므로 다음과 같이 처리
        if will_be_lost_deck_card_count != -1 {
            let mut game_deck_service_guard = self.game_deck_service.lock().await;
            let draw_cards_from_deck_response = game_deck_service_guard
                .draw_cards_from_deck(
                    catastrophic_damage_item_request_form
                        .to_draw_cards_from_deck_request(opponent_unique_id, will_be_lost_deck_card_count)).await;

            drop(game_deck_service_guard);

            let mut will_be_lost_deck_card_list = draw_cards_from_deck_response.get_drawn_card_list().clone();
            let mut game_lost_zone_service_guard = self.game_lost_zone_service.lock().await;
            for will_be_lost_card in will_be_lost_deck_card_list {
                let place_card_to_lost_zone_response = game_lost_zone_service_guard
                    .place_card_to_lost_zone(
                        catastrophic_damage_item_request_form
                            .to_place_card_to_lost_zone_request(opponent_unique_id, will_be_lost_card)).await;
                if !place_card_to_lost_zone_response.is_success() {
                    println!("상대 카드를 로스트 존으로 이동하는데에 실패했습니다.");
                    return CatastrophicDamageItemResponseForm::new(false)
                }
                let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
                let notify_item_card_response = notify_player_action_service_guard
                    .notify_to_opponent_you_use_destroy_deck_item_card(
                        catastrophic_damage_item_request_form
                            .to_notify_opponent_you_use_destroy_deck_item_card_request(
                                opponent_unique_id,
                                item_card_id,
                                will_be_lost_card)).await;

                if !notify_item_card_response.is_success() {
                    println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
                    return CatastrophicDamageItemResponseForm::new(false)
                }
            }

            drop(game_lost_zone_service_guard);
        }

        let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
        let notify_item_card_response = notify_player_action_service_guard
            .notify_to_opponent_you_use_catastrophic_damage_item_card(
                catastrophic_damage_item_request_form
                    .to_notify_opponent_you_use_catastrophic_damage_item_card_request(
                        opponent_unique_id,
                        item_card_id,
                        damage_for_field_unit)).await;

        if !notify_item_card_response.is_success() {
            println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
            return CatastrophicDamageItemResponseForm::new(false)
        }
        drop(notify_player_action_service_guard);


        let usage_hand_card = self.use_item_card(
            catastrophic_damage_item_request_form.to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        self.place_used_card_to_tomb(
            catastrophic_damage_item_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        CatastrophicDamageItemResponseForm::new(true)
    }

    async fn request_to_use_applying_multiple_target_damage_by_field_unit_death_item(
        &self, multiple_target_damage_by_field_unit_death_item_request_form: MultipleTargetDamageByFieldUnitDeathItemRequestForm) -> MultipleTargetDamageByFieldUnitDeathItemResponseForm {

        println!("GameCardItemControllerImpl: request_to_use_catastrophic_damage_by_field_unit_death_item()");

        let account_unique_id = self.is_valid_session(multiple_target_damage_by_field_unit_death_item_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);
        // TODO: 프로토콜 검증은 추후 추가

        // 사용할 변수들 사전 parsing
        let item_card_id_string = multiple_target_damage_by_field_unit_death_item_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        let my_field_unit_index_string = multiple_target_damage_by_field_unit_death_item_request_form.get_unit_index();
        let my_field_unit_index = my_field_unit_index_string.parse::<i32>().unwrap();

        let opponent_target_unit_index_list_string = multiple_target_damage_by_field_unit_death_item_request_form.get_opponent_target_unit_index_list().to_vec();
        let opponent_target_unit_index_list = VectorStringToVectorInteger::vector_string_to_vector_i32(opponent_target_unit_index_list_string);

        // 사용할 아이템 카드 요약 정보
        let mut summarized_item_effect_response = self.get_summary_of_item_card(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_summary_item_effect_request(item_card_id)).await;

        let target_unit_count = summarized_item_effect_response.get_target_count_that_can_be_damaged();
        let can_be_sacrificed_unit_list = summarized_item_effect_response.get_unit_list_that_can_be_sacrificed();

        if target_unit_count != opponent_target_unit_index_list.len() as i32 {
            println!("{}마리의 상대 유닛을 정확히 지정해주세요!", target_unit_count);
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(false)
        }

        // TODO: Naming issue 가 존재. 시간 관점에서, 기능적인 측면에서 의도한 바와 같은 기능을 하여 사용함.
        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let fount_unit_card_id = game_field_unit_service_guard
            .find_target_unit_id_by_index(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_find_target_unit_id_by_index_request(account_unique_id,
                                                             my_field_unit_index)).await.get_found_opponent_unit_id();

        if !can_be_sacrificed_unit_list.contains(&fount_unit_card_id) {
            println!("제물로 사용할 수 없는 유닛입니다!");
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(false)
        }

        let health_point_of_sacrifice = game_field_unit_service_guard
            .get_current_health_point_of_field_unit_by_index(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_get_current_health_point_of_field_unit_by_index_request(account_unique_id,
                                                                                my_field_unit_index)).await.get_current_unit_health_point();

        let apply_instance_death_to_unit_response = game_field_unit_service_guard
            .apply_instant_death_to_target_unit_index(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_apply_instance_death_to_field_unit_request(account_unique_id,
                                                                   my_field_unit_index)).await;

        if !apply_instance_death_to_unit_response.is_success() {
            println!("희생시킬 수 없는 유닛입니다!");
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(false)
        }

        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        for opponent_unit_index in opponent_target_unit_index_list {
            let apply_damage_to_target_unit_response = game_field_unit_service_guard
                .apply_damage_to_target_unit_index(
                    multiple_target_damage_by_field_unit_death_item_request_form
                        .to_apply_damage_to_target_unit_request(opponent_unique_id, opponent_unit_index, health_point_of_sacrifice)).await;
            if !apply_damage_to_target_unit_response.is_success() {
                println!("제물의 체력 수치만큼 데미지를 주는 과정에서 문제가 발생했습니다!");
                return MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(false)
            }
        }

        drop(game_field_unit_service_guard);

        // TODO: Notify service 호출하여 공지

        let usage_hand_item_card = self.use_item_card(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        self.place_used_card_to_tomb(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_item_card)).await;

        // TODO: 죽은 유닛을 묘지로 보내는 것을 일단 이렇게 처리함
        // TODO: 사망 판정이 선행되어야 함 (이 과정에서 필드 Indexmap 에서 제거되어야 함)
        // TODO: 기존에 카드 이동 자체를 담당하는 도메인이 따로 있었어야 할 것이라는 생각이 드는 상황
        self.place_used_card_to_tomb(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_place_to_tomb_request(account_unique_id, fount_unit_card_id)).await;

        MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(true)
    }

    async fn request_to_use_opponent_field_unit_energy_removal_item(
        &self, remove_opponent_field_unit_energy_item_request_form: RemoveOpponentFieldUnitEnergyItemRequestForm) -> RemoveOpponentFieldUnitEnergyItemResponseForm {

        println!("GameCardItemControllerImpl: request_to_use_opponent_field_unit_energy_removal_item()");

        let account_unique_id = self.is_valid_session(remove_opponent_field_unit_energy_item_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                remove_opponent_field_unit_energy_item_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::new(false)
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 프로토콜 검증은 추후 추가

        // 사용할 변수들 사전 parsing
        let item_card_id_string = remove_opponent_field_unit_energy_item_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        let opponent_field_unit_index_string = remove_opponent_field_unit_energy_item_request_form.get_opponent_target_unit_index();
        let opponent_field_unit_index = opponent_field_unit_index_string.parse::<i32>().unwrap();

        // 사용할 아이템 카드 요약 정보
        let mut summarized_item_effect_response = self.get_summary_of_item_card(
            remove_opponent_field_unit_energy_item_request_form
                .to_summary_item_effect_request(item_card_id)).await;

        let energy_quantity = summarized_item_effect_response.get_will_be_removed_energy_count();
        let alternative_damage = summarized_item_effect_response.get_alternatives_damage();

        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                remove_opponent_field_unit_energy_item_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;

        let found_opponent_unit_id = game_field_unit_service_guard
            .find_target_unit_id_by_index(
                remove_opponent_field_unit_energy_item_request_form
                    .to_find_target_unit_id_by_index_request(opponent_unique_id, opponent_field_unit_index)).await.get_found_opponent_unit_id();

        let mut card_race_service_guard = self.card_race_service.lock().await;
        let found_opponent_unit_race = card_race_service_guard.get_card_race(&found_opponent_unit_id).await;

        drop(card_race_service_guard);

        let current_attached_race_energy_of_opponent_unit = game_field_unit_service_guard
            .get_current_attached_energy_of_field_unit_by_index(
                remove_opponent_field_unit_energy_item_request_form
                    .to_get_current_attached_energy_of_unit_by_index(opponent_unique_id,
                                                                     opponent_field_unit_index,
                                                                     found_opponent_unit_race)).await.get_current_attached_energy();

        if current_attached_race_energy_of_opponent_unit == -1 {
            println!("붙은 에너지가 존재하지 않아 변환 데미지를 입힙니다.");
            let apply_alternative_damage_response = game_field_unit_service_guard
                .apply_damage_to_target_unit_index(
                    remove_opponent_field_unit_energy_item_request_form
                        .to_apply_damage_to_target_unit_request(opponent_unique_id,
                                                                opponent_field_unit_index,
                                                                alternative_damage)).await;
            if !apply_alternative_damage_response.is_success() {
                println!("변환 데미지를 주는 데에 실패했습니다!");
                return RemoveOpponentFieldUnitEnergyItemResponseForm::new(false)
            }
            return RemoveOpponentFieldUnitEnergyItemResponseForm::new(true)
        }

        let detach_multiple_energy_form_field_unit_response = game_field_unit_service_guard
            .detach_multiple_energy_from_field_unit(
                remove_opponent_field_unit_energy_item_request_form
                    .to_detach_energy_from_field_unit_request(opponent_unique_id,
                                                              opponent_field_unit_index,
                                                              found_opponent_unit_race,
                                                              energy_quantity)).await;

        if !detach_multiple_energy_form_field_unit_response.is_success() {
            println!("유닛 에너지 제거에 실패했습니다!");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::new(false)
        }

        let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
        let notify_item_card_response = notify_player_action_service_guard
            .notify_to_opponent_you_use_field_unit_energy_removal_item_card(
                remove_opponent_field_unit_energy_item_request_form
                    .to_notify_opponent_you_use_field_unit_energy_removal_item_card_request(
                        opponent_unique_id,
                        item_card_id,
                        energy_quantity)).await;

        if !notify_item_card_response.is_success() {
            println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::new(false)
        }
        drop(notify_player_action_service_guard);

        RemoveOpponentFieldUnitEnergyItemResponseForm::new(true)
    }
}