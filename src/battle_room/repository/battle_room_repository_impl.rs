// use std::sync::Arc;
// use async_trait::async_trait;
// use lazy_static::lazy_static;
//
//
// pub struct BattleRoomWaitQueueRepositoryImpl {
//     wait_queue: BattleRoomWaitingQueue,
// }
//
// impl crate::battle_room::repository::battle_room_wait_queue_repository_impl::BattleRoomWaitQueueRepositoryImpl {
//     pub fn new() -> Self {
//         crate::battle_room::repository::battle_room_wait_queue_repository_impl::BattleRoomWaitQueueRepositoryImpl {
//             wait_queue: BattleRoomWaitingQueue::new(),
//         }
//     }
//
//     pub fn get_instance() -> Arc<AsyncMutex<crate::battle_room::repository::battle_room_wait_queue_repository_impl::BattleRoomWaitQueueRepositoryImpl>> {
//         lazy_static! {
//             static ref INSTANCE: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>> =
//                 Arc::new(AsyncMutex::new(BattleRoomWaitQueueRepositoryImpl::new()));
//         }
//         crate::battle_room::repository::battle_room_wait_queue_repository_impl::INSTANCE.clone()
//     }
// }
//
// #[async_trait]
// impl BattleRoomWaitQueueRepository for crate::battle_room::repository::battle_room_wait_queue_repository_impl::BattleRoomWaitQueueRepositoryImpl {