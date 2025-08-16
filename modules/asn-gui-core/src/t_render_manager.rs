use std::sync::Arc;

pub trait TAsnRenderManager {
    type Window;

    fn init(&mut self, w: Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>>;

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>>;

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
