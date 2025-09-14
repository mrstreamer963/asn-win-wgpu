// event_bus.rs
use crate::future_bus::event::Event;
use async_channel::{Receiver, Sender, bounded};
use std::collections::HashMap;
use std::sync::Arc;

// Уникальный ID подписчика
type SubscriptionId = usize;

// Глобальный EventBus — один на всё приложение
#[derive(Clone)]
pub struct EventBus {
    sender: Sender<Event>,
    last_events: Arc<std::sync::Mutex<HashMap<String, Event>>>, // опционально: сохраняем последние события
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = bounded(10); // буфер на 10 событий
        let last_events = Arc::new(std::sync::Mutex::new(HashMap::new()));

        Self {
            sender,
            last_events,
        }
    }

    // Отправить событие всем подписчикам
    pub fn emit(&self, event: Event) {
        let event_name = event.name().to_string();
        let event_clone = event.clone();

        // Сохраняем последнее событие по имени (опционально)
        {
            let mut map = self.last_events.lock().unwrap();
            map.insert(event_name.clone(), event_clone);
        }

        // Отправляем всем
        if let Err(_) = self.sender.try_send(event) {
            web_sys::console::warn_1(
                &format!("EventBus: не удалось отправить {}", event_name).into(),
            );
        }
    }

    // Подписаться на все события (или фильтровать по типу позже)
    pub fn subscribe(&self) -> (SubscriptionId, EventReceiver) {
        let receiver = self.sender.subscribe(); // новый Receiver
        let id = uuid::Uuid::new_v4().as_u128() as usize; // простой уникальный ID
        (id, EventReceiver { receiver })
    }

    // Подписаться на конкретный тип события
    pub fn subscribe_to<T>(&self, event_type: T) -> (SubscriptionId, EventReceiver)
    where
        T: Fn(&Event) -> bool,
    {
        let receiver = self.sender.subscribe();
        let id = uuid::Uuid::new_v4().as_u128() as usize;
        (
            id,
            EventReceiver {
                receiver: Box::new(FilteredReceiver {
                    inner: receiver,
                    filter: event_type,
                }),
            },
        )
    }

    // Отписаться
    pub fn unsubscribe(&self, _id: SubscriptionId) {
        // В текущей реализации подписчики умирают автоматически,
        // когда их Receiver уничтожается. Можно добавить ручное удаление,
        // если нужно отслеживать активные подписки.
        // Для простоты — просто игнорируем ID.
    }

    // Получить последнее событие по имени (для новых подписчиков)
    pub fn get_last_event(&self, event_name: &str) -> Option<Event> {
        self.last_events.lock().unwrap().get(event_name).cloned()
    }
}

// Простой Receiver — получает все события
pub struct EventReceiver {
    receiver: async_channel::Receiver<Event>,
}

impl EventReceiver {
    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.next().await
    }
}

// Фильтрующий Receiver — получает только нужные события
struct FilteredReceiver {
    inner: async_channel::Receiver<Event>,
    filter: Box<dyn Fn(&Event) -> bool>,
}

impl FilteredReceiver {
    pub async fn next(&mut self) -> Option<Event> {
        loop {
            match self.inner.next().await {
                Some(event) if (self.filter)(&event) => return Some(event),
                None => return None,
                _ => continue, // пропускаем неподходящие
            }
        }
    }
}
