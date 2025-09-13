use asn_core_bus::{AsnBus, AsnTransmitter};
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

// Вспомогательная функция для отправки сообщений
pub fn send_message_internal(message: TaskType) -> Result<(), String> {
    let bus = get_bus();
    let sender = bus.get_sender();

    sender
        .send_message(message)
        .map_err(|e| format!("Failed to send message: {:?}", e))
}
