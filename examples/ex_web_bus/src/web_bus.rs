use std::sync::OnceLock;

use asn_core_bus::AsnBus;
use async_channel_bus::{self, AsyncChannelBus};

#[derive(Clone, Debug)]
pub enum TaskType {
    TaskUpdate,
    TaskNone,
}

// Глобальный EventBus — инициализируется один раз
static EVENT_BUS: OnceLock<AsyncChannelBus<TaskType>> = OnceLock::new();

pub fn get_event_bus() -> &'static impl AsnBus<TaskType> {
    EVENT_BUS.get_or_init(|| async_channel_bus::new_bus(16))
}
