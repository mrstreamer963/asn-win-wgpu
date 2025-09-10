use crate::map::map_utils::generate_random_map;
mod map_utils;

pub struct Map {
    map_width: u32,
    map_height: u32,
    cells: Vec<u32>,
    is_need_update: bool,
}

// getters
impl Map {
    pub fn map_width(&self) -> u32 {
        self.map_width
    }

    pub fn map_height(&self) -> u32 {
        self.map_height
    }

    pub fn cells(&self) -> &[u32] {
        &self.cells
    }

    pub fn is_need_update(&self) -> bool {
        self.is_need_update
    }
}

pub fn get_map(map_width: u32, map_height: u32) -> Map {
    let cells = generate_random_map(map_width, map_height, map_width * map_height - 1);

    Map {
        map_width,
        map_height,
        cells,
        is_need_update: true,
    }
}
