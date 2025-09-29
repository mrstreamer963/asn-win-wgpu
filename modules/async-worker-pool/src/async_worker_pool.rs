extern crate asn_core_bus;

use asn_core_bus::AsnWorkerPool;
use async_channel::{Receiver, Sender};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

// Define the Job type as a boxed async function that takes worker ID
pub type Job = Box<dyn FnOnce(usize) + Send + 'static>;

pub struct AsyncWorker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl AsyncWorker {
    pub fn new<F>(id: usize, receiver: Arc<Receiver<Job>>, rt: Arc<Runtime>) -> AsyncWorker
    where
        F: FnOnce() + Send + 'static,
    {
        let handle = rt.spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job(id);
                        println!("Worker {id} job ending.");
                    }
                    Err(_) => {
                        // Channel closed, exit the loop
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        AsyncWorker {
            id,
            handle: Some(handle),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn take_handle(&mut self) -> Option<JoinHandle<()>> {
        self.handle.take()
    }
}

pub struct AsyncWorkerPool {
    rt: Arc<Runtime>,
    workers: Vec<AsyncWorker>,
    sender: Option<Sender<Job>>,
}

impl AsyncWorkerPool {
    pub fn new(size: usize) -> Self {
        let rt = Arc::new(Runtime::new().unwrap());
        let (sender, receiver) = async_channel::bounded::<Job>(32);
        let receiver = Arc::new(receiver);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = AsyncWorker::new::<Box<dyn FnOnce() + Send + 'static>>(
                id,
                receiver.clone(),
                rt.clone(),
            );
            workers.push(worker);
        }

        AsyncWorkerPool {
            rt,
            workers,
            sender: Some(sender),
        }
    }
}

impl<F> AsnWorkerPool<F> for AsyncWorkerPool
where
    F: FnOnce() + Send + 'static,
{
    fn run_main_thread(&self, func: F) -> Result<(), String> {
        if let Some(sender) = &self.sender {
            let job: Job = Box::new(|_worker_id| {
                func();
            });
            match sender.try_send(job) {
                Ok(()) => Ok(()),
                Err(_) => Err("Failed to send job to worker pool".to_string()),
            }
        } else {
            Err("Worker pool sender closed".to_string())
        }
    }

    fn run_thread(&self, func: F) -> Result<(), String> {
        if let Some(sender) = &self.sender {
            let job: Job = Box::new(|_worker_id| {
                func();
            });
            match sender.try_send(job) {
                Ok(()) => Ok(()),
                Err(_) => Err("Failed to send job to worker pool".to_string()),
            }
        } else {
            Err("Worker pool sender closed".to_string())
        }
    }
}

impl Drop for AsyncWorkerPool {
    fn drop(&mut self) {
        // Close the sender to signal workers to stop
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }

        // Wait briefly for workers to finish
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            // If we're already in a tokio runtime, spawn a task to wait
            let workers_handles: Vec<_> = self
                .workers
                .iter_mut()
                .filter_map(|worker| worker.take_handle())
                .collect();

            for handle in workers_handles {
                let _ = handle;
            }
        } else {
            // Otherwise, we need to create a runtime to wait for tasks
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let workers_handles: Vec<_> = self
                    .workers
                    .iter_mut()
                    .filter_map(|worker| worker.take_handle())
                    .collect();

                for handle in workers_handles {
                    let _ = handle.await;
                }
            });
        }
    }
}

pub fn new_async_pool<F: FnOnce() + Send + 'static>(size: usize) -> impl AsnWorkerPool<F> {
    AsyncWorkerPool::new(size)
}
