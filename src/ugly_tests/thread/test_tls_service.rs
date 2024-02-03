use std::cell::RefCell;
use std::thread;

thread_local! {
    static LOCAL_REPOSITORY: RefCell<ConceptTestRepositoryImpl> =
        RefCell::new(ConceptTestRepositoryImpl::new("Hello, Repository!"));
}

pub trait ConceptTestRepository {
    fn get_data(&self) -> String;
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

impl ConceptTestRepository for ConceptTestRepositoryImpl {
    fn get_data(&self) -> String {
        format!("{} - Count: {}", self.data, self.count)
    }

    fn increment_count(&mut self) {
        self.count += 1;
    }
}

pub trait ConceptTestService {
    fn perform_action(&self, thread_id: usize);
}

pub struct ConceptTestServiceImpl;

impl ConceptTestServiceImpl {
    pub fn new() -> Self {
        ConceptTestServiceImpl
    }
}

impl ConceptTestService for ConceptTestServiceImpl {
    fn perform_action(&self, thread_id: usize) {
        LOCAL_REPOSITORY.with(|r| {
            let mut repository = r.borrow_mut();
            repository.increment_count();
            let data = repository.get_data();
            println!("Thread {}: Result: {}", thread_id, data);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threads() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let service = ConceptTestServiceImpl::new();
                    for _ in 0..1000 {
                        service.perform_action(i);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
