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
use crate::game_card_energy::controller::response_form::attach_general_energy_card_response_form::AttachGeneralEnergyCardResponseForm;
use crate::game_card_support::controller::response_form::draw_support_response_form::DrawSupportResponseForm;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_unit::controller::response_form::deploy_unit_response_form::DeployUnitResponseForm;
use crate::game_deck::service::response::game_deck_start_card_list_response::{GameDeckStartCardListResponse};
use crate::game_hand::controller::response_form::mulligan_response_form::MulliganResponseForm;
use crate::notify_player_action::entity::notify_opponent_to_draw_support_usage::NotifyOpponentToDrawSupportUsage;
use crate::notify_player_action::entity::notify_opponent_to_energy_boost::NotifyOpponentToEnergyBoost;
use crate::notify_player_action::entity::notify_opponent_to_energy_usage::NotifyOpponentToEnergyUsage;
use crate::notify_player_action::entity::notify_opponent_to_unit_deploy::NotifyOpponentToUnitDeploy;

use crate::shop::service::response::free_card_response::FreeCardResponse;
use crate::shop::service::response::get_card_default_response::GetCardDefaultResponse;

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
    SHOP_FREE_CARD(FreeCardResponse),
    SHOP_GET_CARD_DEFAULT(GetCardDefaultResponse),

    // Account Point
    GAIN_GOLD(GainGoldResponse),
    PAY_GOLD(PayGoldResponse),

    // Battle Field
    DEPLOY_UNIT_USAGE(DeployUnitResponseForm),
    ENERGY_BOOST_SUPPORT_USAGE(EnergyBoostSupportResponseForm),
    ATTACH_GENERAL_ENERGY(AttachGeneralEnergyCardResponseForm),
    GENERAL_DRAW_SUPPORT_USAGE(DrawSupportResponseForm),

    // Notification to players
    NOTIFY_OPPONENT_TO_UNIT_DEPLOY(NotifyOpponentToUnitDeploy),
    NOTIFY_OPPONENT_TO_ENERGY_USAGE(NotifyOpponentToEnergyUsage),
    NOTIFY_OPPONENT_TO_ENERGY_BOOST(NotifyOpponentToEnergyBoost),
    NOTIFY_OPPONENT_TO_DRAW_SUPPORT_USAGE(NotifyOpponentToDrawSupportUsage),

    // Mulligan
    CHANGE_FIRST_HAND(MulliganResponseForm),

    // Program Exit
    PROGRAM_EXIT(ClientProgramExitResponse),
}
