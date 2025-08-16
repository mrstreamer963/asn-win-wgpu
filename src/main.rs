use std::{sync::Arc, time::Duration};

use winit::{self, application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow};

use crate::wgpu_utils::{get_state, get_window, State};

mod wgpu_utils;

async fn update() {
    println!("update");
}

fn main() {
    run();

    for i in 0..3 {
        pollster::block_on(update());
        std::thread::sleep(Duration::from_secs(1));
        println!("i: {i}");
    }

    println!("ok!");
}

fn run() {
    let event_loop = winit::event_loop::EventLoop::new()
        .map_err(|e| format!("Failed to create event loop: {e}"))
        .unwrap();

    let s = StateManager { state: None };
    let mut runner = Runner { window: None, s };

    event_loop.set_control_flow(ControlFlow::Poll);
    let result = event_loop.run_app(&mut runner);
    result.unwrap();
}

struct Runner {
    s: StateManager,
    window: Option<Arc<winit::window::Window>>,
}

impl Drop for StateManager {
    fn drop(&mut self) {
        println!("StateManager drop");
        if self.state.is_some() {
            let _ = self.state.take();
        }
    }
}

struct StateManager {
    state: Option<State>,
}

impl StateManager {
    fn init(&mut self, w: Arc<winit::window::Window>) {
        let s = pollster::block_on(get_state(w.clone()));
        self.state = Some(s);
    }
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("resumed!");
        if self.window.is_none() {
            let w = get_window(event_loop);
            let arc_w = Arc::new(w);

            self.s.init(arc_w.clone());

            // let s = pollster::block_on(get_state(arc_w.clone()));
            self.window = Some(arc_w);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let _ = window_id;

        match event {
            WindowEvent::RedrawRequested => {
                if self.window.is_some() {
                    let w = self.window.as_ref().unwrap();
                    w.request_redraw();
                }
            }
            WindowEvent::CloseRequested => {
                println!("CloseRequested event");
                if self.window.is_some() {
                    let _ = self.window.take();
                }
                event_loop.exit();
            }
            _ => {
                // trace!("window_event!");
            }
        }
    }
}
