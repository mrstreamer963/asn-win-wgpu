use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use winit::{
    self,
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

fn main() {
    let event_loop = EventLoop::new()
        .map_err(|e| format!("Failed to create event loop: {e}"))
        .unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut runner = Runner { s: None };
    event_loop.run_app(&mut runner).unwrap();

    println!("event_loop.run_app ended");

    std::thread::sleep(Duration::from_secs(3));

    println!("main ok");
}

pub struct State {
    pub window: Arc<Window>,
}

struct Runner {
    s: Option<Arc<Mutex<State>>>,
}

impl Drop for Runner {
    fn drop(&mut self) {
        println!("Runner drop()");
    }
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("resumed!");
        if self.s.is_none() {
            let window_attrs = WindowAttributes::default();
            let window = event_loop.create_window(window_attrs).unwrap();

            let w = Arc::new(window);
            println!("Runner get_window ok");

            let state = State { window: w.clone() };
            println!("Runner get_state ok");

            self.s = Some(Arc::new(Mutex::new(state)));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let _ = window_id;

        match event {
            WindowEvent::RedrawRequested => {
                if let Some(s) = &self.s {
                    // println!("redraw request");
                    s.lock().unwrap().window.request_redraw();
                }
            }
            WindowEvent::CloseRequested => {
                println!("CloseRequested event");
                if self.s.is_some() {
                    // без этой штуки закрытое окно не закрывается и зависает
                    // let _ = self.s.take();
                }
                event_loop.exit();
            }
            WindowEvent::Destroyed => {
                println!("Destroyed event");
            }
            _ => {
                println!("window_event! {:?}", event);
            }
        }
    }
}
