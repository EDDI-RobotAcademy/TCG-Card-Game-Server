use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

// use crate::battle_ready_monitor::controller::battle_ready_monitor_controller::BattleReadyMonitorController;
use crate::battle_ready_monitor::service::battle_ready_monitor_service::BattleReadyMonitorService;
use crate::battle_ready_monitor::service::battle_ready_monitor_service_impl::BattleReadyMonitorServiceImpl;

pub struct BattleReadyMonitorControllerImpl {
    battle_ready_monitor_service: Arc<AsyncMutex<BattleReadyMonitorServiceImpl>>,
}

impl BattleReadyMonitorControllerImpl {
    pub fn new(battle_ready_monitor_service:
               Arc<AsyncMutex<BattleReadyMonitorServiceImpl>>) -> Self {

        BattleReadyMonitorControllerImpl {
            battle_ready_monitor_service,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleReadyMonitorControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleReadyMonitorControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleReadyMonitorControllerImpl::new(
                            BattleReadyMonitorServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

// #[async_trait]
// impl BattleReadyMonitorController for BattleReadyMonitorControllerImpl {
//     async fn start_monitor_for_battle_match(&mut self) {
//         println!("BattleReadyMonitorControllerImpl: start_monitor_for_battle_match()");
//         let battle_ready_monitor_service_mutex = &self.battle_ready_monitor_service;
//         let mut battle_ready_monitor_service_guard = battle_ready_monitor_service_mutex.lock().await;
//         battle_ready_monitor_service_guard.start_monitor().await;
//     }
// }
