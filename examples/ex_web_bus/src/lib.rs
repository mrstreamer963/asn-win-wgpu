use asn_core_bus::{AsnBus, AsnReceiver, AsnTransmitter};
use tokio_bus::new_tokio_bus;
use wasm_bindgen::prelude::*;

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
    // Простая инициализация для web
    Ok(())
}

#[wasm_bindgen]
pub fn send_task_update() -> String {
    match send_message_internal(TaskType::TaskUpdate) {
        Ok(_) => "Message sent successfully".to_string(),
        Err(e) => format!("Error sending message: {}", e),
    }
}

#[wasm_bindgen]
pub fn send_task_none() -> String {
    match send_message_internal(TaskType::TaskNone) {
        Ok(_) => "Message sent successfully".to_string(),
        Err(e) => format!("Error sending message: {}", e),
    }
}

#[wasm_bindgen]
pub fn receive_message() -> String {
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
    "ASN Web Bus v0.1.0".to_string()
}

// Вспомогательная функция для отправки сообщений
fn send_message_internal(message: TaskType) -> Result<(), String> {
    let bus = new_tokio_bus::<TaskType>(16);
    let sender = bus.get_sender();

    sender
        .send_message(message)
        .map_err(|e| format!("Failed to send message: {:?}", e))
}
