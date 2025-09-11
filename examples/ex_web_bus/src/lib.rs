use asn_core_bus::{AsnBus, AsnReceiver, AsnTransmitter};
use tokio_bus::new_tokio_bus;

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum TaskType {
    TaskNone,
    TaskUpdate,
}

pub fn run_lib() {
    let bus = new_tokio_bus::<TaskType>(16);

    let mut recv = bus.get_receiver();
    let send = bus.get_sender();

    send.send_message(TaskType::TaskUpdate).unwrap();
    send.send_message(TaskType::TaskNone).unwrap();

    let result1 = recv.get_message().unwrap();
    let result2 = recv.get_message().unwrap();

    println!("Result: {:?}", result1);
    println!("Result: {:?}", result2);
}
