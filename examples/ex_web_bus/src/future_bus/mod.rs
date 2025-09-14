mod event;
mod event_bus;

use std::sync::OnceLock;
use wasm_bindgen::prelude::*;

use crate::future_bus::{event::Event, event_bus::EventBus};

// Глобальный EventBus — инициализируется один раз
static EVENT_BUS: OnceLock<EventBus> = OnceLock::new();

pub fn get_event_bus() -> &'static EventBus {
    EVENT_BUS.get_or_init(|| EventBus::new())
}

// Публичные функции для JS
#[wasm_bindgen]
pub fn init_event_bus() {
    // Уже инициализировано через static
}

#[wasm_bindgen]
pub fn emit_event(json: &str) -> Result<(), JsValue> {
    let event: Event = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&format!("Ошибка десериализации: {}", e)))?;
    get_event_bus().emit(event);
    Ok(())
}

#[wasm_bindgen]
pub fn subscribe_to_events(callback: js_sys::Function) -> JsValue {
    let (id, mut rx) = get_event_bus().subscribe();

    // Запускаем фоновый таск, который слушает события
    wasm_bindgen_futures::spawn_local(async move {
        while let Some(event) = rx.next().await {
            let js_value = event.to_js_value();
            let _ = callback.call1(&JsValue::UNDEFINED, &js_value);
        }
    });

    // Возвращаем ID подписки — можно использовать для отписки
    JsValue::from_serde(&id).unwrap()
}

#[wasm_bindgen]
pub fn subscribe_to_event_type(event_type: &str, callback: js_sys::Function) -> JsValue {
    let (id, mut rx) = get_event_bus().subscribe_to(|e| e.name() == event_type);

    wasm_bindgen_futures::spawn_local(async move {
        while let Some(event) = rx.next().await {
            let js_value = event.to_js_value();
            let _ = callback.call1(&JsValue::UNDEFINED, &js_value);
        }
    });

    JsValue::from_serde(&id).unwrap()
}

#[wasm_bindgen]
pub fn get_last_event(event_name: &str) -> JsValue {
    if let Some(event) = get_event_bus().get_last_event(event_name) {
        event.to_js_value()
    } else {
        JsValue::NULL
    }
}
