use async_trait::async_trait;
use crate::thread_worker::entity::closure::Closure;

#[async_trait]
pub trait ThreadWorkerRepositoryTrait {
    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Option<Closure>,
    );
    async fn start_thread_worker(&self, name: &str);
}