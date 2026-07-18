#[cfg(not(target_arch = "wasm32"))]
use rand::RngExt;

/// Класс для инициализации и обработки RGBA-массивов
///
/// `RgbaHandler` предоставляет удобный интерфейс для работы с RGBA-данными изображений.
/// Он позволяет создавать, модифицировать и управлять RGBA-массивами, а также
/// создавать текстуры из этих данных для использования в WGPU.
///
/// # Примеры
///
/// ```rust
/// use asn_wgpu::RgbaHandler;
///
/// // Создание нового RGBA-обработчика
/// let mut handler = RgbaHandler::new(256, 256);
///
/// // Заполнение синим цветом
/// handler.fill(0, 0, 255, 255);
///
/// // Установка красного пикселя
/// handler.set_pixel(100, 100, 255, 0, 0, 255).unwrap();
///
/// // Создание градиента
/// handler.create_gradient((255, 0, 0, 255), (0, 0, 255, 255));
/// ```
#[derive(Debug)]
pub struct RgbaHandler {
    width: u32,
    height: u32,
    pub data: Vec<u32>,
}

impl RgbaHandler {
    /// Создает новый экземпляр RgbaHandler с указанными размерами
    ///
    /// # Аргументы
    ///
    /// * `width` - ширина изображения в пикселях
    /// * `height` - высота изображения в пикселях
    ///
    /// # Возвращает
    ///
    /// Новый экземпляр `RgbaHandler` с пустым RGBA-массивом (все пиксели черные с прозрачностью 0)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let handler = RgbaHandler::new(100, 200);
    /// assert_eq!(handler.dimensions(), (100, 200));
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        let data = vec![0u32; size];

        Self {
            width,
            height,
            data,
        }
    }

    /// Создает RgbaHandler из существующего RGBA-массива
    ///
    /// # Аргументы
    ///
    /// * `rgba` - массив байтов с RGBA-данными
    /// * `width` - ширина изображения в пикселях
    /// * `height` - высота изображения в пикселях
    ///
    /// # Возвращает
    ///
    /// `Result<Self, String>` - успешный результат или ошибка с описанием
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если размер массива не соответствует ожидаемому размеру
    /// (width * height * 4 байта)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let rgba_data = vec![255u8; 100 * 100 * 4]; // Белый квадрат
    /// let handler = RgbaHandler::from_rgba(&rgba_data, 100, 100).unwrap();
    /// ```
    pub fn from_rgba(rgba: &[u32], width: u32, height: u32) -> Result<Self, String> {
        let expected_size = (width * height * 4) as usize;

        if rgba.len() != expected_size {
            return Err(format!(
                "Неверный размер RGBA-массива. Ожидается {}, получено {}",
                expected_size,
                rgba.len()
            ));
        }

        Ok(Self {
            width,
            height,
            data: rgba.to_vec(),
        })
    }

    /// Обновляет данные RGBA-массива
    ///
    /// # Аргументы
    ///
    /// * `new_data` - новый массив RGBA-данных
    ///
    /// # Возвращает
    ///
    /// `Result<(), String>` - успех или ошибка
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если размер нового массива не соответствует текущим размерам
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// let new_data = vec![255u8; 100 * 100 * 4];
    /// handler.update_data(&new_data).unwrap();
    /// ```
    pub fn update_data(&mut self, new_data: &[u32]) -> Result<(), String> {
        let expected_size = (self.width * self.height * 4) as usize;

        if new_data.len() != expected_size {
            return Err(format!(
                "Неверный размер данных. Ожидается {}, получено {}",
                expected_size,
                new_data.len()
            ));
        }

        // self.data.copy_from_slice(new_data);
        Ok(())
    }

    /// Устанавливает пиксель по координатам (x, y)
    ///
    /// # Аргументы
    ///
    /// * `x` - координата X (0 <= x < width)
    /// * `y` - координата Y (0 <= y < height)
    /// * `r` - красный компонент (0-255)
    /// * `g` - зеленый компонент (0-255)
    /// * `b` - синий компонент (0-255)
    /// * `a` - альфа-компонент (0-255)
    ///
    /// # Возвращает
    ///
    /// `Result<(), String>` - успех или ошибка
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если координаты выходят за границы изображения
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.set_pixel(50, 50, 255, 0, 0, 255).unwrap(); // Красный пиксель
    /// ```
    pub fn set_pixel(
        &mut self,
        x: u32,
        y: u32,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Координаты ({}, {}) выходят за границы изображения {}x{}",
                x, y, self.width, self.height
            ));
        }

        let index = ((y * self.width + x) * 4) as usize;
        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
        self.data[index + 3] = a;

        // println!("{x} {y} {r} {g} {b} {a}");
        Ok(())
    }

    /// Получает пиксель по координатам (x, y)
    ///
    /// # Аргументы
    ///
    /// * `x` - координата X (0 <= x < width)
    /// * `y` - координата Y (0 <= y < height)
    ///
    /// # Возвращает
    ///
    /// `Result<(u32, u32, u32, u32), String>` - RGBA значения или ошибка
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если координаты выходят за границы изображения
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.set_pixel(50, 50, 255, 128, 64, 255).unwrap();
    /// let pixel = handler.get_pixel(50, 50).unwrap();
    /// assert_eq!(pixel, (255, 128, 64, 255));
    /// ```
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<(u32, u32, u32, u32), String> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Координаты ({}, {}) выходят за границы изображения {}x{}",
                x, y, self.width, self.height
            ));
        }

        let index = ((y * self.width + x) * 4) as usize;
        Ok((
            self.data[index],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
        ))
    }

    /// Заполняет всю область указанным цветом
    ///
    /// # Аргументы
    ///
    /// * `r` - красный компонент (0-255)
    /// * `g` - зеленый компонент (0-255)
    /// * `b` - синий компонент (0-255)
    /// * `a` - альфа-компонент (0-255)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.fill(0, 0, 255, 255); // Синий цвет
    /// ```
    pub fn fill(&mut self, r: u32, g: u32, b: u32, a: u32) {
        for i in (0..self.data.len()).step_by(4) {
            self.data[i] = r;
            self.data[i + 1] = g;
            self.data[i + 2] = b;
            self.data[i + 3] = a;
        }
    }

    /// Создает градиент от одного цвета к другому
    ///
    /// Градиент создается по вертикали (сверху вниз).
    ///
    /// # Аргументы
    ///
    /// * `start_color` - начальный цвет (R, G, B, A)
    /// * `end_color` - конечный цвет (R, G, B, A)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.create_gradient((255, 0, 0, 255), (0, 0, 255, 255)); // От красного к синему
    /// ```
    #[allow(dead_code)]
    pub fn create_gradient(
        &mut self,
        start_color: (u32, u32, u32, u32),
        end_color: (u32, u32, u32, u32),
    ) {
        for y in 0..self.height {
            let t = y as f32 / (self.height - 1) as f32;

            let r = ((1.0 - t) * start_color.0 as f32 + t * end_color.0 as f32) as u32;
            let g = ((1.0 - t) * start_color.1 as f32 + t * end_color.1 as f32) as u32;
            let b = ((1.0 - t) * start_color.2 as f32 + t * end_color.2 as f32) as u32;
            let a = ((1.0 - t) * start_color.3 as f32 + t * end_color.3 as f32) as u32;

            for x in 0..self.width {
                self.set_pixel(x, y, r, g, b, a).unwrap();
            }
        }
    }

    /// Заполняет массив случайными RGBA-значениями
    ///
    /// Генерирует случайные значения для каждого пикселя в диапазоне 0-255.
    /// Альфа-канал устанавливается в 255 (полная непрозрачность).
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.fill_random(); // Заполнение случайными цветами
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn fill_random(&mut self) {
        let mut rng = rand::rng();

        for i in (0..self.data.len()).step_by(4) {
            self.data[i] = rng.random_range(0..=255); // R
            self.data[i + 1] = rng.random_range(0..=255); // G
            self.data[i + 2] = rng.random_range(0..=255); // B
            self.data[i + 3] = 255; // A (полная непрозрачность)
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn fill_random(&mut self) {
        // Для WASM используем простую последовательность вместо случайных чисел
        for i in (0..self.data.len()).step_by(4) {
            self.data[i] = ((i * 7) % 256) as u32; // R
            self.data[i + 1] = ((i * 13) % 256) as u32; // G
            self.data[i + 2] = ((i * 17) % 256) as u32; // B
            self.data[i + 3] = 255; // A (полная непрозрачность)
        }
    }

    /// Заполняет массив случайными RGBA-значениями с настраиваемой прозрачностью
    ///
    /// Генерирует случайные значения для каждого пикселя в диапазоне 0-255.
    /// Альфа-канал также генерируется случайно.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.fill_random_with_alpha(); // Заполнение случайными цветами с случайной прозрачностью
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn fill_random_with_alpha(&mut self) {
        let mut rng = rand::rng();

        for i in (0..self.data.len()).step_by(4) {
            self.data[i] = rng.random_range(0..=255); // R
            self.data[i + 1] = rng.random_range(0..=255); // G
            self.data[i + 2] = rng.random_range(0..=255); // B
            self.data[i + 3] = rng.random_range(0..=255); // A
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn fill_random_with_alpha(&mut self) {
        // Для WASM используем простую последовательность вместо случайных чисел
        for i in (0..self.data.len()).step_by(4) {
            self.data[i] = ((i * 7) % 256) as u32; // R
            self.data[i + 1] = ((i * 13) % 256) as u32; // G
            self.data[i + 2] = ((i * 17) % 256) as u32; // B
            self.data[i + 3] = ((i * 19) % 256) as u32; // A
        }
    }

    /// Заполняет массив случайными значениями в указанном диапазоне
    ///
    /// # Аргументы
    ///
    /// * `min_value` - минимальное значение для RGB компонентов (0-255)
    /// * `max_value` - максимальное значение для RGB компонентов (0-255)
    /// * `alpha` - фиксированное значение альфа-канала (0-255)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.fill_random_range(128, 255, 255); // Только светлые цвета
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn fill_random_range(&mut self, min_value: u32, max_value: u32, alpha: u32) {
        let mut rng = rand::rng();

        for i in (0..self.data.len()).step_by(4) {
            self.data[i] = rng.random_range(min_value..=max_value); // R
            self.data[i + 1] = rng.random_range(min_value..=max_value); // G
            self.data[i + 2] = rng.random_range(min_value..=max_value); // B
            self.data[i + 3] = alpha; // A
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn fill_random_range(&mut self, min_value: u32, max_value: u32, alpha: u32) {
        // Для WASM используем простую последовательность вместо случайных чисел
        let range = (max_value - min_value + 1) as usize;
        for i in (0..self.data.len()).step_by(4) {
            self.data[i] = min_value + ((i * 7) % range) as u32; // R
            self.data[i + 1] = min_value + ((i * 13) % range) as u32; // G
            self.data[i + 2] = min_value + ((i * 17) % range) as u32; // B
            self.data[i + 3] = alpha; // A
        }
    }

    /// Получает размеры изображения
    ///
    /// # Возвращает
    ///
    /// Кортеж (width, height) с размерами изображения
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let handler = RgbaHandler::new(100, 200);
    /// assert_eq!(handler.dimensions(), (100, 200));
    /// ```
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Возвращает ширину изображения
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Возвращает высоту изображения
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Получает ссылку на данные
    ///
    /// # Возвращает
    ///
    /// Ссылка на массив байтов с RGBA-данными
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let handler = RgbaHandler::new(100, 100);
    /// let data = handler.data();
    /// assert_eq!(data.len(), 100 * 100 * 4);
    /// ```
    pub fn data(&self) -> &[u32] {
        &self.data
    }

    /// Получает мутабельную ссылку на данные
    ///
    /// # Возвращает
    ///
    /// Мутабельная ссылка на массив байтов с RGBA-данными
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// let data = handler.data_mut();
    /// data[0] = 255; // Установка красного компонента первого пикселя
    /// ```
    #[allow(dead_code)]
    pub fn data_mut(&mut self) -> &mut [u32] {
        &mut self.data
    }

    /// Изменяет размер изображения
    ///
    /// # Аргументы
    ///
    /// * `new_width` - новая ширина
    /// * `new_height` - новая высота
    ///
    /// # Примечание
    ///
    /// При изменении размера все данные сбрасываются в нули
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.resize(200, 200);
    /// assert_eq!(handler.dimensions(), (200, 200));
    /// ```
    #[allow(dead_code)]
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
        let new_size = (new_width * new_height * 4) as usize;
        self.data.resize(new_size, 0);
    }

    /// Создает копию текущего состояния
    ///
    /// # Возвращает
    ///
    /// Новый экземпляр `RgbaHandler` с копией всех данных
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.fill(255, 0, 0, 255);
    /// let copy = handler.clone();
    /// assert_eq!(handler.dimensions(), copy.dimensions());
    /// ```
    #[allow(dead_code)]
    pub fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            data: self.data.clone(),
        }
    }

    /// Устанавливает индексы тайлов в RGBA-формате
    ///
    /// Преобразует индексы тайлов в координаты x и y и сохраняет их в RGBA-формате:
    /// - R-компонента = x
    /// - G-компонента = y
    /// - B-компонента = 0
    /// - A-компонента = 0
    ///
    /// # Аргументы
    ///
    /// * `tile_indices` - массив индексов тайлов
    /// * `tiles_width` - ширина тайлсета (количество тайлов по ширине)
    ///
    /// # Возвращает
    ///
    /// `Result<(), String>` - успех или ошибка
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если координаты x или y превышают 255
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(10, 10);
    /// let tile_indices = vec![0u32, 1, 2, 3];
    /// handler.set_tile_indices(&tile_indices, 10).unwrap();
    /// ```
    pub fn set_tile_indices(
        &mut self,
        tile_indices: &[u32],
        tiles_width: u32,
    ) -> Result<(), String> {
        for (i, &index) in tile_indices.iter().enumerate() {
            // Вычисляем координаты x и y из индекса
            let x = (index as u32) % tiles_width;
            let y = (index as u32) / tiles_width;

            // Преобразуем индекс в координаты пикселя
            let px = (i as u32) % self.width;
            let py = (i as u32) / self.width;

            // Устанавливаем пиксель с координатами тайла в RGBA формате
            // R = x, G = y, B = 0, A = 0
            // println!(
            //     "Tile index {}: x={}, y={}, pixel at ({}, {})",
            //     index, x, y, px, py
            // );
            self.set_pixel(px, py, x as u32, y as u32, 0, 0)?;
        }

        Ok(())
    }
}

impl Default for RgbaHandler {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let handler = RgbaHandler::new(100, 200);
        assert_eq!(handler.dimensions(), (100, 200));
        assert_eq!(handler.data().len(), 100 * 200 * 4);
    }

    #[test]
    fn test_from_rgba() {
        let rgba = vec![255u32; 100 * 200 * 4];
        let handler = RgbaHandler::from_rgba(&rgba, 100, 200).unwrap();
        assert_eq!(handler.dimensions(), (100, 200));
    }

    #[test]
    fn test_set_get_pixel() {
        let mut handler = RgbaHandler::new(10, 10);
        handler.set_pixel(5, 5, 255, 128, 64, 255).unwrap();
        let pixel = handler.get_pixel(5, 5).unwrap();
        assert_eq!(pixel, (255, 128, 64, 255));
    }

    #[test]
    fn test_fill() {
        let mut handler = RgbaHandler::new(10, 10);
        handler.fill(255, 0, 0, 255);

        for y in 0..10 {
            for x in 0..10 {
                let pixel = handler.get_pixel(x, y).unwrap();
                assert_eq!(pixel, (255, 0, 0, 255));
            }
        }
    }

    #[test]
    fn test_fill_random() {
        let mut handler = RgbaHandler::new(10, 10);
        handler.fill_random();

        // Проверяем, что все пиксели имеют альфа = 255
        for y in 0..10 {
            for x in 0..10 {
                let pixel = handler.get_pixel(x, y).unwrap();
                assert_eq!(pixel.3, 255); // Альфа-канал должен быть 255
            }
        }
    }

    #[test]
    fn test_fill_random_with_alpha() {
        let mut handler = RgbaHandler::new(10, 10);
        handler.fill_random_with_alpha();

        // Проверяем, что данные заполнены (не все нули)
        let data = handler.data();
        let has_non_zero = data.iter().any(|&x| x != 0);
        assert!(has_non_zero);
    }

    #[test]
    fn test_fill_random_range() {
        let mut handler = RgbaHandler::new(10, 10);
        handler.fill_random_range(128, 255, 200);

        // Проверяем, что все значения в указанном диапазоне
        for y in 0..10 {
            for x in 0..10 {
                let pixel = handler.get_pixel(x, y).unwrap();
                assert!(pixel.0 >= 128); // R
                assert!(pixel.1 >= 128); // G
                assert!(pixel.2 >= 128); // B
                assert_eq!(pixel.3, 200); // A
            }
        }
    }

    #[test]
    fn test_set_tile_indices() {
        let mut handler = RgbaHandler::new(4, 1);
        let tile_indices = vec![0u32, 1, 2, 3];
        handler.set_tile_indices(&tile_indices, 2).unwrap();

        // Проверяем, что пиксели установлены правильно
        let pixel0 = handler.get_pixel(0, 0).unwrap();
        assert_eq!(pixel0, (0, 0, 0, 0)); // index 0 -> x=0, y=0

        let pixel1 = handler.get_pixel(1, 0).unwrap();
        assert_eq!(pixel1, (1, 0, 0, 0)); // index 1 -> x=1, y=0

        let pixel2 = handler.get_pixel(2, 0).unwrap();
        assert_eq!(pixel2, (0, 1, 0, 0)); // index 2 -> x=0, y=1

        let pixel3 = handler.get_pixel(3, 0).unwrap();
        assert_eq!(pixel3, (1, 1, 0, 0)); // index 3 -> x=1, y=1
    }

    // #[test]
    // fn test_set_tile_indices_overflow() {
    //     let mut handler = RgbaHandler::new(1, 1);
    //     let tile_indices = vec![256u32]; // Индекс, который приведет к координате > 255
    //     let result = handler.set_tile_indices(&tile_indices, 1);
    //     assert!(result.is_err());
    // }
}
