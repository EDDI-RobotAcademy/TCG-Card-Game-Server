use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

#[async_trait]
pub trait ThreadWorkerServiceTrait {

    fn save_sync_thread_worker<F>(&mut self, name: &str, will_be_execute_function: F)
        where
            F: FnOnce() -> () + 'static;

}

