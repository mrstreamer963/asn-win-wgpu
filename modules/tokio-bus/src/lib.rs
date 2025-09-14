//! Tokio-based implementation of the ASN event bus.
//!
//! This crate provides an implementation of the `AsnBus` trait using Tokio's broadcast channels.
//! It allows for asynchronous message passing between different parts of an application.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```rust
//! use tokio_bus::new_tokio_bus;
//! use asn_core_bus::{AsnBus, AsnTransmitter, AsnReceiver};
//!
//! #[derive(Clone, Debug)]
//! enum Message {
//!     Update(String),
//!     Shutdown,
//! }
//!
//! let bus = new_tokio_bus::<Message>(16);
//! let sender = bus.get_sender();
//! let mut receiver = bus.get_receiver();
//!
//! sender.send_message(Message::Update("Hello".to_string())).unwrap();
//! let msg = receiver.get_message().unwrap();
//! ```
//!
//! For more detailed information about the API, see the [new_tokio_bus] function and the
//! [tokio_event_bus] module documentation.

extern crate asn_core;
extern crate asn_core_bus;

mod tokio_event_bus;

pub use tokio_event_bus::TokioEventBus;
pub use tokio_event_bus::new_tokio_bus;
