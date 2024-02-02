use tokio::sync::{RwLock, Mutex};
use once_cell::sync::Lazy;

#[async_trait::async_trait]
pub trait ConceptTestRepository {
    async fn get_data(&self) -> String;
    fn increment_count(&mut self);
}

pub struct ConceptTestRepositoryImpl {
    data: String,
    count: u32,
}

impl ConceptTestRepositoryImpl {
    pub fn new(data: &str) -> Self {
        ConceptTestRepositoryImpl {
            data: data.to_string(),
            count: 0,
        }
    }

    pub fn increment_count(&mut self) {
        self.count += 1;
    }
}

#[async_trait::async_trait]
impl ConceptTestRepository for ConceptTestRepositoryImpl {
    async fn get_data(&self) -> String {
        format!("{} - Count: {}", self.data, self.count)
    }

    fn increment_count(&mut self) {
        self.count += 1;
    }
}

#[async_trait::async_trait]
pub trait ConceptTestService {
    async fn perform_action(&self, thread_id: usize);
}

pub struct ConceptTestServiceImpl;

impl ConceptTestServiceImpl {
    pub fn new() -> Self {
        ConceptTestServiceImpl
    }
}

#[async_trait::async_trait]
impl ConceptTestService for ConceptTestServiceImpl {
    async fn perform_action(&self, thread_id: usize) {
        let mut repository = LOCAL_REPOSITORY.write().await;
        repository.increment_count();
        let data = repository.get_data().await;
        println!("Thread {}: Result: {}", thread_id, data);
    }
}

static LOCAL_REPOSITORY: Lazy<RwLock<ConceptTestRepositoryImpl>> = Lazy::new(|| {
    RwLock::new(ConceptTestRepositoryImpl::new("Hello, Repository!"))
});

#[cfg(test)]
mod tests {
    use tokio::task;
    use super::*;

    #[tokio::test]
    async fn test_threads() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                task::spawn(async move {
                    let service = ConceptTestServiceImpl::new();
                    for _ in 0..1000 {
                        service.perform_action(i).await;
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.await.unwrap();
        }
    }
}
