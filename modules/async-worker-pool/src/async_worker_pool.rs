extern crate asn_core_bus;

use asn_core_bus::AsnWorkerPool;
use async_channel::Sender;
use std::sync::Arc;
use std::thread;

// Define the Job type as a boxed function
pub type Job = Box<dyn FnOnce() + Send>;

pub struct AsyncWorkerPool {
    sender: Option<Sender<Job>>,
    handles: Vec<thread::JoinHandle<()>>,
}

impl AsyncWorkerPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = async_channel::bounded::<Job>(32);
        let receiver = Arc::new(receiver);

        let mut handles = Vec::with_capacity(size);

        for _ in 0..size {
            let receiver = receiver.clone();
            let handle = thread::spawn(move || {
                loop {
                    match futures::executor::block_on(receiver.recv()) {
                        Ok(job) => {
                            job();
                        }
                        Err(_) => {
                            // Channel closed, exit the loop
                            break;
                        }
                    }
                }
            });
            handles.push(handle);
        }

        AsyncWorkerPool {
            sender: Some(sender),
            handles,
        }
    }
}

impl<F> AsnWorkerPool<F> for AsyncWorkerPool
where
    F: FnOnce() + Send + 'static,
{
    fn run_main_thread(&self, func: F) -> Result<(), String> {
        if let Some(sender) = &self.sender {
            let job: Job = Box::new(move || {
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
            let job: Job = Box::new(move || {
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

        // Wait for all worker threads to finish
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

pub fn new_async_pool<F: FnOnce() + Send + 'static>(size: usize) -> impl AsnWorkerPool<F> {
    AsyncWorkerPool::new(size)
}
