use std::future::Future;
use std::pin::Pin;

pub struct CustomFuture(pub Pin<Box<dyn Future<Output = ()> + Send>>);
