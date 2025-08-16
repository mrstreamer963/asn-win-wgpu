extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_winit;

mod log_utils;
use std::{sync::Arc, time::Duration};

use asn_gui_core::{TAsnGuiHandler, TAsnRenderManager};
use asn_logger::*;
use asn_winit::WinitWindow;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_winit";

pub struct DummyRenderManager {}
pub struct DummyGuiHandler {}

impl TAsnRenderManager for DummyRenderManager {
    type Window = WinitWindow;

    fn init(&mut self, w: Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        let _ = w;
        m_info!("init()");
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        m_info!("resize from main() {:?} {:?}", width, height);
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        m_info!("draw");
        Ok(())
    }
}

impl TAsnGuiHandler for DummyGuiHandler {
    type GraphContext = ();
    type FrameContext = ();

    fn init(&mut self, gcx: &Self::GraphContext) {
        t_info!("TAsnGuiHandler", "init()");
        let _ = gcx;
    }

    fn update(&mut self) {
        t_info!("TAsnGuiHandler", "update()");
    }

    fn draw(&mut self, fcx: &Self::FrameContext) {
        t_info!("TAsnGuiHandler", "draw()");
        let _ = fcx;
    }
}

async fn update() {
    m_info!("update");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    m_info!("hello from main()");

    {
        let r = DummyRenderManager {};
        asn_winit::run(r)?;
    }

    loop {
        pollster::block_on(update());
        std::thread::sleep(Duration::from_secs(1));
    }
}
