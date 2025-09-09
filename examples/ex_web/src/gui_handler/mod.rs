use asn_logger::*;
pub const LOG_MODULE_NAME: &str = "DummyGuiHandler";

mod gui_handler_state;
mod time;
use asn_gui_core::{TAsnGuiElement, TAsnGuiHandler};
use asn_wgpu::{WgpuFrameContext, WgpuGraphContext, WgpuGuiHandler};
use gui_handler_state::GuiHandlerState;
use gui_handler_state::new_handler_state;

enum WebGuiHandler {
    Zero,
    Loaded(GuiHandlerState),
}

impl TAsnGuiHandler for WebGuiHandler {
    type GraphContext = WgpuGraphContext;
    type FrameContext = WgpuFrameContext;

    fn init(&mut self, gcx: &Self::GraphContext) {
        m_info!("init");
        let h = new_handler_state(gcx);
        *self = WebGuiHandler::Loaded(h);
    }

    fn update(&mut self, gcx: &Self::GraphContext) {
        if let Self::Loaded(h) = self {
            h.update();
            h.m.update(gcx);
        }
    }

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        if let Self::Loaded(h) = self {
            h.m.draw(fcx);
        }
    }
}

pub fn get_handler() -> impl WgpuGuiHandler {
    WebGuiHandler::Zero
}
