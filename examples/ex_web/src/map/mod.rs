use crate::map::map_utils::generate_random_map;
mod map_utils;

pub struct Map {
    map_width: u32,
    map_height: u32,
    cells: Vec<u32>,
    is_need_update: bool,
}

pub fn get_map() -> Map {
    let map_width = 32;
    let map_height = 32;
    let cells = generate_random_map(map_width, map_height, map_width * map_height - 1);

    Map {
        map_width,
        map_height,
        cells,
        is_need_update: true,
    }
}
