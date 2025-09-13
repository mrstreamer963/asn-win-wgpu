use asn_core_bus::{AsnBus, AsnReceiver, AsnTransmitter};
use asn_logger::init_log;
use asn_logger::*;
use tokio_bus::new_tokio_bus;
use wasm_bindgen::prelude::*;

const LOG_MODULE_NAME: &str = "ex_web_bus";

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum TaskType {
    TaskNone,
    TaskUpdate,
}

// Для простоты реализации будем создавать новый bus каждый раз при необходимости
// В реальном приложении лучше использовать более сложное управление состоянием

#[wasm_bindgen]
pub fn init_web_app() -> Result<(), JsValue> {
    setup_log().unwrap();

    // Простая инициализация для web
    m_info!("Hello from init_web_app");
    Ok(())
}

#[wasm_bindgen]
pub fn send_task_update() -> String {
    m_info!("send_task_update");
    match send_message_internal(TaskType::TaskUpdate) {
        Ok(_) => "Message sent successfully".to_string(),
        Err(e) => format!("Error sending message: {}", e),
    }
}

#[wasm_bindgen]
pub fn send_task_none() -> String {
    m_info!("send_task_none");

    match send_message_internal(TaskType::TaskNone) {
        Ok(_) => "Message sent successfully".to_string(),
        Err(e) => format!("Error sending message: {}", e),
    }
}

#[wasm_bindgen]
pub fn receive_message() -> String {
    m_info!("receive_message");

    // Для демонстрации создаем новый receiver и пытаемся получить сообщение
    let bus = new_tokio_bus::<TaskType>(16);
    let mut receiver = bus.get_receiver();

    match receiver.get_message() {
        Ok(msg) => format!("Received: {:?}", msg),
        Err(e) => format!("No messages available: {:?}", e),
    }
}

#[wasm_bindgen]
pub fn get_version() -> String {
    m_info!("get_version");
    "ASN Web Bus v0.1.0".to_string()
}

// Вспомогательная функция для отправки сообщений
fn send_message_internal(message: TaskType) -> Result<(), String> {
    m_info!("send_message_internal");

    let bus = new_tokio_bus::<TaskType>(16);
    let sender = bus.get_sender();

    sender
        .send_message(message)
        .map_err(|e| format!("Failed to send message: {:?}", e))
}

fn setup_log() -> Result<(), String> {
    let c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    init_log(&c)
}
