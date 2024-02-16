use crate::account::service::response::account_register_response::AccountRegisterResponse;
use serde::{Deserialize, Serialize};
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::account::service::response::account_logout_response::AccountLogoutResponse;
use crate::account::service::response::account_modify_response::AccountModifyResponse;
use crate::account::service::response::account_delete_response::AccountDeleteResponse;
use crate::account_card::service::response::account_card_list_response::AccountCardListResponse;
use crate::account_deck::service::response::account_deck_delete_response::AccountDeckDeleteResponse;
use crate::account_deck::service::response::account_deck_list_response::AccountDeckListResponse;
use crate::account_deck::service::response::account_deck_modify_response::AccountDeckModifyResponse;
use crate::account_deck::service::response::account_deck_register_response::AccountDeckRegisterResponse;
use crate::account_deck_card::controller::response_form::account_deck_card_list_response_form::AccountDeckCardListResponseForm;
use crate::account_deck_card::controller::response_form::account_deck_configuration_response_form::AccountDeckConfigurationResponseForm;
use crate::account_point::service::response::gain_gold_response::GainGoldResponse;
use crate::account_point::service::response::pay_gold_response::PayGoldResponse;
use crate::battle_ready_account_hash::service::response::battle_ready_account_hash_response::BattleReadyAccountHashResponse;
use crate::battle_ready_account_hash::service::response::check_battle_prepare_response::CheckBattlePrepareResponse;
use crate::battle_room::service::response::what_is_the_room_number_response::WhatIsTheRoomNumberResponse;
use crate::battle_wait_queue::service::response::battle_wait_queue_response::BattleWaitQueueResponse;
use crate::client_program::service::response::client_program_exit_response::ClientProgramExitResponse;
use crate::game_turn::controller::response_form::first_turn_decision_wait_queue_response_form::FirstTurnDecisionWaitQueueResponseForm;

use crate::game_card_active_skill::controller::response_form::targeting_active_skill_response_form::TargetingActiveSkillResponseForm;
use crate::game_card_energy::controller::response_form::attach_general_energy_card_response_form::AttachGeneralEnergyCardResponseForm;
use crate::game_card_energy::controller::response_form::attach_special_energy_card_response_form::AttachSpecialEnergyCardResponseForm;
use crate::game_card_item::controller::response_form::add_field_energy_with_field_unit_health_point_item_response_form::AddFieldEnergyWithFieldUnitHealthPointResponseForm;
use crate::game_card_item::controller::response_form::catastrophic_damage_item_response_form::CatastrophicDamageItemResponseForm;
use crate::game_card_item::controller::response_form::multiple_target_damage_by_field_unit_death_item_response_form::MultipleTargetDamageByFieldUnitDeathItemResponseForm;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::game_card_support::controller::response_form::draw_support_response_form::DrawSupportResponseForm;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_support::controller::response_form::remove_opponent_field_energy_support_response_form::RemoveOpponentFieldEnergySupportResponseForm;
use crate::game_card_support::controller::response_form::search_unit_support_response_form::SearchUnitSupportResponseForm;
use crate::game_card_unit::controller::response_form::deploy_unit_response_form::DeployUnitResponseForm;
use crate::game_card_unit::controller::response_form::attack_unit_response_form::AttackUnitResponseForm;
use crate::game_deck::service::response::game_deck_start_card_list_response::{GameDeckStartCardListResponse};
use crate::game_hand::controller::response_form::mulligan_response_form::MulliganResponseForm;
use crate::game_turn::controller::response_form::first_turn_decision_response_form::FirstTurnDecisionResponseForm;
use crate::notify_player_action::entity::notify_opponent_increase_field_energy_item_usage::NotifyOpponentIncreaseFieldEnergyItemUsage;
use crate::notify_player_action::entity::notify_opponent_to_enhance_attack_point_tool_usage::NotifyOpponentToEnhanceAttackPointToolUsage;
use crate::notify_player_action::entity::notify_opponent_remove_field_energy_support_usage::NotifyOpponentRemoveFieldEnergySupportUsage;
use crate::notify_player_action::entity::notify_opponent_search_support_usage::NotifyOpponentSearchSupportUsage;
use crate::notify_player_action::entity::notify_opponent_to_draw_support_usage::NotifyOpponentToDrawSupportUsage;
use crate::notify_player_action::entity::notify_opponent_to_energy_boost::NotifyOpponentToEnergyBoost;
use crate::notify_player_action::entity::notify_opponent_to_energy_usage::NotifyOpponentToEnergyUsage;
use crate::notify_player_action::entity::notify_opponent_to_instant_death_item_alternatives_usage::NotifyOpponentToInstantDeathItemAlternativesUsage;
use crate::notify_player_action::entity::notify_opponent_to_instant_death_item_usage::NotifyOpponentToInstantDeathItemUsage;
use crate::notify_player_action::entity::notify_opponent_to_unit_deploy::NotifyOpponentToUnitDeploy;
use crate::notify_player_action::entity::notify_opponent_to_catastrophic_damage_item_usage::NotifyOpponentToCatastrophicDamageItemUsage;
use crate::notify_player_action::entity::notify_opponent_to_damage_main_character_item_usage::NotifyOpponentToDamageMainCharacterItemUsage;
use crate::notify_player_action::entity::notify_opponent_to_destroy_deck_item_usage::NotifyOpponentToDestroyDeckItemUsage;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_item_card_response::NotifyToOpponentYouUseItemCardResponse;

use crate::shop_gacha::service::response::get_specific_race_card_response::GetSpecificRaceCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    // Account
    ACCOUNT_REGISTER(AccountRegisterResponse),
    ACCOUNT_LOGIN(AccountLoginResponse),
    ACCOUNT_SESSION_LOGIN(AccountLoginResponse),
    ACCOUNT_LOGOUT(AccountLogoutResponse),
    ACCOUNT_MODIFY(AccountModifyResponse),
    ACCOUNT_DELETE(AccountDeleteResponse),

    // Battle Entrance
    BATTLE_WAIT_QUEUE_FOR_MATCH(BattleWaitQueueResponse),
    BATTLE_READY(BattleReadyAccountHashResponse),
    CHECK_BATTLE_PREPARE(CheckBattlePrepareResponse),
    WHAT_IS_THE_ROOM_NUMBER(WhatIsTheRoomNumberResponse),
    BATTLE_DECK_LIST(AccountDeckListResponse),
    BATTLE_START_SHUFFLED_GAME_DECK_CARD_LIST(GameDeckStartCardListResponse),

    // First Turn Decision For Wait
    FIRST_TURN_DECISION_WAIT_QUEUE(FirstTurnDecisionWaitQueueResponseForm),
    // First TUrn Decision
    FIRST_TURN_DECISION(FirstTurnDecisionResponseForm),

    // Account Card
    ACCOUNT_CARD_LIST(AccountCardListResponse),

    // Account Deck
    ACCOUNT_DECK_REGISTER(AccountDeckRegisterResponse),
    ACCOUNT_DECK_LIST(AccountDeckListResponse),
    ACCOUNT_DECK_MODIFY(AccountDeckModifyResponse),
    ACCOUNT_DECK_DELETE(AccountDeckDeleteResponse),

    // Account Deck Card
    DECK_CARD_CONFIGURATION(AccountDeckConfigurationResponseForm),
    DECK_CARD_LIST(AccountDeckCardListResponseForm),

    // Shop
    SHOP_GET_SPECIFIC_RACE_CARD_DEFAULT(GetSpecificRaceCardResponse),

    // Account Point
    GAIN_GOLD(GainGoldResponse),
    PAY_GOLD(PayGoldResponse),

    // Battle Field
    DEPLOY_UNIT_USAGE(DeployUnitResponseForm),
    ENERGY_BOOST_SUPPORT_USAGE(EnergyBoostSupportResponseForm),
    ATTACH_GENERAL_ENERGY(AttachGeneralEnergyCardResponseForm),
    ATTACH_SPECIAL_ENERGY(AttachSpecialEnergyCardResponseForm),
    GENERAL_DRAW_SUPPORT_USAGE(DrawSupportResponseForm),
    SEARCH_UNIT_SUPPORT_USAGE(SearchUnitSupportResponseForm),
    REMOVE_OPPONENT_FIELD_ENERGY_SUPPORT_USAGE(RemoveOpponentFieldEnergySupportResponseForm),
    ADD_FIELD_ENERGY_BY_FIELD_UNIT_HEALTH_POINT_ITEM_USAGE(AddFieldEnergyWithFieldUnitHealthPointResponseForm),
    TARGET_DEATH_ITEM_USAGE(TargetDeathItemResponseForm),
    CATASTROPHIC_DAMAGE_ITEM_USAGE(CatastrophicDamageItemResponseForm),
    MULTIPLE_TARGET_DAMAGE_BY_FIELD_UNIT_SACRIFICE_ITEM_USAGE(MultipleTargetDamageByFieldUnitDeathItemResponseForm),
    ATTACK_UNIT(AttackUnitResponseForm),
    TARGETING_ACTIVE_SKILL(TargetingActiveSkillResponseForm),


    // Notification to players
    NOTIFY_OPPONENT_TO_UNIT_DEPLOY(NotifyOpponentToUnitDeploy),
    NOTIFY_OPPONENT_TO_ENERGY_USAGE(NotifyOpponentToEnergyUsage),
    NOTIFY_OPPONENT_TO_ENERGY_BOOST(NotifyOpponentToEnergyBoost),
    NOTIFY_OPPONENT_TO_INSTANT_DEATH_ITEM_USAGE(NotifyOpponentToInstantDeathItemUsage),
    NOTIFY_OPPONENT_TO_INSTANT_DEATH_ITEM_ALTERNATIVES_USAGE(NotifyOpponentToInstantDeathItemAlternativesUsage),
    NOTIFY_OPPONENT_TO_DRAW_SUPPORT_USAGE(NotifyOpponentToDrawSupportUsage),
    NOTIFY_OPPONENT_SEARCH_SUPPORT_USAGE(NotifyOpponentSearchSupportUsage),
    NOTIFY_OPPONENT_REMOVE_FIELD_ENERGY_SUPPORT_USAGE(NotifyOpponentRemoveFieldEnergySupportUsage),
    NOTIFY_OPPONENT_INCREASE_FIELD_ENERGY_ITEM_USAGE(NotifyOpponentIncreaseFieldEnergyItemUsage),
    NOTIFY_OPPONENT_TO_ENHANCE_ATTACK_POINT_TOOL_USAGE(NotifyOpponentToEnhanceAttackPointToolUsage),
    NOTIFY_OPPONENT_TO_CATASTROPHIC_DAMAGE_ITEM_USAGE(NotifyOpponentToCatastrophicDamageItemUsage),
    NOTIFY_OPPONENT_TO_DAMAGE_MAIN_CHARACTER_ITEM_USAGE(NotifyOpponentToDamageMainCharacterItemUsage),
    NOTIFY_OPPONENT_TO_DESTORY_DECK_ITEM_USAGE(NotifyOpponentToDestroyDeckItemUsage),

    // Mulligan
    CHANGE_FIRST_HAND(MulliganResponseForm),

    // Program Exit
    PROGRAM_EXIT(ClientProgramExitResponse),
}
