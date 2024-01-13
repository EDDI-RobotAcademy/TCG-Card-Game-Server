use crate::domain_initializer::initializer::DomainInitializer;

mod thread_worker;
mod common;
mod domain_initializer;

#[tokio::main]
async fn main() {
    let domain_initializer = DomainInitializer;
    domain_initializer.init_every_domain().await;
}
