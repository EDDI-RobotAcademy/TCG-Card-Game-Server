use async_trait::async_trait;

#[async_trait]
pub trait GameBattleFieldMonitorController {
    async fn battle_field_monitoring(&self, battle_room_number: usize);
}