use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::runtime::Handle;
use tokio::{spawn, task};
use crate::thread_control::entity::closure::Closure;
use crate::thread_control::entity::thread_worker::ThreadWorker;
use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;

pub struct ThreadWorkerRepositoryImpl {
    thread_worker_list: Arc<Mutex<HashMap<String, ThreadWorker>>>,
}

impl ThreadWorkerRepositoryImpl {
    pub fn new() -> Self {
        ThreadWorkerRepositoryImpl {
            thread_worker_list: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_instance() -> Arc<Mutex<ThreadWorkerRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ThreadWorkerRepositoryImpl>> = {
                let shared_data = Arc::new(Mutex::new(ThreadWorkerRepositoryImpl::new()));

                Arc::clone(&shared_data)
            };
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ThreadWorkerRepositoryTrait for ThreadWorkerRepositoryImpl {

    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Option<Closure>,
    ) {
        let mut thread_worker_list = self.thread_worker_list.lock().unwrap();

        let mut thread_worker = ThreadWorker::new(name);
        thread_worker.save_function(will_be_execute_function.unwrap());

        thread_worker_list.insert(name.to_string(), thread_worker);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    fn my_sync_function() {
        println!("Synchronous function is executed!");
    }

    async fn my_async_function() {
        println!("Asynchronous function is executed!");
    }

    #[test]
    async fn test_singleton() {
        let instance1 = ThreadWorkerRepositoryImpl::get_instance();
        let instance2 = ThreadWorkerRepositoryImpl::get_instance();

        assert_eq!(Arc::ptr_eq(&instance1, &instance2), true);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        let mut repository = repository.lock().unwrap();

        let custom_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        repository.save_thread_worker("TestWorker", Some(Closure::Async(Box::new(custom_function.clone()))));
        repository.start_thread_worker("TestWorker").await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_sync_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        let mut repository = repository.lock().unwrap();

        let sync_custom_function = || {
            my_sync_function();
        };

        repository.save_thread_worker("SyncTestWorker", Some(Closure::Sync(Box::new(sync_custom_function))));
        repository.start_thread_worker("SyncTestWorker").await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_async_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        let mut repository = repository.lock().unwrap();

        let async_custom_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                my_async_function().await;
            })
        };

        repository.save_thread_worker("AsyncTestWorker", Some(Closure::Async(Box::new(async_custom_function))));
        repository.start_thread_worker("AsyncTestWorker").await;
    }
}
