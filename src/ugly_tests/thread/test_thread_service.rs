use tokio::sync::Mutex;
use async_trait::async_trait;
use std::sync::Arc;

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
    async fn perform_action(&self) -> String;
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
    async fn perform_action(&self) -> String {
        let repository = self.repository.lock().await;
        repository.get_data().await
    }
}

#[tokio::test]
async fn test_thread_service() {
    let shared_repository = Arc::new(Mutex::new(ConceptTestRepositoryImpl::new("Hello, Repository!")));
    let service = ConceptTestServiceImpl::new(shared_repository);
    let result = service.perform_action().await;
    println!("Result from service: {}", result);
}
