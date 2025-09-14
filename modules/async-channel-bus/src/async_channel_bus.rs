use asn_core_bus::{AsnBus, AsnBusRecvError, AsnBusSendError, AsnReceiver, AsnTransmitter};
use async_channel::{Receiver, Sender};

pub struct AsyncChannelTransmitter<E> {
    tx: Sender<E>,
}

impl<E: Send> AsnTransmitter<E> for AsyncChannelTransmitter<E> {
    fn send_message(&self, m: E) -> Result<(), asn_core_bus::AsnBusSendError> {
        self.tx.try_send(m).map_err(|e| match e {
            async_channel::TrySendError::Full(_) => AsnBusSendError::Full,
            async_channel::TrySendError::Closed(_) => AsnBusSendError::Closed,
        })
    }

    fn send_message_async(
        &self,
        m: E,
    ) -> impl std::future::Future<Output = Result<(), asn_core_bus::AsnBusSendError>> + Send {
        let tx = self.tx.clone();
        async move { tx.send(m).await.map_err(|_| AsnBusSendError::Closed) }
    }
}

pub struct AsyncChannelReceiver<E> {
    rx: Receiver<E>,
}

impl<E: Send> AsnReceiver<E> for AsyncChannelReceiver<E> {
    fn get_message(&mut self) -> Result<E, AsnBusRecvError> {
        self.rx.try_recv().map_err(|e| match e {
            async_channel::TryRecvError::Empty => AsnBusRecvError::Empty,
            async_channel::TryRecvError::Closed => AsnBusRecvError::Closed,
        })
    }

    fn wait_for_message(
        &mut self,
    ) -> impl std::future::Future<Output = Result<E, AsnBusRecvError>> + Send {
        let rx = self.rx.clone();
        async move { rx.recv().await.map_err(|_| AsnBusRecvError::Closed) }
    }
}

pub struct AsyncChannelBus<E> {
    tx: Sender<E>,
    rx: Receiver<E>,
}

impl<E: Send + 'static> AsnBus<E> for AsyncChannelBus<E> {
    fn get_sender(&self) -> impl AsnTransmitter<E> {
        AsyncChannelTransmitter {
            tx: self.tx.clone(),
        }
    }

    fn get_receiver(&self) -> impl AsnReceiver<E> {
        AsyncChannelReceiver {
            rx: self.rx.clone(),
        }
    }
}

impl<E: Send + 'static> AsyncChannelBus<E> {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = async_channel::bounded(capacity);
        AsyncChannelBus { tx, rx }
    }
}

pub fn new_bus<E: Clone + Send + Sync + 'static>(capacity: usize) -> AsyncChannelBus<E> {
    AsyncChannelBus::new(capacity)
}
