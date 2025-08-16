use std::sync::Arc;

use asn_logger::*;
use asn_winit::WinitWindow;

use crate::{
    StateError,
    data::{LOG_MODULE_NAME, MIN_WINDOW_SIZE},
};

pub struct WgpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_format: wgpu::TextureFormat,
    // pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
}

impl WgpuContext {
    pub async fn new(window: Arc<WinitWindow>) -> Result<Self, StateError> {
        let size = window.inner_size();
        m_trace!("window size: {size:?}");

        if size.width < MIN_WINDOW_SIZE || size.height < MIN_WINDOW_SIZE {
            return Err(StateError::InvalidWindowSize {
                width: size.width,
                height: size.height,
            });
        }

        let backend_features = wgpu::Instance::enabled_backend_features();
        m_trace!("backend_features: {backend_features:?}");

        // Create GPU instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        m_trace!("instance ok");

        // Create surface
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| StateError::SurfaceCreation(e.to_string()))?;

        m_trace!("surface ok");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|_| StateError::NoAdapter)?;

        m_trace!("adapter ok");

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
            .map_err(|e| StateError::DeviceCreation(e.to_string()))?;

        m_trace!("device, queue ok");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);

        m_trace!("surface_caps {:?}", surface_caps);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        m_trace!("surface_format {:?}", surface_format);

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

        m_trace!("config ready");

        let mut w = WgpuContext {
            device,
            queue,
            surface_format,
            // surface,
            config,
        };

        w.resize(size.width, size.height)?;

        Ok(w)
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), StateError> {
        m_trace!("resize {width} {height}");

        // Size validation
        if width < MIN_WINDOW_SIZE || height < MIN_WINDOW_SIZE {
            return Err(StateError::InvalidWindowSize { width, height });
        }

        self.config.width = width;
        self.config.height = height;
        // self.surface.configure(&self.device, &self.config);

        Ok(())
    }
}
