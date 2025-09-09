use asn_logger::log::info;
mod gui;
mod gui_handler;
mod log_utils;
mod map;

pub const LOG_MODULE_NAME: &str = "ex_web";

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

use crate::gui::run_gui;

#[wasm_bindgen]
pub fn init_web_app() -> Result<(), JsValue> {
    if let Err(e) = log_utils::setup_log() {
        return Err(e.into());
    }

    // Простая инициализация для web
    info!("ASN Web App initialized");

    // Здесь можно добавить базовую логику без проблемных зависимостей
    spawn_local(async move {
        console::log_1(&"Async task started".into());

        // Простая асинхронная задача без задержки
        console::log_1(&"Async task completed".into());
    });

    run_gui();

    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ASN Web!", name)
}

#[wasm_bindgen]
pub fn get_version() -> String {
    "ASN Web WGPU v0.1.0".to_string()
}

#[wasm_bindgen]
pub fn get_system_info() -> String {
    let mut info = HashMap::new();
    info.insert("version", "0.1.0");
    info.insert("platform", "web");
    info.insert("wasm", "enabled");
    info.insert("webgl", "supported");

    format!("System Info: {:?}", info)
}

#[wasm_bindgen]
pub fn calculate_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut a = 0u64;
    let mut b = 1u64;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

#[wasm_bindgen]
pub fn get_memory_usage() -> String {
    // Получаем информацию о памяти из WebAssembly
    let memory = wasm_bindgen::memory();
    let js_memory: js_sys::WebAssembly::Memory = memory.into();
    let buffer = js_memory.buffer();

    // Получаем размер буфера в байтах
    let memory_size = js_sys::Reflect::get(&buffer, &"byteLength".into())
        .map(|v| v.as_f64().unwrap_or(0.0) as u32)
        .unwrap_or(0);

    // Конвертируем в килобайты
    let memory_kb = memory_size / 1024;

    format!("Memory usage: {} KB", memory_kb)
}

#[wasm_bindgen]
pub fn test_async_operation() -> js_sys::Promise {
    let future = async {
        // Имитация асинхронной операции
        console::log_1(&"Starting async operation...".into());

        // Простая задержка через setTimeout
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let closure = Closure::wrap(Box::new(move || {
                console::log_1(&"Async operation completed!".into());
                resolve.call0(&JsValue::undefined()).unwrap();
            }) as Box<dyn FnMut()>);

            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    1000,
                )
                .unwrap();

            closure.forget();
        });

        // Конвертируем Promise в Future
        wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
        Ok(JsValue::undefined())
    };

    wasm_bindgen_futures::future_to_promise(future)
}
