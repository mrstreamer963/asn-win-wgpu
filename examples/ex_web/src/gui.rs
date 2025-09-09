use crate::gui_handler;
use asn_wgpu::run_with_handler;

pub fn run_gui() {
    let h = gui_handler::get_handler();
    run_with_handler(h);
}
