use crate::gui_handler::time::Instant;
use asn_core::cgmath::Vector3;
use asn_core::transform_set::TransformSet;
use asn_wgpu::WgpuGraphContext;
use wgpu_map::{MapParams, MapTilesParams, WgpuMap};

pub const LOOP_MILLIS: u128 = 5;

pub struct GuiHandlerState {
    pub m: WgpuMap,
    pub last_update: Instant,
}

impl GuiHandlerState {
    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update).as_millis() >= LOOP_MILLIS as u128 {
            self.last_update = now;

            // let map = generate_random_map(
            //     self.map_width,
            //     self.map_height,
            //     self.tiles_width * self.tiles_height - 1,
            // );
            // self.m.update_map(&map);
            // Здесь может быть дополнительная логика обновления, если она есть
        }
    }
}

pub fn new_handler_state(
    gcx: &WgpuGraphContext,
    map_params: &MapParams,
    tiles_params: &MapTilesParams,
) -> GuiHandlerState {
    // let map_tiles_bytes = include_bytes!("../../../tiles_64_95.png");
    // let tiles_width = 64;
    // let tiles_height = 95;

    // let map_tiles_bytes = include_bytes!("../../../tiles_16_12.png");
    // let tiles_width = 16;
    // let tiles_height = 12;

    // let tiles_params = MapTilesParams {
    //     map_tiles_bytes,
    //     tiles_width,
    //     tiles_height,
    // };

    // let map_width = 32;
    // let map_height = 32;
    // let map = generate_random_map(map_width, map_height, map_width * map_height - 1);

    // let map_params = MapParams {
    //     map_width,
    //     map_height,
    //     tile_indices: &map,
    // };

    let m = wgpu_map::get_map(gcx, &tiles_params, &map_params);

    let s = TransformSet {
        pos: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rot: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: Vector3 {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        },
    };

    // Создаем и устанавливаем MVP-матрицу
    let mvp_matrix = s.matrix_calculated();
    m.update_mvp_matrix(gcx, mvp_matrix.into());

    GuiHandlerState {
        m,
        last_update: Instant::now(),
    }
}
