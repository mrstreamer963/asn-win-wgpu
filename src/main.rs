use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use winit::{
    self,
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use wgpu_utils::get_window;

fn main() {
    let event_loop = EventLoop::new()
        .map_err(|e| format!("Failed to create event loop: {e}"))
        .unwrap();

    let mut runner = Runner { s: None };
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut runner).unwrap();

    println!("event_loop.run_app ended");

    std::thread::sleep(Duration::from_secs(3));

    println!("main ok");
}

pub struct State {
    pub window: Arc<Window>,
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
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
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

mod wgpu_utils {
    use std::sync::Arc;

    use winit::{
        event_loop::ActiveEventLoop,
        window::{Window, WindowAttributes},
    };

    use crate::State;

    pub async fn get_state(window: Arc<Window>) -> State {
        let size = window.inner_size();
        println!("window size: {size:?}");

        let backend_features = wgpu::Instance::enabled_backend_features();
        println!("backend_features: {backend_features:?}");

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        println!("instance ok");

        let surface = instance.create_surface(window.clone()).unwrap();

        println!("surface ok");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        println!("adapter ok");

        // Create device and queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("ASN WGPU Device"),
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        println!("device, queue ok");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);

        println!("surface_caps {:?}", surface_caps);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        println!("surface_format {:?}", surface_format);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        println!("config ready");

        surface.configure(&device, &config);

        State {
            surface,
            device,
            queue,
            window,
        }
    }

    pub fn get_window(event_loop: &ActiveEventLoop) -> Window {
        let window_attributes = WindowAttributes::default()
            .with_title("No title")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .with_resizable(true)
            .with_decorations(true);

        let w = event_loop.create_window(window_attributes).unwrap();
        w
    }
}
