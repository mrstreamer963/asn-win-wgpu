use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use winit::{self, application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow};

use crate::wgpu_utils::get_window;

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

    // let s = state_manager::get_state_manager();
    let mut runner = Runner { s: None };

    event_loop.set_control_flow(ControlFlow::Poll);
    let result = event_loop.run_app(&mut runner);
    result.unwrap();
}

pub struct State {
    pub window: Arc<winit::window::Window>,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
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
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("resumed!");
        if self.s.is_none() {
            let w = Arc::new(get_window(event_loop));
            println!("Runner get_window ok");

            let state = pollster::block_on(wgpu_utils::get_state(w.clone()));
            println!("Runner get_state ok");

            self.s = Some(Arc::new(Mutex::new(state)));
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
                if let Some(s) = &self.s {
                    s.lock().unwrap().window.request_redraw();
                }
                // println!("redraw request");
            }
            WindowEvent::CloseRequested => {
                println!("CloseRequested event");
                if self.s.is_some() {
                    // без этой штуки закрытое окно не закрывается и зависает
                    // let _ = self.s.take();
                }
                event_loop.exit();
            }
            _ => {
                // trace!("window_event!");
            }
        }
    }
}
