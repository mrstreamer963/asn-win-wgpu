pub trait AsnWorkerPool {
    fn run_main_thread(&self, func: impl FnOnce() + Send + 'static) -> Result<(), String>;
    fn run_thread(&self, func: impl FnOnce() + Send + 'static) -> Result<(), String>;
}
