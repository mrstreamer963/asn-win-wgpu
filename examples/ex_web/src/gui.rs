use crate::gui_handler;
use crate::map::get_map;
use asn_wgpu::run_with_handler;

pub fn run_gui() {
    let map_width = 32;
    let map_height = 32;
    let map = get_map(map_width, map_height);

    let map_tiles_bytes = include_bytes!("../../tiles_16_12.png");
    let tiles_width = 16;
    let tiles_height = 12;

    // прокидывать map --> GuiMap через очередь (?) - как изначально в движке и планировалось
    // инициализировать GuiMap по дефолту пикселем 1x1 (?)
    // model-view-controller (?) mvc через очередь! :3

    let h = gui_handler::get_handler();

    run_with_handler(h);
}
