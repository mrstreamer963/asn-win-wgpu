//! Tokio-based implementation of the ASN event bus.
//!
//! This module provides an implementation of the `AsnBus` trait using Tokio's broadcast channels.
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
//! # Error Handling
//!
//! The module handles various error conditions:
//! - `AsnBusSendError::Closed`: When trying to send a message to a closed channel
//! - `AsnBusRecvError::Empty`: When trying to receive a message from an empty channel
//! - `AsnBusRecvError::Closed`: When trying to receive a message from a closed channel
//! - `AsnBusRecvError::Lagged(n)`: When the receiver lagged too far behind and missed `n` messages
//!
//! Additionally, the module provides asynchronous methods:
//! - `send_message_async`: Sends a message asynchronously, waiting if the channel is full
//! - `wait_for_message`: Waits asynchronously for a message to be available

use asn_core_bus::{AsnBus, AsnBusRecvError, AsnTransmitter};
use asn_core_bus::{AsnBusSendError, AsnReceiver};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

/// Transmitter implementation for Tokio broadcast channels.
///
/// This struct wraps a Tokio `Sender` and implements the `AsnTransmitter` trait.
/// It provides both synchronous and asynchronous methods for sending messages.
struct TokioTransmitter<E> {
    tx: Sender<E>,
}

/// Receiver implementation for Tokio broadcast channels.
///
/// This struct wraps a Tokio `Receiver` and implements the `AsnReceiver` trait.
/// It provides both synchronous and asynchronous methods for receiving messages.
struct TokioReceiver<E> {
    rx: Receiver<E>,
}

impl<E> AsnTransmitter<E> for TokioTransmitter<E>
where
    E: Send,
{
    /// Sends a message through the broadcast channel.
    ///
    /// # Arguments
    ///
    /// * `m` - The message to send
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(AsnBusSendError::Closed)` if the channel is closed
    fn send_message(&self, m: E) -> Result<(), AsnBusSendError> {
        let result = self.tx.send(m);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(AsnBusSendError::Closed),
        }
    }

    /// Sends a message asynchronously, waiting if the channel is full.
    ///
    /// This method will wait until there is space in the channel to send the message.
    ///
    /// # Arguments
    ///
    /// * `m` - The message to send
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(AsnBusSendError)` if there was an error sending the message
    fn send_message_async(
        &self,
        m: E,
    ) -> impl std::future::Future<Output = Result<(), AsnBusSendError>> + Send {
        let tx = self.tx.clone();
        async move {
            let result = tx.send(m);
            match result {
                Ok(_) => Ok(()),
                Err(_) => Err(AsnBusSendError::Closed),
            }
        }
    }
}

impl<E> AsnReceiver<E> for TokioReceiver<E>
where
    E: Clone + Send,
{
    /// Attempts to receive a message from the broadcast channel.
    ///
    /// This method is non-blocking and will return immediately.
    ///
    /// # Returns
    ///
    /// * `Ok(E)` with the received message
    /// * `Err(AsnBusRecvError::Empty)` if no messages are available
    /// * `Err(AsnBusRecvError::Closed)` if the channel is closed
    /// * `Err(AsnBusRecvError::Lagged(n))` if the receiver lagged too far behind and missed `n` messages
    fn get_message(&mut self) -> Result<E, AsnBusRecvError> {
        let result = self.rx.try_recv();
        match result {
            Ok(e) => Ok(e),
            Err(err) => match err {
                broadcast::error::TryRecvError::Empty => Err(AsnBusRecvError::Empty),
                broadcast::error::TryRecvError::Closed => Err(AsnBusRecvError::Closed),
                broadcast::error::TryRecvError::Lagged(n) => Err(AsnBusRecvError::Lagged(n)),
            },
        }
    }

    /// Waits asynchronously for a message to be available.
    ///
    /// This method will wait until a message is available in the channel.
    ///
    /// # Returns
    ///
    /// * `Ok(E)` with the received message
    /// * `Err(AsnBusRecvError)` if there was an error receiving the message
    fn wait_for_message(
        &mut self,
    ) -> impl std::future::Future<Output = Result<E, AsnBusRecvError>> + Send {
        let fut = self.rx.recv();
        async move {
            let result = fut.await;
            match result {
                Ok(e) => Ok(e),
                Err(_) => Err(AsnBusRecvError::Closed),
            }
        }
    }
}

/// Tokio-based implementation of the `AsnBus` trait.
///
/// This struct uses Tokio's broadcast channels to implement the event bus pattern.
/// It allows multiple receivers to receive the same messages.
pub struct TokioEventBus<E> {
    tx: Sender<E>,
}

impl<E> TokioEventBus<E>
where
    E: Clone,
{
    /// Creates a new `TokioEventBus` with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The buffer size of the broadcast channel
    ///
    /// # Returns
    ///
    /// A new `TokioEventBus` instance
    fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel::<E>(capacity);
        TokioEventBus { tx }
    }
}

impl<E> AsnBus<E> for TokioEventBus<E>
where
    E: Clone + Send + Sync,
{
    /// Gets a transmitter for sending messages.
    ///
    /// # Returns
    ///
    /// An implementation of `AsnTransmitter` for sending messages
    fn get_sender(&self) -> impl AsnTransmitter<E> {
        TokioTransmitter {
            tx: self.tx.clone(),
        }
    }

    /// Gets a receiver for receiving messages.
    ///
    /// # Returns
    ///
    /// An implementation of `AsnReceiver` for receiving messages
    fn get_receiver(&self) -> impl AsnReceiver<E> {
        TokioReceiver {
            rx: self.tx.subscribe(),
        }
    }
}

/// Creates a new Tokio-based event bus with the specified capacity.
///
/// # Arguments
///
/// * `capacity` - The buffer size of the broadcast channel
///
/// # Returns
///
/// An implementation of `AsnBus` using Tokio's broadcast channels
///
/// # Examples
///
/// ```rust
/// use tokio_bus::new_tokio_bus;
/// use asn_core_bus::{AsnBus, AsnTransmitter, AsnReceiver};
///
/// #[derive(Clone, Debug)]
/// enum Message {
///     Update(String),
///     Shutdown,
/// }
///
/// let bus = new_tokio_bus::<Message>(16);
/// ```
pub fn new_tokio_bus<E: Clone + Send + Sync>(capacity: usize) -> TokioEventBus<E> {
    TokioEventBus::new(capacity)
}
