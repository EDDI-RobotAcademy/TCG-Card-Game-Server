use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::runtime::Handle;
use tokio::task;
use crate::thread_control::entity::closure::Closure;
use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;
use crate::thread_control::repository::thread_worker_repository_impl::ThreadWorkerRepositoryImpl;
use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;

pub struct ThreadWorkerServiceImpl {
    repository: Arc<Mutex<ThreadWorkerRepositoryImpl>>,
}

impl ThreadWorkerServiceImpl {
    pub fn new(repository: Arc<Mutex<ThreadWorkerRepositoryImpl>>) -> Self {
        ThreadWorkerServiceImpl { repository }
    }

    pub fn get_instance() -> Arc<Mutex<ThreadWorkerServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ThreadWorkerServiceImpl>> =
                Arc::new(Mutex::new(ThreadWorkerServiceImpl::new(ThreadWorkerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ThreadWorkerServiceTrait for ThreadWorkerServiceImpl {

    fn save_sync_thread_worker<F>(&mut self, name: &str, will_be_execute_function: F)
        where
            F: FnOnce() -> () + 'static,
    {
        println!("save sync closure service");
        let closure = Closure::Sync(Box::new(will_be_execute_function));
        self.repository.lock().unwrap().save_thread_worker(name, Some(closure));
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[allow(dead_code)]
    fn my_sync_function() {
        println!("Synchronous function is executed!");
    }

    #[allow(dead_code)]
    async fn my_async_function() {
        println!("Asynchronous function is executed!");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_async_thread_worker() {
        let thread_worker_repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(thread_worker_repository);

        let async_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                println!("Custom async function executed!");
            })
        };

        service.save_async_thread_worker("AsyncTestWorker", Box::new(async_function.clone()));
        service.start_thread_worker("AsyncTestWorker").await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_sync_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(repository);

        let sync_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                println!("Custom sync function executed!");
            })
        };

        service.save_async_thread_worker("SyncTestWorker", Box::new(sync_function));
        service.start_thread_worker("SyncTestWorker").await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_async_thread_and_start() {
        let thread_worker_repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(thread_worker_repository);

        let async_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                println!("Custom async function executed!");
            })
        };

        service.save_async_thread_worker("AsyncTestWorker2", Box::new(async_function.clone()));
        service.start_thread_worker("AsyncTestWorker2").await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_sync_thread_and_start() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(repository);

        let sync_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                println!("Custom sync function executed!");
            })
        };

        service.save_async_thread_worker("SyncTestWorker2", Box::new(sync_function));
        service.start_thread_worker("SyncTestWorker2").await;
    }

    #[test]
    async fn test_singleton() {
        let instance1 = ThreadWorkerServiceImpl::get_instance();
        let instance2 = ThreadWorkerServiceImpl::get_instance();

        assert_eq!(Arc::ptr_eq(&instance1, &instance2), true);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_sync_thread_and_start_with_singleton() {
        let service_instance = ThreadWorkerServiceImpl::get_instance();
        let mut service = service_instance.lock().unwrap();

        let sync_custom_function = || {
            println!("Custom sync function executed!");
        };

        service.save_sync_thread_worker("SyncTestWorker3", Box::new(sync_custom_function));
        service.start_thread_worker("SyncTestWorker3").await;
    }
}