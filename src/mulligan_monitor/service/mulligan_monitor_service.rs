use async_trait::async_trait;

#[async_trait]
pub trait MulliganMonitorService {
    async fn mulligan_monitoring(&self, battle_room_number: usize);
}