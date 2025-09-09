#[allow(dead_code)]
/// Генерирует случайную карту размером map_width x map_height с индексами тайлов от 0 до tiles_width * tiles_height - 1
pub fn generate_random_map(map_width: u32, map_height: u32, max_tile_index: u32) -> Vec<u32> {
    let mut rng = rand::rng();
    let mut map = Vec::with_capacity((map_width * map_height) as usize);
    for _ in 0..map_width * map_height {
        // Генерируем случайное значение в диапазоне [0, max_tile_index] (включая max_tile_index)
        let cell = rand::Rng::random_range(&mut rng, 0..=max_tile_index);
        // println!("cell: {cell}");
        map.push(cell);
    }
    map
}
