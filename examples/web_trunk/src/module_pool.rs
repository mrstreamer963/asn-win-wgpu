use asn_core_bus::AsnWorkerPool;
use async_worker_pool::new_async_pool;

pub fn new_worker_pool() -> impl AsnWorkerPool {
    new_async_pool(16)
}
