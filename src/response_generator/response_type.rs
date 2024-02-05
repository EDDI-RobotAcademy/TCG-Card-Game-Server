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
use crate::game_deck::service::response::game_start_deck_card_list_response::GameStartDeckCardListResponse;
use crate::game_hand::controller::response_form::use_game_hand_energy_card_response_form::UseGameHandEnergyCardResponseForm;
use crate::game_hand::controller::response_form::use_game_hand_unit_card_response_form::UseGameHandUnitCardResponseForm;

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

    // Account Card
    ACCOUNT_CARD_LIST(AccountCardListResponse),

    // Account Deck
    ACCOUNT_DECK_REGISTER(AccountDeckRegisterResponse),
    ACCOUNT_DECK_LIST(AccountDeckListResponse),
    ACCOUNT_DECK_MODIFY(AccountDeckModifyResponse),
    ACCOUNT_DECK_DELETE(AccountDeckDeleteResponse),

    WHAT_IS_THE_ROOM_NUMBER(WhatIsTheRoomNumberResponse),
    CHECK_BATTLE_PREPARE(CheckBattlePrepareResponse),
    BATTLE_START_SHUFFLED_GAME_DECK_CARD_LIST(GameStartDeckCardListResponse),

    // Account Deck Card
    DECK_CARD_CONFIGURATION(AccountDeckConfigurationResponseForm),
    DECK_CARD_LIST(AccountDeckCardListResponseForm),

    // Account Point
    GAIN_GOLD(GainGoldResponse),
    PAY_GOLD(PayGoldResponse),

    // Shop
    SHOP_FREE_CARD(FreeCardResponse),
    SHOP_GET_CARD_DEFAULT(GetCardDefaultResponse),

    // Battle Entrance
    BATTLE_DECK_LIST(AccountDeckListResponse),
    BATTLE_WAIT_QUEUE_FOR_MATCH(BattleWaitQueueResponse),
    BATTLE_READY(BattleReadyAccountHashResponse),
    BATTLE_DECK_CARD_LIST(AccountDeckCardListResponseForm),

    // Battle Field
    HAND_TO_BATTLE_FIELD_UNIT_USAGE(UseGameHandUnitCardResponseForm),
    ENERGY_TO_BATTLE_FIELD_UNIT_USAGE(UseGameHandEnergyCardResponseForm),

    // Program Exit
    PROGRAM_EXIT(ClientProgramExitResponse),
}
