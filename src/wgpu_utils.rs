use std::sync::Arc;

use crate::State;

pub async fn get_state(window: Arc<winit::window::Window>) -> State {
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

pub fn get_window(event_loop: &winit::event_loop::ActiveEventLoop) -> winit::window::Window {
    let window_attributes = winit::window::WindowAttributes::default()
        .with_title("No title")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .with_resizable(true)
        .with_decorations(true);

    let w = event_loop.create_window(window_attributes).unwrap();
    w
}
