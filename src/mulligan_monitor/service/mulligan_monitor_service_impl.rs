use std::sync::Arc;
use async_trait::async_trait;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;
use crate::mulligan::repository::mulligan_repository::MulliganRepository;
use crate::mulligan::repository::mulligan_repository_impl::MulliganRepositoryImpl;

use crate::mulligan_monitor::service::mulligan_monitor_service::MulliganMonitorService;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::notify_player_action_info::repository::notify_player_action_info_repository_impl::NotifyPlayerActionInfoRepositoryImpl;

pub struct MulliganMonitorServiceImpl {
    battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
    mulligan_repository: Arc<AsyncMutex<MulliganRepositoryImpl>>,
    notify_player_action_info: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
}

impl MulliganMonitorServiceImpl {
    pub fn new() -> Self {
        MulliganMonitorServiceImpl {
            battle_room_repository: BattleRoomRepositoryImpl::get_instance(),
            mulligan_repository: MulliganRepositoryImpl::get_instance(),
            notify_player_action_info: NotifyPlayerActionInfoRepositoryImpl::get_instance(),
        }
    }
}

#[async_trait]
impl MulliganMonitorService for MulliganMonitorServiceImpl {
    async fn mulligan_monitoring(&self, battle_room_number: usize) {
        
        let mut battle_room_repository_guard =
            self.battle_room_repository.lock().await;

        let player_list =
            battle_room_repository_guard.get_players_in_battle_room(battle_room_number).await;

        drop(battle_room_repository_guard);

        let account_list = player_list.unwrap();
        let first_account = account_list[0];
        let second_account = account_list[1];

        loop {
            println!("Mulligan monitoring for room number {} is on going", battle_room_number);

            let mut mulligan_repository_guard =
                self.mulligan_repository.lock().await;

            let first_account_has_finished =
                mulligan_repository_guard.check_mulligan_finish(first_account).await;
            let second_account_has_finished =
                mulligan_repository_guard.check_mulligan_finish(second_account).await;

            if first_account_has_finished && second_account_has_finished {
                println!("Both players finished mulligan.");

                // TODO: UI 에서 Notify 로만 대응하는 것이 확정일 경우 살리면 됨
                let first_account_record_removed =
                    mulligan_repository_guard.remove_mulligan_finish_record(first_account).await;
                let second_account_record_removed =
                    mulligan_repository_guard.remove_mulligan_finish_record(second_account).await;

                if first_account_record_removed && second_account_record_removed {
                    println!("Mulligan record removed successfully");
                }

                // TODO: Timer 까지 Server 에서 감지할 경우 살리면 됨
                // let first_account_mulligan_timer_removed =
                //     mulligan_repository_guard.remove_mulligan_timer(first_account).await;
                // let second_account_mulligan_timer_removed =
                //     mulligan_repository_guard.remove_mulligan_timer(second_account).await;
                //
                // if first_account_mulligan_timer_removed && second_account_mulligan_timer_removed {
                //     println!("Mulligan timers removed successfully");
                // }

                drop(mulligan_repository_guard);

                let mut notify_player_action_info_guard =
                    self.notify_player_action_info.lock().await;

                let _ = notify_player_action_info_guard
                    .notice_mulligan_finished(first_account, second_account).await;

                drop(notify_player_action_info_guard);

                break
            }

            // TODO: Server 에서 제한 시간을 감지해야 할 경우 제한 시간 내에 멀리건이 진행되지 않았을 경우에 대한 대응 추가 필요

            drop(mulligan_repository_guard);

            tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
        }

        println!("Mulligan monitoring is finished.")
    }
}