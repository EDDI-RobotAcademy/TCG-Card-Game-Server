use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;
use crate::thread_worker::service::thread_worker_service_impl::ThreadWorkerServiceImpl;

use crate::common::mpsc::mpsc_creator::mpsc_channel::define_channel;

define_channel!(AcceptorReceiverChannel, Arc<Mutex<TcpStream>>);

pub struct DomainInitializer;

impl DomainInitializer {
    pub fn init_server_socket_domain(&self) {
        let _ = ServerSocketServiceImpl::get_instance();
    }
    pub fn init_thread_worker_domain(&self) {
        let _ = ThreadWorkerServiceImpl::get_instance();
    }

    pub async fn init_every_domain(&self) {
        self.init_server_socket_domain();
        self.init_thread_worker_domain();
    }
}

