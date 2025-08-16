use std::{sync::Arc, time::Duration};

use winit::{self, application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow};

mod state_manager;
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

    let s = state_manager::get_state_manager();
    let mut runner = Runner { window: None, s };

    event_loop.set_control_flow(ControlFlow::Poll);
    let result = event_loop.run_app(&mut runner);
    result.unwrap();
}

struct Runner {
    s: state_manager::StateManager,
    window: Option<Arc<winit::window::Window>>,
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("resumed!");
        if self.window.is_none() {
            let w = Arc::new({
                let window_attributes = winit::window::WindowAttributes::default()
                    .with_title("No title")
                    .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
                    .with_resizable(true)
                    .with_decorations(true);

                let w = event_loop.create_window(window_attributes).unwrap();
                w
            });

            let state = pollster::block_on(wgpu_utils::get_state(w.clone()));

            print!("Runner get_state ok");
            self.s.set_state(state);
            self.window = Some(w.clone());
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
                    println!("redraw request");
                    w.request_redraw();
                    println!("redraw request Ok");
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
