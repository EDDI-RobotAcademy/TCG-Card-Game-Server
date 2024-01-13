use std::future::Future;
use std::pin::Pin;

pub enum Closure {
    Async(Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>>>),
    Sync(Box<dyn FnOnce() -> ()>),
}
