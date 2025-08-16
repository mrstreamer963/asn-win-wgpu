extern crate asn_logger;
extern crate asn_win_wgpu;

mod log_utils;

use std::{sync::Arc, time::Duration};

use asn_logger::log::*;
use asn_wgpu::wgpu;

use asn_winit::{
    WinitWindow,
    winit::{self, application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow},
};
use log_utils::setup_log;

async fn update() {
    info!("update");
}

fn main() {
    setup_log();

    run();

    for i in 0..3 {
        pollster::block_on(update());
        std::thread::sleep(Duration::from_secs(1));
        info!("i: {i}");
    }

    trace!("ok!");
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
    window: Option<Arc<WinitWindow>>,
    s: StateManager,
}

struct StateManager {
    state: Option<State>,
}

impl StateManager {
    fn init(&mut self, w: Arc<WinitWindow>) {
        let s = pollster::block_on(get_state(w.clone()));
        self.state = Some(s);
    }
}

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn get_state(window: Arc<WinitWindow>) -> State {
    let size = window.inner_size();
    trace!("window size: {size:?}");

    let backend_features = wgpu::Instance::enabled_backend_features();
    trace!("backend_features: {backend_features:?}");

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    trace!("instance ok");

    let surface = instance.create_surface(window.clone()).unwrap();

    trace!("surface ok");

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    trace!("adapter ok");

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

    trace!("device, queue ok");

    // Configure surface
    let surface_caps = surface.get_capabilities(&adapter);

    trace!("surface_caps {:?}", surface_caps);

    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);

    trace!("surface_format {:?}", surface_format);

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

    trace!("config ready");

    surface.configure(&device, &config);

    State {
        surface,
        device,
        queue,
    }
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        trace!("resumed!");
        if self.window.is_none() {
            let window_attributes = winit::window::WindowAttributes::default()
                .with_title("No title")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
                .with_resizable(true)
                .with_decorations(true);

            let w = event_loop.create_window(window_attributes).unwrap();
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
        match event {
            WindowEvent::RedrawRequested => {
                if self.window.is_some() {
                    let w = self.window.as_ref().unwrap();
                    w.request_redraw();
                }
            }
            WindowEvent::CloseRequested => {
                trace!("CloseRequested event");
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
