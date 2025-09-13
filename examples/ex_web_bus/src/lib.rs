use asn_core_bus::{AsnBus, AsnReceiver, AsnTransmitter};
use asn_logger::init_log;
use asn_logger::*;
use std::sync::OnceLock;
use tokio_bus::{TokioEventBus, new_tokio_bus};
use wasm_bindgen::prelude::*;

const LOG_MODULE_NAME: &str = "ex_web_bus";

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum TaskType {
    TaskNone,
    TaskUpdate,
}

// Глобальная переменная для хранения шины данных
static BUS: OnceLock<TokioEventBus<TaskType>> = OnceLock::new();

// Функция для инициализации шины данных
fn init_bus() {
    BUS.get_or_init(|| new_tokio_bus::<TaskType>(16));
}

// Функция для получения ссылки на шину данных
fn get_bus() -> &'static TokioEventBus<TaskType> {
    BUS.get().expect("Bus should be initialized")
}

#[wasm_bindgen]
pub fn init_web_app() -> Result<(), JsValue> {
    setup_log().unwrap();

    m_info!("Hello from init_web_app");

    // Инициализируем шину данных
    init_bus();

    let bus = get_bus();
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

    // Инициализируем шину данных, если она еще не инициализирована
    init_bus();

    let bus = get_bus();
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

    // Инициализируем шину данных, если она еще не инициализирована
    init_bus();

    let bus = get_bus();
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
