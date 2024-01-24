use std::sync::Arc;
use lazy_static::lazy_static;
use crate::battle_ready_monitor::service::battle_ready_monitor_service_impl::BattleReadyMonitorServiceImpl;

pub struct BattleReadyMonitorControllerImpl {
    battle_ready_monitor_service: Arc<BattleReadyMonitorServiceImpl>,
}

impl BattleReadyMonitorControllerImpl {
    pub fn new(battle_ready_monitor_service:
               Arc<BattleReadyMonitorServiceImpl>) -> Self {

        BattleReadyMonitorControllerImpl {
            battle_ready_monitor_service,
        }
    }

    pub fn get_instance() -> Arc<BattleReadyMonitorControllerImpl> {
        lazy_static! {
            static ref INSTANCE: Arc<BattleReadyMonitorControllerImpl> =
                Arc::new(
                    BattleReadyMonitorControllerImpl::new(
                        BattleReadyMonitorServiceImpl::get_instance()));
        }
        INSTANCE.clone()
    }
}