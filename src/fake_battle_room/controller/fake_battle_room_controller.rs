use async_trait::async_trait;
use crate::fake_battle_room::controller::request_form::create_fake_battle_room_request_form::CreateFakeBattleRoomRequestForm;
use crate::fake_battle_room::controller::response_form::create_fake_battle_room_response_form::CreateFakeBattleRoomResponseForm;

#[async_trait]
pub trait FakeBattleRoomController {
    async fn request_to_create_fake_battle_room(
        &self,
        create_fake_battle_room_request_form: CreateFakeBattleRoomRequestForm)
        -> CreateFakeBattleRoomResponseForm;
}