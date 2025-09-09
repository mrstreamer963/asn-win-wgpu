use crate::gui_handler;
use asn_wgpu::run_with_handler;

#[allow(dead_code)]
const LOG_MODULE_NAME: &str = "gui_wgpu";

#[allow(dead_code)]
pub fn run_gui() {
    let h = gui_handler::get_handler();
    run_with_handler(h);
}
