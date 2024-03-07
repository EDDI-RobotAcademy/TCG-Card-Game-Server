use async_trait::async_trait;
use crate::fake_battle_room::controller::request_form::create_fake_battle_room_request_form::CreateFakeBattleRoomRequestForm;
use crate::fake_battle_room::controller::request_form::fake_multi_draw_request_form::FakeMultiDrawRequestForm;
use crate::fake_battle_room::controller::response_form::create_fake_battle_room_response_form::CreateFakeBattleRoomResponseForm;
use crate::fake_battle_room::controller::response_form::fake_multi_draw_response_form::FakeMultiDrawResponseForm;

#[async_trait]
pub trait FakeBattleRoomController {
    async fn request_to_create_fake_battle_room(
        &self,
        create_fake_battle_room_request_form: CreateFakeBattleRoomRequestForm)
        -> CreateFakeBattleRoomResponseForm;

    async fn request_to_fake_multi_draw(
        &self,
        fake_multi_draw_request_form: FakeMultiDrawRequestForm)
        -> FakeMultiDrawResponseForm;
}