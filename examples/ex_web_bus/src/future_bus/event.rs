// events.rs

use serde::{Deserialize, Serialize}; // для сериализации в JS
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Event {
    UserLogin { username: String },
    UserLogout { user_id: u64 },
    DataUpdated { data: String },
    ErrorOccurred { message: String },
    CustomMessage { payload: String },
}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Event::UserLogin { .. } => "UserLogin",
            Event::UserLogout { .. } => "UserLogout",
            Event::DataUpdated { .. } => "DataUpdated",
            Event::ErrorOccurred { .. } => "ErrorOccurred",
            Event::CustomMessage { .. } => "CustomMessage",
        }
    }

    pub fn to_js_value(&self) -> JsValue {
        // Сериализуем в JSON и передаём в JS
        let json = serde_json::to_string(self).unwrap_or_default();
        JsValue::from_str(&json)
    }
}
