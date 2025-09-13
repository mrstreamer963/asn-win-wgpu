use std::sync::OnceLock;
use tokio_bus::{TokioEventBus, new_tokio_bus};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TaskType {
    TaskNone,
    TaskUpdate,
}

pub type WebBus = TokioEventBus<TaskType>;

// Глобальная переменная для хранения шины данных
static BUS: OnceLock<WebBus> = OnceLock::new();

// Функция для получения ссылки на шину данных
pub fn get_bus() -> &'static WebBus {
    BUS.get_or_init(|| new_tokio_bus::<TaskType>(16))
}
