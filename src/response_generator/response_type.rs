use crate::account::service::response::account_register_response::AccountRegisterResponse;
use serde::{Deserialize, Serialize};
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::account::service::response::account_logout_response::AccountLogoutResponse;
use crate::account::service::response::account_modify_response::AccountModifyResponse;
use crate::account::service::response::account_delete_response::AccountDeleteResponse;
use crate::account_deck::service::response::account_deck_list_response::AccountDeckListResponse;
use crate::account_deck::service::response::account_deck_modify_response::AccountDeckModifyResponse;
use crate::account_deck::service::response::account_deck_register_response::AccountDeckRegisterResponse;
use crate::account_point::service::response::gain_gold_response::GainGoldResponse;
use crate::account_point::service::response::pay_gold_response::PayGoldResponse;
use crate::battle_ready_account_hash::service::response::battle_ready_account_hash_response::BattleReadyAccountHashResponse;
use crate::battle_ready_account_hash::service::response::check_battle_prepare_response::CheckBattlePrepareResponse;
use crate::battle_room::service::response::what_is_the_room_number_response::WhatIsTheRoomNumberResponse;
use crate::battle_wait_queue::service::response::battle_wait_queue_response::BattleWaitQueueResponse;
use crate::client_program::service::response::client_program_exit_response::ClientProgramExitResponse;

use crate::shop::service::request::get_card_default_request::GetCardDafaultRequest;

use crate::account_deck_card::controller::response::account_deck_card_list_response::AccountDeckCardListResponse;
use crate::account_deck_card::controller::response::account_deck_configuration_response::AccountDeckConfigurationResponse;

use crate::shop::service::response::free_card_response::FreeCardResponse;
use crate::shop::service::response::get_card_default_response::GetCardDafaultResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    ACCOUNT_REGISTER(AccountRegisterResponse),
    ACCOUNT_LOGIN(AccountLoginResponse),
    ACCOUNT_SESSION_LOGIN(AccountLoginResponse),
    ACCOUNT_LOGOUT(AccountLogoutResponse),
    ACCOUNT_MODIFY(AccountModifyResponse),
    ACCOUNT_DELETE(AccountDeleteResponse),

    ACCOUNT_DECK_REGISTER(AccountDeckRegisterResponse),
    ACCOUNT_DECK_LIST(AccountDeckListResponse),
    ACCOUNT_DECK_MODIFY(AccountDeckModifyResponse),

    WHAT_IS_THE_ROOM_NUMBER(WhatIsTheRoomNumberResponse),
    CHECK_BATTLE_PREPARE(CheckBattlePrepareResponse),

    DECK_CARD_CONFIGURATION(AccountDeckConfigurationResponse),
    DECK_CARD_LIST(AccountDeckCardListResponse),

    SHOP_FREE_CARD(FreeCardResponse),

    GAIN_GOLD(GainGoldResponse),
    PAY_GOLD(PayGoldResponse),

    SHOP_GET_CARD_DEFAULT(GetCardDafaultResponse),

    BATTLE_DECK_LIST(AccountDeckListResponse),
    BATTLE_WAIT_QUEUE_FOR_MATCH(BattleWaitQueueResponse),
    BATTLE_READY(BattleReadyAccountHashResponse),
    BATTLE_DECK_CARD_LIST(AccountDeckCardListResponse),

    PROGRAM_EXIT(ClientProgramExitResponse),
}
