use tokio::sync::Mutex;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task;

#[async_trait]
pub trait ConceptTestRepository: Send + Sync {
    async fn get_data(&self) -> String;
}

pub struct ConceptTestRepositoryImpl {
    data: String,
}

impl ConceptTestRepositoryImpl {
    pub fn new(data: &str) -> Self {
        ConceptTestRepositoryImpl {
            data: data.to_string(),
        }
    }
}

#[async_trait]
impl ConceptTestRepository for ConceptTestRepositoryImpl {
    async fn get_data(&self) -> String {
        self.data.clone()
    }
}

#[async_trait]
pub trait ConceptTestService {
    async fn perform_action(&self, thread_id: usize) -> String;
}

pub struct ConceptTestServiceImpl {
    repository: Arc<Mutex<dyn ConceptTestRepository>>,
}

impl ConceptTestServiceImpl {
    pub fn new(repository: Arc<Mutex<dyn ConceptTestRepository>>) -> Self {
        ConceptTestServiceImpl { repository }
    }
}

#[async_trait]
impl ConceptTestService for ConceptTestServiceImpl {
    async fn perform_action(&self, thread_id: usize) -> String {
        let repository = self.repository.lock().await;
        let data = repository.get_data().await;
        format!("Thread {}: {}", thread_id, data)
    }
}

#[tokio::test]
async fn test_another_thread_service() {
    let shared_repository: Arc<Mutex<dyn ConceptTestRepository>> =
        Arc::new(Mutex::new(ConceptTestRepositoryImpl::new("Hello, Repository!")));

    let mut handles = vec![];

    for i in 0..10 {
        let service = ConceptTestServiceImpl::new(Arc::clone(&shared_repository));

        let handle = tokio::spawn(async move {
            for _ in 0..1000 {
                let result = service.perform_action(i).await;
                println!("Result from thread {}: {}", i, result);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
