use crate::thread_worker::service::thread_worker_service_impl::ThreadWorkerServiceImpl;

pub struct DomainInitializer;

impl DomainInitializer {
    pub fn init_thread_worker_domain(&self) {
        let _ = ThreadWorkerServiceImpl::get_instance();
    }

    pub async fn init_every_domain(&self) {
        self.init_thread_worker_domain();
    }
}

