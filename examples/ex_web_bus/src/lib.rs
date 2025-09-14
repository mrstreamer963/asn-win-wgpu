use asn_core_bus::{AsnBus, AsnReceiver, AsnTransmitter};
use asn_logger::*;
use wasm_bindgen::prelude::*;

mod setup_log;
mod web_bus;

use setup_log::setup_log;
use web_bus::{TaskType, get_event_bus};

const LOG_MODULE_NAME: &str = "ex_web_bus";

#[wasm_bindgen]
pub fn init_web_app() -> Result<(), JsValue> {
    setup_log().unwrap();

    m_info!("Hello from init_web_app");

    let bus = get_event_bus();
    let sender = bus.get_sender();
    let mut receiver = bus.get_receiver();

    sender.send_message(TaskType::TaskUpdate).unwrap();
    sender.send_message(TaskType::TaskNone).unwrap();

    let mess = receiver.get_message().unwrap();
    m_info!("Mess: {:?}", mess);

    let mess = receiver.get_message().unwrap();
    m_info!("Mess: {:?}", mess);

    // Простая инициализация для web
    Ok(())
}

#[wasm_bindgen]
pub fn send_task_update() -> String {
    m_info!("send_task_update");

    let s = get_event_bus().get_sender();

    match s.send_message(TaskType::TaskUpdate) {
        Ok(_) => "Message sent successfully".to_string(),
        Err(e) => format!("Error sending message: {:?}", e),
    }
}

#[wasm_bindgen]
pub fn send_task_none() -> String {
    m_info!("send_task_none");

    let s = get_event_bus().get_sender();

    match s.send_message(TaskType::TaskNone) {
        Ok(_) => "Message sent successfully".to_string(),
        Err(e) => format!("Error sending message: {:?}", e),
    }
}

#[wasm_bindgen]
pub fn receive_message() -> String {
    m_info!("receive_message");

    let bus = get_event_bus();
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
