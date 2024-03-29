use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

#[async_trait]
pub trait ThreadWorkerServiceTrait {
    fn save_async_thread_worker<F>(&mut self, name: &str, will_be_execute_function: F)
        where
            F: FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + 'static;
    fn save_sync_thread_worker<F>(&mut self, name: &str, will_be_execute_function: F)
        where
            F: FnOnce() -> () + 'static;
    async fn start_thread_worker(&self, name: &str);
}

