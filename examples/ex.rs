// https://gist.github.com/mrstreamer963/85ab238fd42b7a2077a22d179e1d1c54#file-main-rs-L82

use std::sync::Arc;
use std::time::Duration;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Debug)]
struct DemoResource;

enum DemoApp {
    Loading,
    Ready {
        window: Arc<Window>,
        resource: DemoResource,
    },
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = DemoApp::Loading;

    event_loop.run_app(&mut app).unwrap();

    println!("event_loop.run_app ended");

    std::thread::sleep(Duration::from_secs(3));

    println!("main ok");
}

impl ApplicationHandler for DemoApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Self::Loading = self {
            let window_attrs = WindowAttributes::default();

            let window = event_loop.create_window(window_attrs).unwrap();

            let resource = DemoResource;

            *self = Self::Ready {
                window: Arc::new(window),
                resource,
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Self::Ready { window, resource } = self else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                println!("Window title: {}", window.title());
                println!("Demo resource: {:?}", resource);
            }
            WindowEvent::CloseRequested => {
                *self = Self::Loading;
                event_loop.exit()
            }
            _ => {}
        }
    }
}
