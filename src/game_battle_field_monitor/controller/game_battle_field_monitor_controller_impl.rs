use async_trait::async_trait;

use crate::game_battle_field_monitor::controller::game_battle_field_monitor_controller::GameBattleFieldMonitorController;

pub struct GameBattleFieldMonitorControllerImpl;

impl GameBattleFieldMonitorControllerImpl {
    pub fn new() -> Self {
        GameBattleFieldMonitorControllerImpl {}
    }
}

#[async_trait]
impl GameBattleFieldMonitorController for GameBattleFieldMonitorControllerImpl {
    async fn battle_field_monitoring(&self, battle_room_number: usize) {
        loop {
            println!("Battle Field Monitor -> Room number: {}", battle_room_number);
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }
}