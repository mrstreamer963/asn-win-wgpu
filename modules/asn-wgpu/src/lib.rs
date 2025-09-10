extern crate asn_winit;

mod data;
pub mod render_manager;
mod state_error;

use crate::render_manager::RenderManager;
use asn_gui_core::TAsnGuiHandler;
use asn_logger::*;
pub use state_error::StateError;
use std::sync::{Arc, Mutex};

use data::LOG_MODULE_NAME;

// reexport
pub use wgpu;

pub use render_manager::WgpuFrameContext;
pub use render_manager::WgpuGraphContext;

pub trait WgpuGuiHandler:
    TAsnGuiHandler<
        GraphContext = render_manager::WgpuGraphContext,
        FrameContext = render_manager::WgpuFrameContext,
    >
{
}

// Blanket implementation
impl<T> WgpuGuiHandler for T where
    T: TAsnGuiHandler<
            GraphContext = render_manager::WgpuGraphContext,
            FrameContext = render_manager::WgpuFrameContext,
        >
{
}

// Выдаем на выход TAsnGuiHandler совместимый с WinitRenderManager
pub fn get_manager<H: WgpuGuiHandler>(h: Arc<Mutex<H>>) -> impl asn_winit::WinitRenderManager {
    RenderManager::new(h)
}

// Функция-дженерик для облегчения запуска WgpuGuiHandler
pub fn run_with_handler<H>(h: H)
where
    H: WgpuGuiHandler + 'static,
{
    let h_safe = Arc::new(Mutex::new(h));

    let r = get_manager(h_safe);

    match asn_winit::run(r) {
        Ok(_) => {
            m_info!("Application finished successfully");
        }
        Err(e) => {
            m_error!("Application failed with error: {}", e);
        }
    }
}
